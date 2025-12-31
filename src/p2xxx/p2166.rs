// Problem 2166
#[derive(Debug)]
struct Bitset {
    size: u32,
    mask: u64,
    flipped: bool,
    count: u32,
    bits: Vec<u64>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Bitset {
    #[inline]
    fn _index_shift(idx: i32) -> (usize, usize) {
        let idx = idx as usize;
        (idx / 64, idx % 64)
    }

    fn new(size: i32) -> Self {
        let size = size as u32;
        let len = size.div_ceil(64) as usize;
        let remainder = size % 64;

        let mask = if remainder == 0 {
            u64::MAX
        } else {
            (1u64 << remainder) - 1
        };

        Self {
            size,
            bits: vec![0; len],
            flipped: false,
            mask,
            count: 0,
        }
    }

    fn fix(&mut self, idx: i32) {
        let (index, shift) = Self::_index_shift(idx);
        let bit = if self.flipped { 0 } else { 1u64 << shift };
        let mask = !(1u64 << shift);

        if self.bits[index] & !mask != bit {
            self.count += 1;
            self.bits[index] = (self.bits[index] & mask) | bit;
        }
    }

    fn unfix(&mut self, idx: i32) {
        let (index, shift) = Self::_index_shift(idx);
        let bit = if self.flipped { 1u64 << shift } else { 0 };
        let mask = !(1u64 << shift);

        if self.bits[index] & !mask != bit {
            self.count -= 1;
            self.bits[index] = (self.bits[index] & mask) | bit;
        }
    }

    fn flip(&mut self) {
        self.flipped = !self.flipped;
        self.count = self.size - self.count;
    }

    #[inline]
    fn _normalize(&mut self) {
        if self.flipped {
            self.bits.iter_mut().for_each(|s| *s = !*s);
            *self.bits.last_mut().unwrap() &= self.mask;
            self.flipped = false;

            // println!("normalized {:?}", self.to_string());
        }
    }

    fn all(&mut self) -> bool {
        // self._normalize();

        // let stop = self.bits.len().saturating_sub(1);
        // self.bits[0..stop].iter().all(|&s| s == u64::MAX)
        //     && self.bits.last().copied().unwrap() == self.mask
        self.size == self.count
    }

    fn one(&mut self) -> bool {
        // self._normalize();
        // let stop = self.bits.len().saturating_sub(1);
        // self.bits[0..stop].iter().any(|&s| s != 0)
        //     || (self.bits.last().copied().unwrap() & self.mask) != 0
        self.count >= 1
    }

    fn count(&mut self) -> i32 {
        // self._normalize();

        // self.bits
        //     .iter()
        //     .copied()
        //     .map(|s| s.count_ones())
        //     .sum::<u32>() as i32

        self.count as i32
    }

    fn to_string(&mut self) -> String {
        self._normalize();

        let mut res = String::with_capacity(self.size as usize);
        let (stop, last) = if self.mask != u64::MAX {
            (self.bits.len().saturating_sub(1), self.bits.last().copied())
        } else {
            (self.bits.len(), None)
        };

        for idx in 0..stop {
            let byte = self.bits[idx];
            for shift in 0..64 {
                let bit = 1 << shift;
                res.push(if byte & bit == bit { '1' } else { '0' });
            }
        }

        if let Some(byte) = last {
            for shift in 0..64 {
                let bit = 1 << shift;
                if self.mask & bit == 0 {
                    break;
                }

                res.push(if byte & bit == bit { '1' } else { '0' });
            }
        }

        res
    }
}

/**
 * Your Bitset object will be instantiated and called as such:
 * let obj = Bitset::new(size);
 * obj.fix(idx);
 * obj.unfix(idx);
 * obj.flip();
 * let ret_4: bool = obj.all();
 * let ret_5: bool = obj.one();
 * let ret_6: i32 = obj.count();
 * let ret_7: String = obj.to_string();
 */
fn f() {}
