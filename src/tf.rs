use std::mem;

extern {
    fn Threefish_256_Process_Block(keyPtr: *const u8, blkPtr: *const u8, cryptPtr: *mut u8, w32out: i32);
}

pub type Block = [u8; 32];

pub fn tf256_process_block(key: &Block, blk: &Block) -> Block {
    unsafe {
        let mut crypt: Block = mem::uninitialized();
        Threefish_256_Process_Block(
            key.as_ptr(),
            blk.as_ptr(),
            crypt.as_mut_ptr(),
            1);
        crypt
    }
}

pub fn tf256_hash(key: &Block, s: &[u8]) -> Block {
    let mut k = key.clone();
    for c in s.chunks(32) {
        let mut buff = [0; 32];
        let c = if c.len() == 32 { c } else { buff.clone_from_slice(c); &buff[..] };
        let mut crypt: Block = unsafe { mem::uninitialized() };
        unsafe {
            Threefish_256_Process_Block(
                k.as_ptr(),
                c.as_ptr(),
                crypt.as_mut_ptr(),
                1);
        }
        k = crypt;
    }
    k
}

