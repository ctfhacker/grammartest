//! Reimplementation of
//! https://lemire.me/blog/2019/03/19/the-fastest-conventional-random-number-generator-that-can-pass-big-crush

/// Rng seeded with rdtsc that is generated using Lehmer64
pub struct Rng {
    value: u128,
}

impl Rng {
    pub fn new() -> Rng {
        let mut res = Rng {
            value: unsafe { core::arch::x86_64::_rdtsc() } as u128,
        };

        // Cycle through to create some chaos
        for _ in 0..100 {
            let _ = res.next();
        }

        res
    }

    pub fn next(&mut self) -> usize {
        self.value = self.value.wrapping_mul(0xda942042e4dd58b5);
        (self.value >> 64) as usize
    }
}
