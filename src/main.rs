use num::BigInt;
use num::Zero;
use num::One;
use std::io::{self, Write};

fn main() {
    // Initialize variables
    let mut q = BigInt::one();
    let mut r = BigInt::zero();
    let mut t = BigInt::one();
    let mut k = BigInt::one();
    let mut n = BigInt::from(3);
    let mut l = BigInt::from(3);

    // Prepare for efficient output
    let stdout = io::stdout();
    let mut handle = stdout.lock();

    loop {
        // Compute temporary value
        let four_qr = &q * 4 + &r;

        if &four_qr - &t < &n * &t {
            // Output the next digit of π
            write!(handle, "{}", n).unwrap();
            handle.flush().unwrap();

            // Update variables for the next iteration
            let tmp_r = (&r - &n * &t) * 10;
            q = &q * 10;
            r = tmp_r;
            n = ((&q * 3 + &r) / &t) - &n * 10;
        } else {
            // Update variables when the next digit is not ready
            let q_old = q.clone();
            let r_old = r.clone();
            let t_old = t.clone();
            let k_old = k.clone();
            let l_old = l.clone();

            q = &q * &k;
            r = (&q_old * 2 + &r_old) * &l_old;
            t = &t * &l_old;
            k = &k + 1;
            l = &l + 2;
            n = (&q_old * (&k_old * 7 + 2) + &r_old * &l_old) / (&t_old * &l_old);
        }
    }
}
