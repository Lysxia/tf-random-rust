#![feature(core, collections)]
extern crate rand;

pub mod tf;
pub mod splittable;

#[cfg(test)]
mod test {
    use tf;
    use splittable::*;
    use rand::Rng;

    #[test]
    fn tf_determinism() {
        let s = [42; 1000];
        let key = [0; 32];
        let h = tf::tf256_hash(&key, &s[..]);
        let t = [42; 1000];
        let kei = [0; 32];
        let i = tf::tf256_hash(&kei, &t[..]);
        assert_eq![h, i];
    }

    #[test]
    fn rng_determinism() {
        let k = [32; 32];
        let mut g = Gen::new(k);
        let mut h = Gen::new(k);
        for _ in 0..100000 {
            let a = g.next_u32();
            let b = h.next_u32();
            //println!("{}", std::rand::random::<u32>());
            assert_eq!(a, b);
            println!("{}", a);
        }
    }
}
