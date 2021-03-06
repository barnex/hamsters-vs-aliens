use super::internal::*;
use gl_safe::*;
use glutin::event::{DeviceEvent, ElementState, Event, VirtualKeyCode, WindowEvent};
use glutin::event_loop::ControlFlow;
use glutin::window;
use std::path::PathBuf;
use std::thread::sleep;
use std::time::{Duration, Instant};
use structopt::StructOpt;

type Window = glutin::ContextWrapper<glutin::PossiblyCurrent, glutin::window::Window>;
type EventLoop = glutin::event_loop::EventLoop<()>;

#[derive(StructOpt)]
struct Args {
	/// Resolution: width (pixels).
	#[structopt(short, long, default_value = "1024")]
	pub width: u32,

	/// Resolution: height (pixels).
	#[structopt(short, long, default_value = "768")]
	pub height: u32,

	/// Run in borderless fullscreen mode
	#[structopt(short, long)]
	pub fullscreen: bool,

	/// Disable window resizing.
	#[structopt(long)]
	pub no_resize: bool,

	/// Disable vsync.
	#[structopt(long)]
	pub no_vsync: bool,

	/// Framerate cap in milliseconds.
	#[structopt(long, default_value = "1")]
	pub fps_cap_ms: u32,

	/// Print frames per second info.
	#[structopt(long)]
	pub print_fps: bool,

	/// Render wire frame instead of solid faces (DEBUG).
	#[structopt(long)]
	pub wireframe: bool,

	/// Disable alpha blending.
	#[structopt(long)]
	pub no_alpha: bool,

	/// Disable face culling (DEBUG)
	#[structopt(long)]
	pub no_cull_face: bool,

	/// Multi-sampling anti aliasing number of samples (must be a power of 2).
	#[structopt(long, default_value = "8")]
	pub msaa: u16,

	/// Texture directory.
	#[structopt(long, default_value = "assets/textures/hi/")]
	pub textures: String,

	/// Mesh directory.
	#[structopt(long, default_value = "assets/obj/")]
	pub meshes: String,

	/// Mouse sensitivity.
	#[structopt(long, default_value = "100")]
	pub mouse_sens: f64,

	/// Player skin (1: hamster, 2: frog).
	#[structopt(long, default_value = "1")]
	pub skin: usize,

	/// Server address
	#[structopt(long, default_value = "localhost:3344")]
	pub server: String,
}

pub fn main_loop() -> Result<()> {
	let args = Args::from_args();

	let client = Client::connect(&args.server, args.skin)?;
	let mut controller = LocalPlayer::new(client);

	// this initializes the GL context, has to be called before any other GL calls.
	let (win, event_loop) = init_gl_window(&args);
	init_gl_options(&args);
	let mut ctx = GLContext::new(&PathBuf::from(&args.textures), &PathBuf::from(&args.meshes));

	// continuously pump redraws
	let fps_cap_time = Duration::from_millis(args.fps_cap_ms as u64);
	let proxy = event_loop.create_proxy();
	std::thread::spawn(move || loop {
		proxy.send_event(()).expect("send event"); // empty user event used to signal redraw request.
		sleep(fps_cap_time);
	});

	// main loop
	let mut fps = FramerateCounter::new(args.print_fps);
	let mut input_grabbed = release_input(&win, false /*input_grabbed*/); // start not grabbed, some window systems refuse to grab if cursor not in window.
	let mut last_tick = Instant::now();

	let mouse_sens = args.mouse_sens * 0.00001;

	event_loop.run(move |event, _, control_flow| {
		*control_flow = ControlFlow::Wait;
		match event {
			Event::LoopDestroyed => *control_flow = ControlFlow::Exit,
			Event::UserEvent(_) => win.window().request_redraw(), // empty user event used to signal redraw request.
			Event::RedrawRequested(_) => {
				{
					let now = Instant::now();
					let dt = (now - last_tick).as_secs_f32();
					last_tick = now;

					controller.tick(dt);
				}

				let size = win.window().inner_size();
				let (width, height) = (size.width, size.height);
				ctx.set_viewport((width, height));

				controller.draw(&ctx);

				win.swap_buffers().unwrap();
				fps.tick();
			}
			Event::DeviceEvent { event, .. } => match event {
				DeviceEvent::MouseMotion { delta, .. } => {
					if input_grabbed {
						controller.record_mouse((delta.0 * mouse_sens, delta.1 * mouse_sens))
					}
				}
				DeviceEvent::Button { button, state } => {
					if input_grabbed {
						match button {
							1 => controller.record_key(Key::Mouse1, state == ElementState::Pressed),
							3 => controller.record_key(Key::Mouse3, state == ElementState::Pressed),
							_ => (),
						}
					}
				}
				DeviceEvent::Key(input) => {
					if let Some(code) = input.virtual_keycode {
						if code == VirtualKeyCode::Escape {
							input_grabbed = release_input(&win, input_grabbed);
						}
						if input_grabbed {
							if let Some(key) = keymap(code) {
								let pressed = input.state == ElementState::Pressed;
								controller.record_key(key, pressed)
							}
						}
					}
				}
				_ => (),
			},
			Event::WindowEvent { event, .. } => match event {
				WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
				WindowEvent::MouseInput { /*state, button*/ .. } => {
					input_grabbed = grab_input(&win, input_grabbed);
				}
				_ => (),
			},
			_ => (),
		}
	});
}

// use: grabbed = grab_input(&window, grabbed);
#[must_use]
fn grab_input(win: &Window, input_grabbed: bool) -> bool {
	if !input_grabbed {
		println!("Press ESC to release mouse");
		if let Err(e) = win.window().set_cursor_grab(true) {
			eprintln!("failed to grab curor: {}", e);
		}
		win.window().set_cursor_visible(false);
	}
	true
}

// use: grabbed = release_input(&window, grabbed);
#[must_use]
fn release_input(win: &Window, input_grabbed: bool) -> bool {
	if input_grabbed {
		let _ = win.window().set_cursor_grab(false);
		win.window().set_cursor_visible(true);
	}
	false
}

/// Initialize the GL context
/// and create a window and associated event loop.
fn init_gl_window(args: &Args) -> (Window, EventLoop) {
	let title = "hamsters vs aliens";
	let size = glutin::dpi::LogicalSize::new(args.width, args.height);
	let fullscreen = if args.fullscreen { Some(window::Fullscreen::Borderless(None)) } else { None };
	let event_loop = glutin::event_loop::EventLoop::new();
	let window = glutin::window::WindowBuilder::new() //
		.with_inner_size(size)
		.with_title(title)
		.with_fullscreen(fullscreen)
		.with_resizable(!args.no_resize);
	let gl_window = glutin::ContextBuilder::new() //
		.with_vsync(!args.no_vsync)
		.with_multisampling(args.msaa)
		.build_windowed(window, &event_loop)
		.unwrap();
	let gl_window = unsafe { gl_window.make_current() }.unwrap();
	gl::load_with(|symbol| gl_window.get_proc_address(symbol));
	(gl_window, event_loop)
}

// GL setup
fn init_gl_options(args: &Args) {
	if args.msaa != 0 {
		glEnable(gl::MULTISAMPLE);
	}
	if args.wireframe {
		glPolygonMode(gl::FRONT_AND_BACK, gl::LINE);
	}

	if !args.no_alpha {
		// TODO: ctx.enable_blend(a, b), ctx.disable_blend()
		glBlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA); // TODO: tweak?
		glEnable(gl::BLEND);
	}
}
