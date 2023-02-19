fn main() {
    dangling();
    return_longest();
    different_lifetime();
    struct_with_lifetime();
}

fn dangling() {
    {
        let r;

        {
            let x = 5;
            // error[E0597]: `x` does not live long enough
            // r = &x;
            r = x;
        }

        println!("r: {}", r);
    }
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// &i32        // a reference
// &'a i32     // a reference with an explicit lifetime
// &'a mut i32 // a mutable reference with an explicit lifetime

fn return_longest() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}

fn different_lifetime() {
    let string1 = String::from("long string is long");

    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }
    // error[E0425]: cannot find value `result` in this scope
    // println!("The longest string is {}", result);
}

// In this case, y doesn't need lifetime parameter
// fn longest<'a>(x: &'a str, y: &str) -> &'a str {
//     x
// }

// fn longest<'a>(x: &str, y: &str) -> &'a str {
//     let result = String::from("really long string");
//     // error[E0515]: cannot return reference to local variable `result`
//     result.as_str()
// }

struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn struct_with_lifetime() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
}

// without lifetime annotations
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

// Let’s pretend we’re the compiler.
// fn first_word(s: &str) -> &str {

// the compiler applies the first rule, which specifies that each parameter gets its own lifetime.
// fn first_word<'a>(s: &'a str) -> &str {

// The second rule applies because there is exactly one input lifetime.
// The second rule specifies that the lifetime of the one input parameter gets assigned to the output lifetime.
// fn first_word<'a>(s: &'a str) -> &'a str {

impl<'a> ImportantExcerpt<'a> {
    // no need to speficy, because of second rule.
    fn level(&self) -> i32 {
        3
    }
}

// the third lifetime elision rule applies
impl<'a> ImportantExcerpt<'a> {
    // There are two input lifetimes, so Rust applies the first lifetime elision rule and gives both &self and announcement their own lifetimes.
    // Then, because one of the parameters is &self, the return type gets the lifetime of &self, and all lifetimes have been accounted for.
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

fn static_lifetime() {
    let s: &'static str = "I have a static lifetime.";
}

fn generics_type_parameters_trait_bounds_liftimes() {
    use std::fmt::Display;

    fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
    where
        T: Display,
    {
        println!("Announcement! {}", ann);
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }
}
