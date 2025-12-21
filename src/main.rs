use std::env;
use std::process;

use zx_spectrum_emulator::cpu::Cpu;
use zx_spectrum_emulator::memory::load_rom;
use zx_spectrum_emulator::Emulator;

fn main() {
    let args: Vec<String> = env::args().collect();

    // We need a minimum of 2 args
    if args.len() < 2 {
        eprintln!("Usage: {} <rom_file> [--debug]", args[0]);
        process::exit(1);
    }

    // Check if debug is enabled
    let debug_enabled: bool = args.contains(&"--debug".to_string());

    if debug_enabled {
        println!("Debug mode enabled...");
    } else {
        println!("Debug mode disabled...");
    }

    // Remove --debug from args if it exists
    let args: Vec<String> = args.into_iter().filter(|arg| arg != "--debug").collect();
    // Load ROM file from args[1]
    let rom = match load_rom(&args[1]) {
        Ok(data) => {
            println!("Loaded ROM: {} ({} bytes)", args[1], data.len());
            data
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            process::exit(1);
        }
    };

    let mut emulator = match Emulator::new(rom, debug_enabled) {
        Ok(emu) => emu,
        Err(e) => {
            eprintln!("Failed to create emulator: {}", e);
            process::exit(1);
        }
    };

    println!("Starting emulation...\n");

    // ZX Spectrum 48K timing
    const CYCLES_PER_FRAME: u64 = 69888; // 3.5MHz / 50Hz
    const INIT_FRAMES: u32 = 50; // Wait 50 frames (~1 second) for ROM to initialise

    let mut total_cycles = 0u64;
    let mut frame_count = 0u32;
    let mut _frames_since_init = 0u32;

    // Initialise screen to white paper, black ink
    emulator.clear_screen(0, 7, false);

    while emulator.is_window_open() {
        let target_cycles = total_cycles + CYCLES_PER_FRAME;
        let mut frame_instruction_count = 0;

        while total_cycles < target_cycles {
            let cycles = emulator.step();
            total_cycles += cycles as u64;
            frame_instruction_count += 1;

            if emulator.is_halted() {
                println!("CPU halted at PC: 0x{:04X}", emulator.cpu().pc);
                break;
            }

            // Safety check to prevent infinite loops
            if frame_instruction_count > 200000 {
                eprintln!("WARNING: Too many instructions in one frame!");
                break;
            }
        }

        frame_count += 1;

        // Wait for init period
        if frame_count < INIT_FRAMES {
            if frame_count % 10 == 0 {
                println!("Initialising... frame {}/{}", frame_count, INIT_FRAMES);
            }

            // Still update display to keep window responsive
            emulator.update_display().unwrap_or_else(|e| {
                eprintln!("Error updating display: {}", e);
            });
        } else {
            // Start rendering
            if frame_count == INIT_FRAMES {
                println!("Initialisation complete! Starting display rendering");
                println!("Total cycles executed: {}", total_cycles);
                emulator.dump_system_info();
            }

            _frames_since_init += 1;

            // Get keyboard input
            emulator.update_keyboard();

            let keys = emulator.video().get_keys();

            // Debug key shortcuts
            if keys.contains(&minifb::Key::F1) {
                emulator.dump_system_info();
            }

            // Reset emulator
            if keys.contains(&minifb::Key::F5) {
                println!("Resetting emulator...");
                *emulator.cpu_mut() = Cpu::new();
                emulator.clear_screen(0, 7, false);
                total_cycles = 0;
                frame_count = 0;
                _frames_since_init = 0;
            }

            // Border colour cycling (for testing)
            if keys.contains(&minifb::Key::Key1) {
                emulator.set_border_colour(0); // Black
            }
            if keys.contains(&minifb::Key::Key2) {
                emulator.set_border_colour(1); // Blue
            }
            if keys.contains(&minifb::Key::Key3) {
                emulator.set_border_colour(2); // Red
            }
            if keys.contains(&minifb::Key::Key4) {
                emulator.set_border_colour(3); // Magenta
            }
            if keys.contains(&minifb::Key::Key5) {
                emulator.set_border_colour(4); // Green
            }
            if keys.contains(&minifb::Key::Key6) {
                emulator.set_border_colour(5); // Cyan
            }
            if keys.contains(&minifb::Key::Key7) {
                emulator.set_border_colour(6); // Yellow
            }
            if keys.contains(&minifb::Key::Key8) {
                emulator.set_border_colour(7); // White
            }

            // Render display
            emulator
                .render_display()
                .unwrap_or_else(|e| eprintln!("Display error: {}", e));
        }

        // Maintain ~50Hz refresh rate (Spectrum standard)
        std::thread::sleep(std::time::Duration::from_millis(20));
    }

    println!("\nEmulation stopped.");
    println!("Total frames: {}", frame_count);
    println!("Total cycles: {}", total_cycles);
    println!(
        "Average cycles per frame: {}",
        if frame_count > 0 {
            total_cycles / frame_count as u64
        } else {
            0
        }
    );
}
