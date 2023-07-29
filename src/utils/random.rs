use solana_program::pubkey::Pubkey;

pub struct SplitMix64 {
    pub seed: u64,
    pub state: u64,
}

impl SplitMix64 {
    pub fn new(seed: &Pubkey) -> Self {
        let seed_bytes = seed.to_bytes();
        let mut seed_num: u64 = 0;
        for i in 0..8 {
            seed_num = (seed_num << 8) + (seed_bytes[i] as u64);
        }
        Self {
            seed: seed_num,
            state: seed_num,
        }
    }

    pub fn reset(&mut self) -> () {
        self.state = self.seed
    }

    pub fn skip(&mut self, amount: u32) -> () {
        for _ in 0..amount {
            self.next();
        }
    }

    pub fn next(&mut self) -> u64 {
        self.state = self.state.wrapping_add(0x9e3779b97f4a7c15);
        let mut z = self.state;
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d049bb133111eb);
        z ^ (z >> 31)
    }

    pub fn range(&mut self, min: i32, max: i32) -> i32 {
        (self.next() % (max - min) as u64) as i32 + min
    }

    pub fn next_double(&mut self) -> f64 {
        let int_value = self.next();
        let float_value = int_value as f64 / u64::MAX as f64;
        float_value
    }
}
