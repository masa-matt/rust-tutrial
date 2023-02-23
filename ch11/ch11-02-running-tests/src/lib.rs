fn prints_and_returns_10(a: i32) -> i32 {
    println!("I got the value {}", a);
    10
}

// test with single thread
// cargo test -- --test-threads=1

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn this_test_will_pass() {
        let value = prints_and_returns_10(4);
        assert_eq!(10, value);
    }

    #[test]
    fn this_test_will_fail() {
        let value = prints_and_returns_10(8);
        assert_eq!(5, value);
    }

    #[ignore]
    fn expensive_test() {
        // code that takes an hour to run
    }
}

// to show log always
// cargo test -- --nocapture

// test only has ignore tag
// cargo test -- --ignored
