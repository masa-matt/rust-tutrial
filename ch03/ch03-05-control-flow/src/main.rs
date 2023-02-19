use ::rug::Integer;
use std::{
    ops::{Div, Sub},
    vec,
};

fn main() {
    if_expressions();
    if_multiple_conditions();
    if_in_a_let_statement();

    infinite_loop();
    loop_with_return_values();
    loop_with_label();
    while_loop();
    for_alternative_while();

    // Exercises
    // Convert temperatures between Fahrenheit and Celsius.
    convert_fahrenheit_celsius();
    // Generate the nth Fibonacci number.
    fibonacci();
    // Print the lyrics to the Christmas carol “The Twelve Days of Christmas,” taking advantage of the repetition in the song.
}

fn if_expressions() {
    // let number = 3;
    let number = 7;

    // compile error
    // if number {
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    let number = 3;

    if number != 0 {
        println!("number was something other than zero");
    }
}

fn if_multiple_conditions() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}

fn if_in_a_let_statement() {
    let condition = false;
    let number = if condition { 5 } else { 6 };
    // compile error
    // let number = if condition { 5 } else { "six" };

    println!("The value of number is: {number}");
}

fn infinite_loop() {
    // loop {
    // println!("again!");
    // }
}

fn loop_with_return_values() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
}

fn loop_with_label() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}

fn while_loop() {
    let mut number = 3;

    while number != 0 {
        println!("number = {number}");

        number -= 1;
    }

    println!("LIFTOFF!!!");
}

fn for_alternative_while() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    // this approach is error prone
    while index < 5 {
        println!("The value is: {}", a[index]);

        index += 1;
    }

    for element in a {
        println!("The value is: {}", element);
    }

    // count down
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}

fn convert_fahrenheit_celsius() {
    let fahrenheit = 10.5;
    let celsius = fahrenheit_to_celsius(fahrenheit);
    println!("The temperatures is: {fahrenheit}°F({celsius}°C)");
}

fn fahrenheit_to_celsius(x: f64) -> f64 {
    x.sub(32.0).div(1.8000)
}

fn fibonacci() {
    // 0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55
    let n = 10;
    let result = fib(n);
    println!("Fibonacci number. If n is {n}, result is {result}");
}

fn fib(n: usize) -> Integer {
    let mut f: Vec<Integer> = vec![Integer::from(0), Integer::from(1)];

    for i in 2..(n + 1) {
        f.push(f[i - 1].clone() + f[i - 2].clone());
    }

    f[n].clone()
}
