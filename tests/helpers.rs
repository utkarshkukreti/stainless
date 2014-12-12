#![feature(phase)]
#[phase(plugin, link)]
extern crate stainless;

#[cfg(test)]
mod test {
    pub fn test_helper<T: PartialEq>(x: T, y: T) {
        if x != y { panic!("Not equal.") }
    }

    stainless! helpers {
        it "should be able to use helpers" {
            test_helper(7u, 7);
        }
    }
}

