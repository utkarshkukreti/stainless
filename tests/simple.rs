#![feature(phase)]
#[phase(plugin, link)]
extern crate stainless;

stainless! addition {
    before_each {
        let x = 5u; let y = 6u;
    }

    it "should add 5 and 6 together" {
        assert_eq!(x + y, 11);
    }

    after_each {
        assert_eq!(x, 5);
        assert_eq!(y, 6);
    }
}

