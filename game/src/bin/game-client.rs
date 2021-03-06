use game::glutin_frontend;
use std::process::exit;

fn main() {
	match glutin_frontend::main_loop() {
		Ok(()) => (),
		Err(e) => {
			eprintln!("{}", e);
			exit(1);
		}
	}
}
