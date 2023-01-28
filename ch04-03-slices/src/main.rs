fn main() {
    let a = first_word(&String::from("a"));
    let b = first_word(&String::from("test"));
    let c = first_word(&String::from("cc c"));
    let d = first_word(&String::from(" d"));

    println!("{a} {b} {c} {d}");

    let mut s = String::from("hello world");

    let word = first_word(&s); // word will get the value 5

    s.clear(); // this empties the String, making it equal to ""

    // word still has the value 5 here, but there's no more string that
    // we could meaningfully use the value 5 with. word is now totally invalid!
    println!("{s} {word}");

    string_slices();

    let mut s = String::from("hello world");

    let word = first_word_return_string_slices(&s);

    // cannot borrow `s` as mutable because it is also borrowed as immutable
    // s.clear();

    println!("The first word of '{s}' is: {word}");

    let s = "Hello, world!";
    // s is &str. it is a slice, means immutable

    let my_string = String::from("hello world");

    // `first_word` works on slices of `String`s, whether partial or whole
    let word = string_slices_as_parameters(&my_string[0..6]);
    let word = string_slices_as_parameters(&my_string[..]);
    // `first_word` also works on references to `String`s, which are equivalent
    // to whole slices of `String`s
    let word = first_word(&my_string);

    let my_string_literal = "hello world";

    // `first_word` works on slices of string literals, whether partial or whole
    let word = string_slices_as_parameters(&my_string_literal[0..6]);
    let word = string_slices_as_parameters(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = string_slices_as_parameters(my_string_literal);
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn string_slices() {
    let s = String::from("hello");

    // this are equal
    let slice = &s[0..2];
    let slice = &s[..2];
    println!("{slice}");

    let len = s.len();

    // this are equal
    let slice = &s[3..len];
    let slice = &s[3..];
    println!("{slice}");

    // this are equal
    let slice = &s[0..len];
    let slice = &s[..];
    println!("{slice}");
}

fn first_word_return_string_slices(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn string_slices_as_parameters(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
