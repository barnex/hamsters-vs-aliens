use game::prelude::*;
use std::fs;
use std::io::BufWriter;
use std::process::exit;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Args {
	/// Serving address + port.
	#[structopt(short, long, default_value = "0.0.0.0:3344")]
	pub addr: String,

	/// Create new file with given size, instead of opening.
	#[structopt(short, long)]
	pub create: Option<u32>,

	/// Auto-save map whenever a client drops out.
	#[structopt(long)]
	pub autosave: bool,

	/// Map file to open
	pub map_file: String,
}

fn main() {
	match main_result() {
		Ok(()) => eprintln!("server exited successfully"),
		Err(e) => {
			eprintln!("error: {}", e);
			exit(1)
		}
	}
}

fn main_result() -> Result<()> {
	let args = Args::from_args();
	let map_file = PathBuf::from(&args.map_file);

	if let Some(size) = args.create {
		let map = Map::flat(uvec3(size, size, size));
		if map_file.exists() {
			return err(&format!("create {}: file already exists", map_file.to_string_lossy()));
		}
		let f = fs::File::create(&args.map_file)?;
		map.serialize(BufWriter::new(f))?;
		println!("created {}, size {}x{}x{}", &args.map_file, size, size, size);
	}

	if args.autosave {
		println!("auto save ENABLED (edit mode)");
	} else {
		println!("auto save DISABLED (play mode)");
	}

	println!("serving map {}", &map_file.to_string_lossy());
	Server::serve(&args.addr, map_file, args.autosave)
}
