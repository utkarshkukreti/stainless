#![feature(phase)]
#[phase(plugin, link)]
extern crate stainless;
extern crate test;

stainless! benchmarking {
    bench "should benchmark" (bencher) {
            bencher.iter(|| 2u * 2)
    }
}

