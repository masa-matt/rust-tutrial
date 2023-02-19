fn main() {
    create_new_string();
    update_string();
    concatenation();
    indexing_into_string();
    slicing_string();
    iterating_over_string();
}

fn create_new_string() {
    // let mut s = String::new();

    let data = "initial contents";

    let s = data.to_string();

    // the method also works on a literal directly:
    let s = "initial contents".to_string();

    let s = String::from("initial contents");

    // all valid
    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שָׁלוֹם");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");
}

fn update_string() {
    let mut s = String::from("foo");
    s.push_str("bar");
    println!("{s}");

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {s2}");

    let mut s = String::from("lo");
    s.push('l');
    println!("{s}");
}

fn concatenation() {
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3;
    println!("{s}");
    // moved!!
    // println!("{s1}");
    println!("{s2}");
    println!("{s3}");

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("{s}");
    println!("{s1}");
    println!("{s2}");
    println!("{s3}");
}

fn indexing_into_string() {
    let s1 = String::from("hello");
    // let h = s1[0];
    // `String` cannot be indexed by `{integer}`
    // = help: the trait `Index<{integer}>` is not implemented for `String`
    // = help: the following other types implement trait `Index<Idx>`:

    // A String is a wrapper over a Vec<u8>
    let len = String::from("Hola").len();
    println!("len is {len}");
    let len = String::from("Здравствуйте").len();
    println!("len is {len}");

    // this actually compile error
    // the answer is not a character, but bytes
    // let hello = String::from("hello");
    // let answer = &hello[0];
}

fn slicing_string() {
    let hello = String::from("Здравствуйте");
    let s = &hello[0..4];
    println!("s is {s}");

    // compile error
    // let s = &hello[0..1];
    // thread 'main' panicked at 'byte index 1 is not a char boundary; it is inside 'З' (bytes 0..2) of `Здравствуйте`'
}

fn iterating_over_string() {
    for c in "नमस्ते".chars() {
        println!("{c}");
    }

    for b in "नमस्ते".bytes() {
        println!("{b}");
    }
}
