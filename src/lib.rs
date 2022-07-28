use components::cpu::CPU;
use components::cartridges::ROM;
use components::bus::BUS;
use components::ppu::PPU;
use components::ppu::Frame;
use components::ppu::render;
use sdl2::{
    event::Event,
    keyboard::Keycode,
    pixels::{Color, PixelFormatEnum}, EventPump,
};

pub mod components;
pub mod tile_viewer;
pub mod trace;

fn color(byte: u8) -> Color {
    match byte {
        0 => sdl2::pixels::Color::BLACK,
        1 => sdl2::pixels::Color::WHITE,
        2 | 9 => sdl2::pixels::Color::GREY,
        3 | 10 => sdl2::pixels::Color::RED,
        4 | 11 => sdl2::pixels::Color::GREEN,
        5 | 12 => sdl2::pixels::Color::BLUE,
        6 | 13 => sdl2::pixels::Color::MAGENTA,
        7 | 14 => sdl2::pixels::Color::YELLOW,
        _ => sdl2::pixels::Color::CYAN,
    }
}

fn read_screen_state(cpu: &mut CPU, frame: &mut [u8; 32 * 3 * 32]) -> bool {
    let mut frame_index = 0;
    let mut update = false;
    for i in 0x0200..0x600 {
        let color_index = cpu.memory_read(i as u16);
        let (r, g, b) = color(color_index).rgb();
        if frame[frame_index] != r || frame[frame_index + 1] != g || frame[frame_index + 2] != b {
            frame[frame_index] = r;
            frame[frame_index + 1] = g;
            frame[frame_index + 2] = b;
            update = true;
        }
        frame_index += 3;
    }
    update
}

fn handle_user_input(cpu: &mut CPU, event_pump: &mut EventPump) {
    for event in event_pump.poll_iter() {
        match event {
            Event::Quit { .. }
            | Event::KeyDown {
                keycode: Some(Keycode::Escape),
                ..
            } => std::process::exit(0),
            Event::KeyDown {
                keycode: Some(Keycode::W),
                ..
            } => {
                cpu.memory_write(0xff, 0x77);
            }
            Event::KeyDown {
                keycode: Some(Keycode::S),
                ..
            } => {
                cpu.memory_write(0xff, 0x73);
            }
            Event::KeyDown {
                keycode: Some(Keycode::A),
                ..
            } => {
                cpu.memory_write(0xff, 0x61);
            }
            Event::KeyDown {
                keycode: Some(Keycode::D),
                ..
            } => {
                cpu.memory_write(0xff, 0x64);
            }
            _ => {}
        }
    }
}

pub fn start(game: &str) {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("Tile viewer", (256.0 * 3.0) as u32, (240.0 * 3.0) as u32)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().present_vsync().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();
    canvas.set_scale(3.0, 3.0).unwrap();

    let creator = canvas.texture_creator();
    let mut texture = creator
        .create_texture_target(PixelFormatEnum::RGB24, 256, 240)
        .unwrap();
    let path_to_game = format!("games/{}.nes", game);
    let bytes: Vec<u8> = std::fs::read(path_to_game).unwrap();
    let rom = ROM::new(&bytes).unwrap();

    let mut frame = Frame::new();

    let bus = BUS::new(rom, move |ppu: &PPU| {
        render(ppu, &mut frame);

        texture.update(None, frame.get_data(), 256 * 3).unwrap();
        canvas.copy(&texture, None, None).unwrap();
        canvas.present();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    std::process::exit(0);
                }

                _ => {}
            }
        }
    });

    let mut cpu = CPU::new(bus);
    cpu.reset();
    cpu.run();
}
