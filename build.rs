extern crate gcc;

fn main() {
    gcc::compile_library("libtf.a", &["extern/threefish_block.c"]);
}

