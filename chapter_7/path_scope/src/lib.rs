mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

// Use the hosting module instead of the add_to_waitlist function is a rust idiom.
// This helps to indicate tha the function isn't locally defined in the current module.
// It is idiomatic to use full paths for structs, enums and other items not defined in the current
// module.
pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}

use std::fmt::Result;
use std::io::Result as IoResult;

fn function1() -> Result {
    todo!("");
}

fn function2() -> IoResult<u8> {
    todo!("");
}

use std::{cmp::Ordering, error};

use std::io::{self, Write};

use std::collections::*;
