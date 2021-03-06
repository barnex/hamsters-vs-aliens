use super::internal::*;
use Message::*;

/// A client provides remote access to a GameState,
/// with a locally cached view.
///
/// Clients and server each have their local GameState copy,
/// which they continuously synchronize.
///
/// Clients see an up-to-date version of their own player's data,
/// and a potentially outdated but eventually consistent view of the rest of the data
/// (Map, other Players, Effects, ...).
///
/// The server stores the authoritative version of the Map data
/// (the voxels that make up the world, etc).
///
/// Clients can send GameState mutation requests.
/// These are ordered by the server,
/// applied to the server's version of the GameState,
/// and broadcast back to all clients so that they can update their local view.
pub struct Client {
	netpipe: NetPipe,      // bi-directional server connection
	player_id: usize,      // our player ID on the server, the one we control
	game_state: GameState, // local copy of the server's game state, continually catching up with server
}

impl Client {
	/// Connect to server and join game.
	/// TODO: pass PlayerOptions{skin, name, ..}
	pub fn connect(server_addr: &str, player_skin: usize) -> Result<Self> {
		// connect
		let mut netpipe = NetPipe::new(TcpStream::connect(server_addr)?);

		// join
		netpipe.send(Message::Join { skin: player_skin });

		// receive state
		let (game_state, player_id) = match netpipe.recv() {
			Ok(Accepted { player_id, map_data, players }) => (GameState::new(Map::from_bytes(&map_data)?, players), player_id),
			Err(e) => return Err(e),
			Ok(bad_msg) => return err(format!("client: connect: got bad reply: {}", &bad_msg)),
		};

		Ok(Self { netpipe, player_id, game_state })
	}

	/// Advance our local time by `dt` seconds:
	///	  - Catch up with server state
	///   - Extrapolate other player's positions (to avoid jitter)
	///   - Tick effects (advanced locally, as they are fast and ephemeral).
	///
	/// Does not affect our own player (see `update_player`).
	pub fn tick(&mut self, dt: f32) {
		self.receive_updates();
		self.game_state.time += dt;
		for (&id, player) in self.game_state.players.iter_mut() {
			if id != self.player_id {
				player.model.extrapolate(dt)
			}
		}
		self.game_state.effects.tick(dt);
	}

	/// Update our local player state,
	/// and send it back to the server.
	pub fn update_player(&mut self, player: Player) {
		self.game_state.players.set(self.player_id, player.clone());
		self.send(Message::UpdatePlayer { player_id: self.player_id, player });
	}

	/// Send GameState updates to the server.
	/// These will be broadcast back to all clients (incl. ourself).
	pub fn send_updates(&mut self, updates: Updates) {
		for msg in updates {
			self.send(msg)
		}
	}

	fn send(&mut self, msg: Message) {
		self.netpipe.send(msg)
	}

	// apply state updates from server.
	// TODO: return Result<()> !!
	fn receive_updates(&mut self) {
		for msg in self.netpipe.try_iter() {
			use Message::*;
			match msg {
				UpdateMap { index, voxel } => self.game_state.update_map(index, voxel),
				UpdatePlayer { player_id, player } => {
					// Ignore updates to the local player, who is updated client-side for smoother movement.
					if player_id != self.player_id {
						self.game_state.update_player(player_id, player)
					}
				}
				DropPlayer { player_id } => self.game_state.drop_player(player_id),
				AddEffect(e) => self.game_state.effects.push(e),
				_ => panic!("unsupported message: {}", &msg),
			}
		}
	}

	/// Iterates over the other players.
	pub fn other_players(&self) -> impl Iterator<Item = (&usize, &Player)> {
		let player_id = self.player_id;
		self.game_state.players.iter().filter(move |(&i, _)| i != player_id)
	}

	/// The local player, controlled by this client.
	pub fn player(&self) -> &Player {
		self.game_state.players.get(self.player_id)
	}

	/// Get the client's current view of the game state.
	/// May lag behind the server view, except for the local Player state.
	pub fn game_state(&self) -> &GameState {
		&self.game_state
	}

	/// Draw our local GameState, seen from our player.
	pub fn draw(&self, ctx: &GLContext) {
		self.game_state.draw(self.player_id, ctx)
	}
}
