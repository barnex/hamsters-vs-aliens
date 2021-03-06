use super::internal::*;

use structopt::StructOpt;

#[derive(StructOpt)]
struct Args {
	/// Server address
	#[structopt(long, default_value = "localhost:3344")]
	pub server: String,
	// num bots etc.
}

pub fn main_loop() -> Result<()> {
	let args = Args::from_args();

	let skin = 2; // TODO
	let client = Client::connect(&args.server, skin)?;
	let mut bot = Bot::new(client);

	bot.run_loop()
}
