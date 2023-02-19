fn main() {
    finding_largest();
    // finding_largest_generics();
    struct_with_generics();
    method_with_generics();
    mixed_up();
}

fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn finding_largest() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest_i32(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest_char(&char_list);
    println!("The largest char is {}", result);
}

// fn largest<T>(list: &[T]) -> T {
//     let mut largest = list[0];

//     for &item in list.iter() {
//         // error[E0369]: binary operation `>` cannot be applied to type `T`
//         // help: consider restricting type parameter `T`
//         if item > largest {
//             largest = item;
//         }
//     }

//     largest
// }

// fn finding_largest_generics() {
//     let number_list = vec![34, 50, 25, 100, 65];

//     let result = largest(&number_list);
//     println!("The largest number is {}", result);

//     let char_list = vec!['y', 'm', 'a', 'q'];

//     let result = largest(&char_list);
//     println!("The largest char is {}", result);
// }

struct Point<T, U> {
    x: T,
    // y: T,
    y: U,
}

fn struct_with_generics() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };

    // If Both generics is T, then compile error.
    // expected integer, found floating-point number
    let wont_work = Point { x: 5, y: 4.0 };
}

impl<T, U> Point<T, U> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f64, f64> {
    fn distance_from_origin(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn method_with_generics() {
    let p = Point { x: 5, y: 10 };

    println!("p.x = {}", p.x());

    let p = Point { x: 1.0, y: 4.0 };
    println!("distance = {}", p.distance_from_origin());
}

impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn mixed_up() {
    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}

// The good news is that using generic types won't make your program run any slower than it would with concrete types!!!
// Rust accomplishes this by performing monomorphization of the code using generics at compile time.
// Monomorphization is the process of turning generic code into specific code by filling in the concrete types that are used when compiled.

fn monomorphized_code() {
    enum Option_i32 {
        Some(i32),
        None,
    }

    enum Option_f64 {
        Some(f64),
        None,
    }

    fn main() {
        let integer = Option_i32::Some(5);
        let float = Option_f64::Some(5.0);
    }
}
