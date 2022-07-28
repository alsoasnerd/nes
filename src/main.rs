// use nes::tile_viewer;
use std::io::stdin;
fn main() {
    let mut input = String::new();
    println!("Insert Game Name (Don't use any special character or spaces):");
    stdin().read_line(&mut input).expect("Read Line Failed");

    let game = input.trim().to_lowercase();
    nes::start(&game);
    // tile_viewer::start(&game);
}
