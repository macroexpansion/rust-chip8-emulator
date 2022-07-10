pub struct Register {
    pub vx: [u8; 16],
    pub i: u16,  // index register
    pub pc: u16, // program counter
}

impl Register {
    pub fn new() -> Self {
        Self {
            vx: [0; 16],
            i: 0,
            pc: 0,
        }
    }
}
