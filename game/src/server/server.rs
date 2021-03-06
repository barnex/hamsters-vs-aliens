use super::internal::*;
use Message::*;

pub struct Server {
	clients: HashMap<usize, NetSender>,
	events: Receiver<ServerEvent>,

	_clients_send: Sender<ServerEvent>,

	next_player_id: ID,
	game_state: GameState,
	map_file: PathBuf,
	autosave: bool,
}

enum ServerEvent {
	Conn(NetPipe),
	Drop(ID),
	ClientMessage((ID, Message)),
}

impl Server {
	/// Listen for connections on `address`,
	/// serve a game with map loaded from `map_file`.
	/// Save map edits on client disconnect if `autosave` == true.
	///
	/// This function does not return unless there's an error.
	pub fn serve(address: &str, map_file: PathBuf, autosave: bool) -> Result<()> {
		let map = Map::load(&map_file)?;
		let players = Players::new();
		let game_state = GameState::new(map, players);

		let (clients_send, server_recv) = channel();
		Self::spawn_listen_loop(address, clients_send.clone())?;

		let mut server = Self {
			clients: HashMap::default(),
			events: server_recv,
			_clients_send: clients_send,
			next_player_id: 0,
			game_state,
			map_file,
			autosave,
		};

		server.serve_loop()
	}

	// Run the "manager task", who exclusively controls the shared state
	// (game state + client connections) via message passing.
	fn serve_loop(&mut self) -> Result<()> {
		use ServerEvent::*;
		loop {
			match self.events.recv()? {
				Conn(netpipe) => self.handle_conn(netpipe),
				Drop(id) => Ok(self.drop_client(id)),
				ClientMessage((id, msg)) => self.handle_client_msg(id, msg),
			}?;
		}
	}

	// Handle a connection event:
	// add new player to the game, send them the full state.
	fn handle_conn(&mut self, mut netpipe: NetPipe) -> Result<()> {
		// First incoming message should be `Join`,
		// sending us player settings.
		let skin = match netpipe.recv()? {
			Join { skin } => skin,
			bad => return err(format!("server: handle_conn: expected Join, got {}", &bad)),
		};

		// Add new player to game
		let mut player = Player::new(skin);
		player.model.pos = (self.game_state.map().size() / 2).map(|v| v as f32);
		let player_id = self.next_player_id;
		self.next_player_id += 1;
		self.game_state.update_player(player_id, player);

		// Add new client to clients list
		let (send, recv) = netpipe.split();
		assert!(!self.clients.contains_key(&player_id));
		self.clients.insert(player_id, send);
		Self::start_pipe(player_id, recv, self._clients_send.clone());

		// Respond with full map, player list, new player's ID.
		self.send(
			player_id,
			Accepted {
				map_data: self.game_state.map().to_bytes(),
				players: self.game_state.players().clone(),
				player_id,
			},
		);

		Ok(())
	}

	// Handle a dropped connection event:
	// remove the player from the players list,
	// and broadcast this to all remaining clients.
	//
	// Also save the map if `autosave` == true.
	fn drop_client(&mut self, player_id: ID) {
		println!("dropping player {}", player_id);
		self.clients.remove(&player_id);
		self.game_state.drop_player(player_id);
		self.broadcast(DropPlayer { player_id });

		self.trigger_autosave();
	}

	fn trigger_autosave(&self) {
		if !self.autosave {
			return;
		}
		println!("autosaving {}", &self.map_file.to_string_lossy());
		if let Err(e) = self.game_state.map().save(&self.map_file) {
			// There is not much the server can do when an autosave fails.
			// Aborting would end the game and prevent an future autosave attempt.
			eprintln!("[!] save error: {}", e)
		}
	}

	/// Handle an incoming game state mutation from one of the connected clients.
	fn handle_client_msg(&mut self, client_id: ID, msg: Message) -> Result<()> {
		use Message::*;
		match msg {
			UpdateMap { index, voxel } => Ok(self.update_map(index, voxel)),
			UpdatePlayer { player_id, player } => Ok(self.update_player(client_id, player_id, player)),
			AddEffect(e) => Ok(self.broadcast(AddEffect(e))),
			bad => err(format!("server: handle_msg: not allowed: {}", &bad)),
		}
	}

	/// Handle a map mutation message:
	///   * Mutate the server's map.
	///   * Broadcast the mutation to all clients.
	fn update_map(&mut self, index: ivec3, voxel: Voxel) {
		self.game_state.update_map(index, voxel);
		self.broadcast(UpdateMap { index, voxel });
	}

	/// Handle a player state mutation (e.g. moving, looking around):
	///   * Broadcast to all players.
	///
	/// Note: the player who sent this update will ignore the broadcast
	/// (has already updated their own position).
	fn update_player(&mut self, client_id: ID, player_id: ID, player: Player) {
		if client_id != player_id {
			// clients can't move other players but themselves.
			println!("client {} attempted to update player {}, dropping", client_id, player_id);
			self.drop_client(client_id);
			return;
		}

		self.game_state.update_player(player_id, player.clone());
		self.broadcast(UpdatePlayer { player_id, player });
	}

	/// Send a game state mutation to one client.
	fn send(&mut self, client_id: ID, msg: Message) {
		self.clients.get_mut(&client_id).unwrap().send(msg)
	}

	/// Send a message to all connected clients.
	fn broadcast(&mut self, msg: Message) {
		for (_client_id, netpipe) in &mut self.clients {
			netpipe.send(msg.clone())
		}
	}

	/// Spawn a loop that continuously reads incoming messages from one client,
	/// sends the server a `ServerEvent::ClientMessage` for each incoming message.
	fn start_pipe(player_id: ID, mut recv: NetReceiver, send: Sender<ServerEvent>) {
		std::thread::spawn(move || loop {
			match recv.recv() {
				Err(_e) => {
					//eprintln!("server: pipe messages from client {}: {}", player_id, e);
					send.send(ServerEvent::Drop(player_id)).unwrap();
					return;
				}
				Ok(msg) => match send.send(ServerEvent::ClientMessage((player_id, msg))) {
					Ok(()) => (),
					Err(_e) => {
						//eprintln!("server: pipe messages from client {}: {}", player_id, e);
						send.send(ServerEvent::Drop(player_id)).unwrap();
						return;
					}
				},
			}
		});
	}

	// Spawn a loop that accepts incoming connections,
	// sends the server a `ServerEvent::Conn` event for each accepted connection.
	fn spawn_listen_loop(address: &str, clients_send: Sender<ServerEvent>) -> Result<()> {
		let listener = TcpListener::bind(address)?;
		println!("listening on {}", listener.local_addr().unwrap());
		std::thread::spawn(move || {
			for stream in listener.incoming() {
				match stream {
					Err(e) => eprintln!("{}", e), // client failed to connect, server carries on.
					Ok(tcp_stream) => {
						println!("connected to {}", tcp_stream.peer_addr().unwrap());
						let pipe = NetPipe::new(tcp_stream);
						if clients_send.send(ServerEvent::Conn(pipe)).is_err() {
							return; // server quit, so stop worker thread.
						}
					}
				}
			}
		});
		Ok(())
	}
}
