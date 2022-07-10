pub struct Timers {
    pub delay_timer: u8,
    pub sound_timer: u8,
}

impl Timers {
    pub fn new(delay: u8, sound: u8) -> Self {
        Self {
            delay_timer: delay,
            sound_timer: sound,
        }
    }
}
