use crate::getch::Key;
use crate::default::escape_signals::*;
use std::collections::LinkedList;

pub struct Queue {
    queue: LinkedList<Key>
}

impl Default for Queue {
    fn default() -> Self {
        Self::new()
    }
}

impl Queue {
    pub fn new() -> Self {
        let mut queue = LinkedList::new();
        queue.push_back(Key::Null);
        queue.push_back(Key::Null);
        Self {
            queue,
        }
    }

    pub fn enqueue(&mut self, ch: &Key) {
        self.queue.pop_front();
        self.queue.push_back(ch.clone());
    }

    pub fn is_exit_chars(&mut self) -> bool {
        if self.queue.front() == Some(&ESCAPE_SIGNAL) {
            self.queue.back() == Some(&EXIT_CHAR_0) || self.queue.back() == Some(&EXIT_CHAR_1)
        } else {
            false
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enqueue() {
        let mut q = Queue::new();

        q.enqueue(&Key::Char('A'));
        q.enqueue(&Key::Char('B'));
    }

    #[test]
    fn test_is_exit_chars() {
        let mut q = Queue::new();

        q.enqueue(&Key::Char('A'));
        q.enqueue(&Key::Char('B'));
        assert!(!q.is_exit_chars());

        q.enqueue(&Key::Char('~'));
        q.enqueue(&Key::Char('.'));
        assert!(q.is_exit_chars());

        q.enqueue(&Key::Char('.'));
        q.enqueue(&Key::Char('~'));
        assert!(!q.is_exit_chars());

        q.enqueue(&Key::Char('~'));
        q.enqueue(&Key::Char('\x04'));
        assert!(!q.is_exit_chars());

        q.enqueue(&Key::Char('~'));
        q.enqueue(&Key::Ctrl('d'));
        assert!(q.is_exit_chars());
    }
}
