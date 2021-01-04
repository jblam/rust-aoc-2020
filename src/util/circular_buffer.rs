use std::fmt::Debug;
use std::cmp::max;

pub struct CircularBuffer<T> {
    buffer: Vec<T>,
    tail: usize,
}
impl<T> CircularBuffer<T> {
    pub fn with_capacity(capacity: usize) -> CircularBuffer<T> {
        if capacity == 0 {
            panic!("Zero-capacity buffer not supported")
        }
        CircularBuffer {
            buffer: Vec::with_capacity(capacity),
            tail: 0,
        }
    }
    pub fn fill_with(items: Vec<T>) -> CircularBuffer<T> {
        CircularBuffer {
            buffer: items,
            tail: 0
        }
    }
    #[inline]
    fn index_into(&self, raw_index: usize) -> usize {
        raw_index % self.buffer.capacity()
    }
    #[inline]
    fn head_unchecked(&self) -> usize {
        self.index_into(self.tail + self.buffer.capacity())
    }
    pub fn push(&mut self, item: T) {
        if !self.is_full() {
            self.buffer.push(item);
        } else {
            self.buffer[self.tail] = item;
            self.tail = self.index_into(self.tail + 1);
        }
    }
    pub fn is_full(&self) -> bool {
        debug_assert!(self.buffer.len() <= self.buffer.capacity());
        self.buffer.len() == self.buffer.capacity()
    }
    pub fn len(&self) -> usize {
        if !self.is_full() {
            self.buffer.len()
        } else {
            self.buffer.capacity()
        }
    }
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        let (u, v) = self.slices();
        u.iter().chain(v.iter())
    }
    pub fn slices(&self) -> (&[T], &[T]) {
        if !self.is_full() {
            (self.buffer.as_slice(), &[])
        } else {
            (
                &self.buffer[self.tail..],
                &self.buffer[..self.head_unchecked()],
            )
        }
    }
}

impl<T: Debug> Debug for CircularBuffer<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:?} @ {}",
            self.buffer,
            max(self.head_unchecked(), self.len())
        )
    }
}
impl<T: PartialEq> PartialEq for CircularBuffer<T> {
    fn eq(&self, other: &Self) -> bool {
        // JB 2021-01-02: can do advanced magic with slices, but unnecessary
        // for any existing requirements
        self.iter().eq(other.iter())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn can_enumerate_empty_buffer() {
        let b = CircularBuffer::<i32>::with_capacity(2);
        let mut i = b.iter();
        assert_eq!(None, i.next())
    }
    #[test]
    fn can_enumerate_partial_buffer() {
        let mut b = CircularBuffer::with_capacity(2);
        b.push(0i32);
        let mut i = b.iter();
        assert_eq!(Some(&0), i.next());
        assert_eq!(None, i.next());
    }
    #[test]
    fn can_enumerate_full_buffer() {
        let mut b = CircularBuffer::with_capacity(2);
        b.push(0i32);
        b.push(1);
        let mut i = b.iter();
        assert_eq!(Some(&0), i.next());
        assert_eq!(Some(&1), i.next());
        assert_eq!(None, i.next());
    }
    #[test]
    fn can_enumerate_overfull_buffer() {
        let mut b = CircularBuffer::with_capacity(2);
        b.push(0i32);
        b.push(1);
        b.push(2);
        let mut i = b.iter();
        assert_eq!(Some(&1), i.next());
        assert_eq!(Some(&2), i.next());
        assert_eq!(None, i.next());
    }
}
