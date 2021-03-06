use super::internal::*;

pub struct NetPipe {
	pub send: NetSender,
	pub recv: NetReceiver,
}

impl NetPipe {
	pub fn new(tcp_stream: TcpStream) -> Self {
		let (send, recv) = netpipe(tcp_stream);
		Self { send, recv }
	}

	pub fn split(self) -> (NetSender, NetReceiver) {
		(self.send, self.recv)
	}

	pub fn send(&mut self, msg: Message) {
		self.send.send(msg)
	}

	pub fn recv(&mut self) -> Result<Message> {
		self.recv.recv()
	}

	pub fn try_iter(&mut self) -> TryIter<Message> {
		self.recv.try_iter()
	}
}

pub struct NetSender(Sender<Message>);

pub struct NetReceiver(Receiver<Message>);

pub fn netpipe(tcp_stream: TcpStream) -> (NetSender, NetReceiver) {
	tcp_stream.set_nodelay(true).expect("set TCP no delay");
	let tcp_stream2 = tcp_stream.try_clone().expect("clone TCP stream");

	let (send, worker_recv) = channel();
	let (worker_send, recv) = channel();

	start_upload(tcp_stream, worker_recv);
	start_download(tcp_stream2, worker_send);

	(NetSender(send), NetReceiver(recv))
}

impl NetSender {
	/// Attempts to send a Message.
	///
	/// If an error occurs, the pipe shuts down and will return an error upon
	/// one of the next `recv` calls.
	// (Because even a successful send would not guarantee that the server successfully
	// processed the message, it may have crashed right after receiving.
	// Only a server response guarantees success.)
	pub fn send(&mut self, msg: Message) {
		if let Err(e) = self.0.send(msg) {
			eprintln!("netpipe: send: {}", e)
		}
	}
}

impl NetReceiver {
	/// Receive a message.
	///
	/// Returns an error when either the receive itself, or a previous send failed.
	/// I.e. send errors are deferred to being handled at receive time.
	pub fn recv(&mut self) -> Result<Message> {
		Ok(self.0.recv()?)
	}

	pub fn try_iter(&mut self) -> TryIter<Message> {
		self.0.try_iter()
	}
}

// Spawn a loop taking messages from `worker_recv` and serializing them to `tcp_stream`.
//
// The loop aborts on error, causing the next `NetPipe::recv` call to error out.
// I.e.: errors are to be handled on receive, not send.
// (Because even a successful send would not guarantee that the server successfully
// processed the message, it may have crashed right after receiving.
// Only a server response guarantees success.)
fn start_upload(tcp_stream: TcpStream, worker_recv: Receiver<Message>) {
	let mut buf = BufWriter::new(tcp_stream);

	spawn(move || {
		if let Err(e) = || -> Result<()> {
			// wrapper so we can use ? operator
			loop {
				// wait for the first message,
				// then consume all further pending messages, if any,
				// flush them all together.
				let msg = worker_recv.recv()?;
				msg.serialize(&mut buf)?;
				for msg in worker_recv.try_iter() {
					msg.serialize(&mut buf)?;
				}
				buf.flush()?;
			}
		}() {
			eprintln!("netpipe: upload: {}", e);
			return;
		}
	});
}

// Spawn a loop deserializing messages from `tcp_stream` and sending them to `worker_send`.
// Drops `worker_send` on error, causing future `recv` calls to error out.
fn start_download(tcp_stream: TcpStream, worker_send: Sender<Message>) {
	let mut buf = BufReader::new(tcp_stream);

	spawn(move || {
		if let Err(_e) = || -> Result<()> {
			// wrapper so we can use ? operator
			loop {
				let msg = Message::deserialize(&mut buf)?;
				worker_send.send(msg)?;
			}
		}() {
			//eprintln!("netpipe: download: {}", e);
			return;
		}
	});
}
