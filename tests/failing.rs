#![feature(phase)]
#[phase(plugin, link)]
extern crate stainless;

stainless! failing {
    failing "should fail" {
        panic!("should still pass");
    }
}

