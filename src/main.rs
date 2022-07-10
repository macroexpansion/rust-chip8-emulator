use chip8::Chip8;

fn main() {
    println!("test");

    let mut emu = Chip8::new();
    emu.load_game("pong");
}
