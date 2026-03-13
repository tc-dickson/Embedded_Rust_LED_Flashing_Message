// This module will implement a basic integrator using a ring buffer

use core::ops::{Add, Sub};

mod ring_buffer;

// Integrate by addition of N items
pub struct Integrator<T, const N: usize>
where
    T: Add + Sub + Copy,
{
    buffer: ring_buffer::RingBuffer<T, N>,
    value: T,
}

impl<T, const N: usize> Integrator<T, N>
where
    T: Add<Output = T> + Sub<Output = T> + Copy,
{
    pub fn new(value: T) -> Integrator<T, N> {
        Integrator {
            buffer: ring_buffer::RingBuffer::new(),
            value,
        }
    }

    /// # Errors
    /// This function will return an error if the underlying ring buffer malfunctions.
    pub fn insert(&mut self, inserted_value: T) -> Result<(), &str> {
        // insert values without poping until the buffer is full
        if self.buffer.push(inserted_value).is_ok() {
            return Err("Tried to push when buffer was full");
        }

        let (true, Some(popped_value)) = (self.buffer.is_full(), self.buffer.pop()) else {
            return Err("Tried to pop when buffer was empty");
        };

        self.integrate(inserted_value, popped_value);
        Ok(())
    }

    // Use a simple addition for integration
    fn integrate(&mut self, inserted_value: T, popped_value: T) {
        self.value = self.value + inserted_value;
        self.value = self.value - popped_value;
    }

    pub fn read(&self) -> T {
        self.value
    }
}
