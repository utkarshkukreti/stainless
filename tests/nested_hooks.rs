#![feature(phase)]
#[phase(plugin, link)]
extern crate stainless;

stainless! top_level {
    before_each {
        let mut foo = 1u;
    }

    after_each {
        assert_eq!(foo, 4);
    }

    describe nested {
        before_each {
            assert_eq!(foo, 1);
            foo += 1;
        }

        it "should be more specific" {
            assert_eq!(foo, 2);
            foo += 1;
        }

        after_each {
            assert_eq!(foo, 3);
            foo += 1;
        }
    }
}
