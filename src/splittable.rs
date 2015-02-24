extern crate rand;

use std;
use std::num::Int;
use std::mem;
use rand::Rng;
use tf;

/// Structures that can be "split".
pub trait Splittable where <Self as Splittable>::Iter : Iterator<Item=Self> {
    type Iter;
    fn split(&mut self) -> Self;
    /// n-way split.
    fn splitn(&mut self, n: usize) -> <Self as Splittable>::Iter;
}

#[derive(Clone, Debug)]
pub struct RawGen {
    key: tf::Block,
    level: u64,
    position: u64,
    p_index: u16,
}

fn hash_block(key: &tf::Block, lvl: u64, pos:u64) -> tf::Block {
    let blk: tf::Block = unsafe { mem::transmute([lvl, pos, 0, 0]) };
    tf::tf256_process_block(key, &blk)
}

impl RawGen {
    pub fn new(seed: tf::Block) -> Self {
        RawGen {
            key: seed,
            level: 0,
            position: 0,
            p_index: 0,
        }
    }

    pub fn g_next(&mut self) -> tf::Block {
        let blk = self.hash();
        self.level += 1;
        if self.level == <u64 as Int>::max_value() {
            if (self.p_index as usize) < std::u64::BITS {
                self.level = 0;
                self.position |= 1 << self.p_index;
                self.p_index += 1;
            } else { *self = RawGen::new(self.hash()); }
        }
        blk
    }

    fn hash(&self) -> tf::Block {
        hash_block(&self.key, self.level, self.position)
    }
}

struct RawGenIter {
    raw_gen: RawGen,
    shift: usize,
    n: u64,
}

impl Iterator for RawGenIter {
    type Item = RawGen;
    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        if self.n == 0 { None } else {
          let mut right = self.raw_gen.clone();
          right.position |= self.n << self.shift;
          self.n -= 1;
          Some(right)
        }
    }
}

impl Splittable for RawGen {
    type Iter = RawGenIter;

    fn split(&mut self) -> Self {
        if (self.p_index as usize) < std::u64::BITS {
            let pi = self.p_index;
            self.p_index += 1;
            let mut right = self.clone();
            right.position |= 1 << pi;
            right
        } else {
            self.key = self.hash();
            self.level = 0;
            self.position = 0;
            self.p_index = 1;
            let mut right = self.clone();
            right.position = 1;
            right
        }
    }

    fn splitn(&mut self, n: usize) -> <Self as Splittable>::Iter {
        let n = n as u64;
        let x = n.leading_zeros() as u16;
        if x < self.p_index {
            self.key = self.hash();
            self.level = 0;
            self.position = 0;
            self.p_index = 0;
        }
        let pi = self.p_index;
        self.p_index += (std::u64::BITS as u16) - x;
        RawGenIter {
            raw_gen: self.clone(),
            shift: pi as usize,
            n: n,
        }
    }
}

/// A splittable random generator.
///
/// This generator can be efficiently split with the `.split()` method,
/// resulting in two subsequently independent generator states.
///
/// ```
/// extern crate "tf-random-rust" as srand;
/// extern crate rand;
/// use srand::splittable::*;
/// use rand::Rng;
///
/// fn main() {
///     let mut left = Gen::new([0; 32]); // All zero seed
///     let mut right = left.split();
///     println!("{} {}", left.next_u32(), right.next_u32());
/// }
/// ```
///
#[derive(Clone, Debug)]
pub struct Gen {
    gen: RawGen,
    b_index: u16,
    block: [u32; 8],
}

impl Gen {
    pub fn new(seed: tf::Block) -> Self {
        Gen::from_raw(RawGen::new(seed))
    }

    pub fn from_raw(raw: RawGen) -> Self {
        Gen {
            gen: raw,
            b_index: 8,
            block: [0; 8],
        }
    }
}

impl Rng for Gen {
    fn next_u32(&mut self) -> u32 {
        let i =
            if self.b_index == 8 {
                self.b_index = 1;
                self.block = unsafe { mem::transmute(self.gen.g_next()) };
                0
            } else {
                self.b_index += 1;
                self.b_index - 1
            } as usize;
        self.block[i]
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        for c in dest.chunks_mut(32) {
            c.clone_from_slice(&self.gen.g_next());
        }
    }
}

struct GenIter { raw_gen_iter: RawGenIter }

impl Iterator for GenIter {
    type Item = Gen;
    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        self.raw_gen_iter.next().map(Gen::from_raw)
    }
}

impl Splittable for Gen {
    type Iter = GenIter;
    fn split(&mut self) -> Self { Gen::from_raw(self.gen.split()) }
    fn splitn(&mut self, n: usize) -> <Self as Splittable>::Iter {
        GenIter { raw_gen_iter: self.gen.splitn(n) }
    }
}

