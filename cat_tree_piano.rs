extern crate rand;

use rand::{Rng, prelude::ThreadRng};

fn main() { 
    let mut rng = rand::thread_rng();
    let mut a = 0;
    let mut b = 0;
    let mut c = 0;
    let mut d = 0;
    let mut e = 0;
    let mut f = 0;
    let mut g = 0;
    let mut h = 0;
    let mut i = 0;
    let mut j = 0;
    let mut k = 0;
    let mut l = 0;
    let mut m = 0;
    let mut n = 0;
    let mut o = 0;
    let mut p = 0;
    
    loop {
        a += 1;
        b += rng.gen_range(1, 10);
        c += rng.gen_range(1, 100);
        d += rng.gen_range(1, 1000);
        e += rng.gen_range(1, 10000);
        f += rng.gen_range(1, 100000);
        g += rng.gen_range(1, 1000000);
        h += rng.gen_range(1, 10000000);
        i += rng.gen_range(1, 100000000);
        j += rng.gen_range(1, 1000000000);
        k += rng.gen_range(1, 10000000000);
        l += rng.gen_range(1, 100000000000);
        m += rng.gen_range(1, 1000000000000);
        n += rng.gen_range(1, 10000000000000);
        o += rng.gen_range(1, 100000000000000);
        p += rng.gen_range(1, 1000000000000000);
        if a > 1000 {
            break;
        }
    }
    println!("{} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {}",
        a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p);
}