extern crate test;
extern crate specfn;

use std::rand::random;

#[bench]
fn inc_beta(b: &mut test::Bencher) {
    let (p, q) = (0.5, 1.5);
    let log_beta = specfn::log_beta(p, q);
    let x: Vec<f64> = range(0u, 1000).map(|_| random()).collect();

    b.iter(|| {
        for &x in x.iter() {
            test::black_box(specfn::inc_beta(x, p, q, log_beta))
        }
    });
}