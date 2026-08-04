/// xorshift64* — tiny, deterministic, good enough for jitter. No `rand` dep.
pub struct Rng(u64);

impl Rng {
    pub fn new(seed: u64) -> Self {
        // Golden-ratio scramble; state must be non-zero.
        Rng(seed.wrapping_mul(0x9E37_79B9_7F4A_7C15) | 1)
    }
    fn next_u64(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.0 = x;
        x.wrapping_mul(0x2545_F491_4F6C_DD1D)
    }
    pub fn uniform(&mut self) -> f64 {
        (self.next_u64() >> 11) as f64 / (1u64 << 53) as f64
    }
    pub fn range(&mut self, lo: f64, hi: f64) -> f64 {
        lo + (hi - lo) * self.uniform()
    }
    /// Triangular noise in [-1, 1], centered at 0 (cheap gaussian stand-in).
    pub fn tri(&mut self) -> f64 {
        self.uniform() + self.uniform() - 1.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deterministic_for_same_seed() {
        let a: Vec<f64> = {
            let mut r = Rng::new(42);
            (0..8).map(|_| r.uniform()).collect()
        };
        let b: Vec<f64> = {
            let mut r = Rng::new(42);
            (0..8).map(|_| r.uniform()).collect()
        };
        assert_eq!(a, b);
        let c: Vec<f64> = {
            let mut r = Rng::new(43);
            (0..8).map(|_| r.uniform()).collect()
        };
        assert_ne!(a, c);
    }

    #[test]
    fn uniform_in_unit_interval_even_for_seed_zero() {
        let mut r = Rng::new(0);
        for _ in 0..1000 {
            let u = r.uniform();
            assert!((0.0..1.0).contains(&u));
        }
    }
}
