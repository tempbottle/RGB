#[macro_use]
extern crate glium;
extern crate glium_sdl2;
extern crate sdl2;
extern crate tini;

mod emulator;
mod input;
mod rendering;
mod programstate;

use glium_sdl2::DisplayBuild;
use tini::Ini;

use emulator::emulator::Emulator;
use input::*;
use rendering::*;
use programstate::*;

fn main() {
    let mut state = ProgramState::new();

	let config = Ini::from_file("settings.ini").unwrap();
	let game_path: String = config.get("game", "game").unwrap();
	let buttons = ["up", "down", "left", "right", "a", "b", "start", "select"];
	let controls: Vec<u8> = buttons.iter()
								   .map(|a| config.get("controls", a).unwrap())
								   .collect();

	let mut emu = Emulator::new();
	emu.set_controls(controls);
    emu.load_game(game_path.clone());

	let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let display = video_subsystem.window("Rust Gameboy", 800, 600)
                                 .resizable()
                                 .build_glium()
                                 .unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();
    let renderer = Renderer::new(&display);
    while !state.done {
        handle_input(&mut event_pump, &mut state);
        if !state.paused {
            emu.emulate_cycle(&state);
        }
        renderer.render(&display, emu.gpu.get_screen());
    }
}