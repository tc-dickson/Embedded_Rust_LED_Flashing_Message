// This module implements a basic ring buffer using MaybeUninit

use core::{mem::MaybeUninit, usize};

#[derive(Debug)]
struct RingBuffer<T, const N: usize> {
    buffer: [MaybeUninit<T>; N],
    head: usize,
    tail: usize,
}

impl<T: Copy, const N: usize> RingBuffer<T, N> {
    pub const fn new() -> RingBuffer<T, N> {
        RingBuffer {
            buffer: [MaybeUninit::<T>::uninit(); N],
            head: 0,
            tail: 0,
        }
    }

    pub fn push(&mut self, item: T) -> Result<(), T> {
        // Check if the ring buffer is full
        let next_head = (self.head + 1) % N;
        if next_head == self.tail {
            return Err(item);
        } else {
            self.buffer[self.head].write(item);
            self.head = next_head;
            return Ok(());
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        // Check if ring buffer is empty
        if self.head == self.tail {
            return None;
        } else {
            // Safe because the value will be initialized if head != tail
            let val = unsafe { Some(self.buffer[self.tail].assume_init()) };
            self.tail = (self.tail + 1) % N;
            return val;
        }
    }
}
