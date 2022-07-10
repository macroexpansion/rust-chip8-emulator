pub struct Display {
    graphic: [u8; 64 * 32],
}

impl Display {
    pub fn new() -> Self {
        Self {
            graphic: [0; 64 * 32],
        }
    }
}
