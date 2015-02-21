extern crate gcc;

fn main() {
    gcc::compile_library("libtf.a", &["cbits/threefish_block.c"]);
}

