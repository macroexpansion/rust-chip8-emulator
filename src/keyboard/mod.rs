pub struct Keyboard {
    pub key: [u8; 16],
}

impl Keyboard {
    pub fn new() -> Self {
        Self {
            key: [0; 16],
        }
    }
}
