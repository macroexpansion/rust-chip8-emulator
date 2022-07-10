pub struct Ram {
    pub size: u32,
    pub interp: [u8; 512],
    pub program: [u8; 3584],
}

impl Ram {
    pub fn new() -> Self {
        Self {
            size: 4096,
            interp: [0; 512],
            program: [0; 3584],
        }
    }
}
