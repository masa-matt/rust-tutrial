mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

// like symbolic link
// use crate::front_of_house::hosting;
// relative
// use self::front_of_house::hosting;
// re-exporting
pub use crate::front_of_house::hosting;

// Why not?
// Specifying the parent module when calling the function makes it clear that
// the function isn’t locally defined while still minimizing repetition of the full path.
// The code in Listing 7-13 is unclear as to where add_to_waitlist is defined.
// use crate::front_of_house::hosting::add_to_waitlist;

// when bringing in structs, enums, and other items
// it’s idiomatic to specify the full path
use std::collections::HashMap;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();

    // add_to_waitlist();

    let mut map = HashMap::new();
    map.insert(1, 2);
}

mod customer {
    pub fn eat_at_restaurant() {
        // A use statement only applies in the scope it’s in
        // error[E0433]: failed to resolve: use of undeclared crate or module `hosting`
        // hosting::add_to_waitlist();
    }
}

// Bringing two types with the same name into the same scope requires using their parent modules.
use std::fmt;
use std::io;

fn function1() -> fmt::Result {
    // --snip--
    Ok(())
}

fn function2() -> io::Result<()> {
    // --snip--
    Ok(())
}

use std::fmt::Result;
use std::io::Result as IoResult;

fn function3() -> Result {
    // --snip--
    Ok(())
}

fn function4() -> IoResult<()> {
    // --snip--
    Ok(())
}

use std::{cmp::Ordering, io};

// use std::io;
// use std::io::Write;
// this is same
use std::io::{self, Write};

// The Glob Operator
use std::collections::*;
