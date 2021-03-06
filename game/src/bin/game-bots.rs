use game::bots;
use std::process::exit;

fn main() {
	match bots::main_loop() {
		Ok(()) => (),
		Err(e) => {
			eprintln!("{}", e);
			exit(1);
		}
	}
}
