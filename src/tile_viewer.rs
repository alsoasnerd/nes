use crate::components::cartridges::ROM;
use crate::components::ppu::Frame;
use crate::components::ppu::SYSTEM_PALLETE;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum;

fn show_tiles(chr_rom: &Vec<u8>, bank: usize) -> Frame {
    assert!(bank <= 1);

    let mut frame = Frame::new();
    let tile_x = 0;
    let tile_y = 0;
    let bank = (bank * 0x1000) as usize;

    for tile in 0..255 {
        if tile != 0 && tile % 20 == 0 {
            tile_x = 0;
            tile_y += 10;
        }

        let tile = &chr_rom[(bank + tile * 16)..=(bank + tile * 16 + 15)];

        for y in 0..=7 {
            let mut upper = tile[y];
            let mut lower = tile[y + 8];

            for x in (0..=7).rev() {
                let value = (1 & upper) << 1 | (1 & lower);
                upper >>= 1;
                lower >>= 1;

                let rgb = match value {
                    0 => SYSTEM_PALLETE[0x01],
                    1 => SYSTEM_PALLETE[0x23],
                    2 => SYSTEM_PALLETE[0x27],
                    3 => SYSTEM_PALLETE[0x30],
                    _ => panic!("Invalid Color"),
                };

                frame.set_pixel(tile_x + x, tile_y + y, rgb);
            }
        }

        tile_x += 10;
    }
    frame
}

pub fn start() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("Tile Viewer", (256.0 * 3.0) as u32, (240.0 * 3.0) as u32)
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

    let bytes: Vec<u8> = std::fs::read("games/pacman.nes").unwrap();
    let rom = ROM::new(&bytes).unwrap();

    let right_bank = show_tiles(&rom.chr_rom, 1);

    texture
        .update(None, right_bank.get_data(), 256 * 3)
        .unwrap();
    canvas.copy(&texture, None, None).unwrap();
    canvas.present();

    loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => std::process::exit(0),

                _ => {}
            }
        }
    }
}
