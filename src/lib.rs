pub mod tf;
pub mod splittable;

mod test {
    use tf;
    use splittable::*;
    use std;

    #[test]
    fn tf_determinism() {
        let s = [42; 1000];
        let key = [0; 32];
        let h = tf::tf256_hash(&key, &s[]);
        let t = [42; 1000];
        let kei = [0; 32];
        let i = tf::tf256_hash(&kei, &t[]);
        assert_eq![h, i];
    }

    #[test]
    fn rng_determinism() {
        let k = [32; 32];
        let mut g = GenU32::new(k);
        let mut h = GenU32::new(k);
        for _ in 0..100000 {
            let a = g.next();
            let b = h.next();
            //println!("{}", std::rand::random::<u32>());
            assert_eq!(a, b);
            println!("{}", a);
        }
    }
}
