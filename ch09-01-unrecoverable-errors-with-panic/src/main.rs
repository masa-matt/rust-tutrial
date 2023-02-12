fn main() {
    // panic();
    panic_with_library();
}

fn panic() {
    panic!("crash and burn");
}

fn panic_with_library() {
    let v = vec![1, 2, 3];
    // This is called a buffer overread and can lead to security vulnerabilities if an attacker is able to manipulate the index in such a way as to read data they shouldn’t be allowed to that is stored after the data structure.
    v[99];
}
