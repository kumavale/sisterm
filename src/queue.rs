use crate::getch::Key;
use std::collections::LinkedList;

pub struct Queue {
    exit_char1: Key,
    exit_char2: Key,

    queue: LinkedList<Key>
}

impl Queue {
    pub fn new(exit_char1: Key, exit_char2: Key) -> Self {
        let mut queue = LinkedList::new();
        queue.push_back(Key::Null);
        queue.push_back(Key::Null);
        Self {
            exit_char1,
            exit_char2,
            queue,
        }
    }

    pub fn enqueue(&mut self, ch: &Key) {
        self.queue.pop_front();
        self.queue.push_back(ch.clone());
    }

    pub fn is_exit_chars(&mut self) -> bool {
        self.queue.front() == Some(&self.exit_char1) && self.queue.back() == Some(&self.exit_char2)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enqueue() {
        let mut q = Queue::new(Key::Char('~'), Key::Char('.'));

        q.enqueue(Key::Char('A'));
        q.enqueue(Key::Char('B'));
    }

    #[test]
    fn test_is_exit_chars() {
        let mut q = Queue::new(Key::Char('~'), Key::Char('.'));

        q.enqueue(Key::Char('A'));
        q.enqueue(Key::Char('B'));
        assert!(!q.is_exit_chars());

        q.enqueue(Key::Char('~'));
        q.enqueue(Key::Char('.'));
        assert!(q.is_exit_chars());

        q.enqueue(Key::Char('.'));
        q.enqueue(Key::Char('~'));
        assert!(!q.is_exit_chars());
    }
}
