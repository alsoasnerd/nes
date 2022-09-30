use std::io::stdin;

fn main() {
    let mut game = String::new();

    println!("Enter the name of game (without symbols and spaces)");
    stdin().read_line(&mut game).expect("Read Line error");

    game = game.trim().to_lowercase();

    nes::run(&game);
}
