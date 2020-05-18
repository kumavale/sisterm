
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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enqueue() {
        let mut q = Queue::new(b'~', b'.');

        q.enqueue(b'A');
        q.enqueue(b'B');
    }

    #[test]
    fn test_is_exit_chars() {
        let mut q = Queue::new(b'~', b'.');

        q.enqueue(b'A');
        q.enqueue(b'B');
        assert!(!q.is_exit_chars());

        q.enqueue(b'~');
        q.enqueue(b'.');
        assert!(q.is_exit_chars());

        q.enqueue(b'.');
        q.enqueue(b'~');
        assert!(!q.is_exit_chars());
    }
}
