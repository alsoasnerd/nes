pub mod components;
pub mod render;
pub mod trace;

use components::bus::BUS;
use components::cartridge::Rom;
use components::cpu::CPU;
use components::joypads::Joypad;
use components::ppu::PPU;
use render::Frame;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate bitflags;

pub fn run(game: &str) {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window(game, (256.0 * 3.0) as u32, (240.0 * 3.0) as u32)
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
    let rom = Rom::new(&bytes).unwrap();

    let mut frame = Frame::new();

    let bus = BUS::new(rom, move |ppu: &PPU| {
        render::render(ppu, &mut frame);
        texture.update(None, &frame.data, 256 * 3).unwrap();

        canvas.copy(&texture, None, None).unwrap();
        canvas.present();

        let joypad = Joypad::new();
        let keymap = joypad.keymap;

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => std::process::exit(0),

                Event::KeyDown { keycode, .. } => {
                    if let Some(key) = keymap.get(&keycode.unwrap_or(Keycode::Ampersand)) {
                        // joypad.set_button_pressed_status(key, true)
                    }
                }

                Event::KeyUp { keycode, .. } => {
                    if let Some(key) = keymap.get(&keycode.unwrap_or(Keycode::Ampersand)) {
                        // joypad.set_button_pressed_status(key, false)
                    }
                }

                _ => { /* do nothing */ }
            }
        }
    });

    let mut cpu = CPU::new(bus);

    cpu.reset();
    cpu.run();
}
