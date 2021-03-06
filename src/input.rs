use sdl2::EventPump;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use utils::*;
use programstate::*;
use emulator::Emulator;

pub fn handle_input(events: &mut EventPump, state: &mut ProgramState, dstate: &mut DebugState, 
                    emu: &mut Emulator, dev_keys_enabled: bool, only_gb_buttons: bool) {
	for event in events.poll_iter() {
        match event {
            Event::Quit{..} => {
                state.done = true;
            },
            Event::KeyDown{keycode: key, ..} => {
            	if let Some(key) = key {
                    handle_keydown(key, state, dstate, emu, dev_keys_enabled, only_gb_buttons);
                    emu.update_keys(key as u8, true);
            	}
            },
            Event::KeyUp{keycode: key, ..} => {
                if let Some(key) = key {
                    emu.update_keys(key as u8, false);
                }
            },
            _ => ()
        }
    }
}

fn handle_keydown(key: Keycode, state: &mut ProgramState, dstate: &mut DebugState, emu: &Emulator, 
                    dev_keys_enabled: bool, only_gb_buttons: bool) {
    if only_gb_buttons {
        return state.done = key == Keycode::Escape;
    }
	match key {
        Keycode::Num1 => {state.speed = 1},
        Keycode::Num2 => {state.speed = 2},
        Keycode::Num3 => {state.speed = 3},
        Keycode::Num4 => {state.speed = 4},
        Keycode::Num5 => {state.speed = 5},
        Keycode::Num6 => {state.speed = 6},
        Keycode::Num7 => {state.speed = 7},
        Keycode::Num8 => {state.speed = 8},
        Keycode::Num9 => {state.speed = 9},
        Keycode::Num0 => {state.speed = 10},
		Keycode::D if dev_keys_enabled => {
            state.debug = !state.debug; 
            if dstate.num_lines > 0 && !state.debug {
                // TODO: Use NUM_CHARS_PER_LINE to make this right length
                dstate.buffer += "========== QUIT DEBUG MODE ===========\n";
                dstate.num_lines += 1;
            }
            dstate.cursor = dstate.num_lines;
        },
        Keycode::R if dev_keys_enabled => {state.debug_regs = !state.debug_regs},
        Keycode::F if dev_keys_enabled => {state.adv_frame = true},
        Keycode::P => {state.paused = !state.paused},
        Keycode::M if dev_keys_enabled => {
            //Prompt use for range of memory and then dump memory
            let start = prompt_for_val("Enter the starting memory address: ");
            let stop  = prompt_for_val("Enter the ending memory adress: ");

            let start = string_to_u16(&start).unwrap_or(0);
            let stop  = string_to_u16(&stop).unwrap_or(0xFFFF);

            let diff = stop - start;
            let num_rows = (diff as f64/16f64).ceil() as u16;

            for row in 0..num_rows {
                print!("{:#X}: ", row*16 + start);
                let end = if (diff - row*16) < 16 {diff - row*16} else {16};
                for col in 0..end {
                    print!("{:#X} ", emu.rb(row*16 + col + start));
                }
                println!("");
            }
            println!("");
        },
        Keycode::Up if dev_keys_enabled => {
            if (state.paused || emu.is_stopped()) && state.debug {
                dstate.cursor = max(dstate.cursor-1, 0);
            }
        },
        Keycode::Down if dev_keys_enabled => {
            if (state.paused || emu.is_stopped()) && state.debug {
                dstate.cursor = min(dstate.cursor+1, dstate.num_lines-NUM_LINES_ON_SCREEN);
            }
        },
		Keycode::Escape => {state.done = true},
		_ => ()
	}
}