fn main() {
    reference_and_borrowing();
    borrowed_value_cannot_change();
    mutable_reference();
    mutable_ristriction();
    dangling_reference();
}

fn reference_and_borrowing() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, it is not dropped.

fn borrowed_value_cannot_change() {
    let s = String::from("hello");

    change(&s);
}

fn change(some_string: &String) {
    // some_string.push_str(", world");
}

fn mutable_reference() {
    let mut s = String::from("hello");

    change_mut(&mut s);
}

fn change_mut(some_string: &mut String) {
    some_string.push_str(", world");
}

fn mutable_ristriction() {
    let mut s = String::from("hello");

    let r1 = &mut s;
    // let r2 = &mut s; // cannot borrow `s` as mutable more than once at a time

    {
        let r1 = &mut s;
    } // r1 goes out of scope here, so we can make a new reference with no problems.

    let r2 = &mut s;

    let mut s2 = String::from("hello");

    let r1 = &s2; // no problem
    let r2 = &s2; // no problem

    // let r3 = &mut s2; // BIG PROBLEM
    // We also cannot have a mutable reference while we have an immutable one to the same value.
}

fn dangling_reference() {
    let reference_to_nothing = dangle();
}

fn dangle() -> &String {
    // dangle returns a reference to a String

    let s = String::from("hello"); // s is a new String

    &s // we return a reference to the String, s
} // Here, s goes out of scope, and is dropped. Its memory goes away.
  // Danger!
