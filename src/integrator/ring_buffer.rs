// This module implements a basic ring buffer using MaybeUninit

use core::mem::MaybeUninit;

#[derive(Debug)]
pub struct RingBuffer<T: Copy, const N: usize> {
    buffer: [MaybeUninit<T>; N],
    head: usize,
    tail: usize,
}

impl<T: Copy, const N: usize> RingBuffer<T, N> {
    pub const fn new() -> RingBuffer<T, N> {
        RingBuffer {
            buffer: [const { MaybeUninit::<T>::uninit() }; N],
            head: 0,
            tail: 0,
        }
    }

    pub fn push(&mut self, item: T) -> Result<(), &str> {
        if self.is_full() {
            Err("Buffer is full")
        } else {
            self.buffer[self.head].write(item);
            self.head = Self::next_index(self.head);
            Ok(())
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        // The ring buffer is empty when the head equals the tail
        if self.is_empty() {
            None
        } else {
            // Safe because the value will be initialized if tail != head
            let val = unsafe { Some(self.buffer[self.tail].assume_init()) };
            self.tail = Self::next_index(self.tail);
            val
        }
    }

    pub fn is_full(&self) -> bool {
        // The ring buffer is full when the next_head equals the tail
        Self::next_index(self.head) == self.tail
    }

    pub fn is_empty(&self) -> bool {
        // The ring buffer is empty when the head equals the tail
        self.head == self.tail
    }

    fn next_index(index: usize) -> usize {
        (index + 1) % N
    }

    fn ref_at_index(&self, index: usize) -> Result<&T, &str> {
        // Make sure the index is within the range of indicies of the buffer
        //
        // Three cases
        // If head == tail then the buffer is empty
        // If head < tail then the in-range indecies after  head or  before tail have been initialized
        // If head > tail then the in-range indecies before head and after  tail have been initialized
        //
        // NOTE: the head points to uninitialized data in this implementation, but the tail points
        // to initialized data

        let mut is_in_init_vals = false;
        if index >= N {
            return Err("index out of bounds");
        }

        match self.head.cmp(&self.tail) {
            core::cmp::Ordering::Equal => {}
            core::cmp::Ordering::Less => {
                if index > self.head || index <= self.tail {
                    is_in_init_vals = true;
                }
            }
            core::cmp::Ordering::Greater => {
                if index < self.head && index >= self.tail {
                    is_in_init_vals = true;
                }
            }
        }

        // Safety: The value at the index has been initialized if it passes the above checks
        if is_in_init_vals {
            Ok(unsafe { self.buffer[index].assume_init_ref() })
        } else {
            Err("Index in uninitialized data")
        }
    }

    pub fn iter(&self) -> RingBufferIter<'_, T, N> {
        self.into_iter()
    }
}

impl<T: Copy, const N: usize> Drop for RingBuffer<T, N> {
    fn drop(&mut self) {
        while self.pop().is_some() {}
    }
}

// Implement PartialEq for testing purposes
impl<T: Copy + PartialEq, const N: usize> PartialEq for RingBuffer<T, N> {
    fn eq(&self, other: &Self) -> bool {
        // We don't need the head and tail values to be the same for equality
        self.iter()
            .zip(other)
            .fold(true, |acc, (l, r)| acc && (l == r))
    }
}

//--------------------Iterator Implementation--------------------
impl<'a, T: Copy, const N: usize> IntoIterator for &'a RingBuffer<T, N> {
    type Item = &'a T;
    type IntoIter = RingBufferIter<'a, T, N>;

    fn into_iter(self) -> Self::IntoIter {
        RingBufferIter {
            ring_buffer: self,
            index: self.tail,
        }
    }
}

#[derive(Debug)]
pub struct RingBufferIter<'a, T: Copy, const N: usize> {
    ring_buffer: &'a RingBuffer<T, N>,
    index: usize,
}

impl<'a, T: Copy, const N: usize> Iterator for RingBufferIter<'a, T, N> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        let next = self.ring_buffer.ref_at_index(self.index).ok();
        self.index = self.index.saturating_add(1);
        next
    }
}
//------------------End Iterator Implementation------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initialization() {
        assert_eq!(
            RingBuffer::<f32, 8>::new(),
            RingBuffer {
                buffer: [MaybeUninit::uninit(); 8],
                head: 0,
                tail: 0,
            }
        )
    }

    #[test]
    fn test_push() {
        let mut buffer = RingBuffer::<f32, 8>::new();
        assert_eq!(buffer.push(0.0), Ok(()))
    }

    #[test]
    fn test_pop() {
        let mut buffer = RingBuffer::<f32, 8>::new();
        let _ = buffer.push(1.0);
        assert_eq!(buffer.pop(), Some(1.0));
    }

    #[test]
    fn test_push_overflow() {
        let mut buffer = RingBuffer::<i32, 4>::new();

        // Note that the buffer can only hold N - 1 values in this implementation
        let _ = buffer.push(1);
        let _ = buffer.push(1);
        let _ = buffer.push(1);

        assert_eq!(buffer.push(1), Err(1));
    }

    #[test]
    fn test_pop_underflow() {
        let mut buffer = RingBuffer::<i32, 4>::new();

        assert_eq!(buffer.pop(), None);
    }

    #[test]
    fn test_compare_not_equal() {
        let mut buffer_1 = RingBuffer::<i32, 8>::new();
        let mut buffer_2 = RingBuffer::<i32, 8>::new();
        for i in 0..4 {
            let _ = buffer_1.push(i);
        }
        for i in 0..4 {
            let _ = buffer_2.push(i * 2);
        }

        assert_ne!(buffer_1, buffer_2)
    }

    #[test]
    fn test_push_pop_compare() {
        const BUFFER_LENGTH: usize = 8;

        let mut buffer_1 = RingBuffer::<i32, BUFFER_LENGTH>::new();
        let mut buffer_2 = RingBuffer::<i32, BUFFER_LENGTH>::new();
        let _ = buffer_1.push(1);
        let _ = buffer_1.push(2);
        let _ = buffer_1.push(3);
        let _ = buffer_1.push(4);
        let _ = buffer_1.push(5);
        let _ = buffer_1.push(6);
        let _ = buffer_1.push(7);
        let _ = buffer_1.pop();
        let _ = buffer_1.pop();
        let _ = buffer_1.pop();

        for i in 4..8 {
            let _ = buffer_2.push(i);
        }

        assert_eq!(buffer_1, buffer_2);
    }
}
