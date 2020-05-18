
pub struct Queue {
    exit_char1: u8,
    exit_char2: u8,

    queue: [u8; 2],
}

impl Queue {
    pub fn new(exit_char1: u8, exit_char2: u8) -> Self {
        Self {
            exit_char1,
            exit_char2,
            queue: [exit_char1, exit_char2],
        }
    }

    pub fn enqueue(&mut self, ch: u8) {
        let tmp = self.queue[1];
        self.queue = [tmp, ch];
    }

    pub fn is_exit_chars(&mut self) -> bool {
        self.queue[0] == self.exit_char1 && self.queue[1] == self.exit_char2
    }
}
