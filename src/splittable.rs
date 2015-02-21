extern crate rand;

use std;
use std::num;
use std::mem;
use rand::Rng;
use tf;

pub trait Splittable {
    fn split(&mut self) -> Self;
}

#[derive(Clone)]
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
        if self.level == <u64 as num::Int>::max_value() {
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

impl Splittable for RawGen {
    fn split(&mut self) -> Self {
        if self.p_index < 64 {
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
}

#[derive(Clone)]
pub struct Gen {
    gen: RawGen,
    b_index: u16,
    block: [u32; 8],
}

impl Gen {
    pub fn new(seed: tf::Block) -> Self {
        Gen {
            gen: RawGen::new(seed),
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

impl Splittable for Gen {
    fn split(&mut self) -> Self {
        let right = self.gen.split();
        Gen {
            gen: right,
            b_index: 8,
            block: [0; 8],
        }
    }
}

