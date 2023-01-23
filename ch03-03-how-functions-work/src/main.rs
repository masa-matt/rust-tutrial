fn main() {
    println!("Hello, world!");

    another_function();
    another_function_with_arg(5);
    print_labeled_measurement(5, 'h');
    statements_expression();
    functions_with_return_values();
}

fn another_function() {
    println!("Another function.");
}

fn another_function_with_arg(x: i32) {
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn statements_expression() {
    // compile error
    // let x = (let y = 6);

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    // compile error
    // x + 1;
    x + 1
}

fn functions_with_return_values() {
    let x = five();
    println!("The value of x is: {x}");

    let y = plus_one(x);
    println!("The value of y is: {y}");
}
