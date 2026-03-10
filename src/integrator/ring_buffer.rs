// This module implements a basic ring buffer using MaybeUninit

use core::mem::MaybeUninit;

#[derive(Debug)]
struct RingBuffer<T: Copy, const N: usize> {
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

    pub fn push(&mut self, item: T) -> Result<(), T> {
        // The ring buffer is full when the next_head equals the tail
        if Self::next_index(self.head) == self.tail {
            Err(item)
        } else {
            self.buffer[self.head].write(item);
            self.head = Self::next_index(self.head);
            Ok(())
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        // The ring buffer is empty when the head equals the tail
        if self.head == self.tail {
            None
        } else {
            // Safe because the value will be initialized if tail != head
            let val = unsafe { Some(self.buffer[self.tail].assume_init()) };
            self.tail = Self::next_index(self.tail);
            val
        }
    }

    fn next_index(index: usize) -> usize {
        (index + 1) % N
    }

}

impl<T: Copy, const N: usize> Drop for RingBuffer<T, N> {
    fn drop(&mut self) {
        while self.pop().is_some() {}
    }
}
