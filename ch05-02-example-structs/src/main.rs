fn main() {
    area_of_rectangle();
    refactored_with_tuple_area_of_rectangle();
    refactored_with_struct_area_of_rectable();
}

fn area_of_rectangle() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn refactored_with_tuple_area_of_rectangle() {
    let rect1 = (30, 50);

    println!(
        "The area of the rectangle is {} square pixels.",
        refactored_with_tuple_area(rect1)
    );
}

fn refactored_with_tuple_area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn refactored_with_struct_area_of_rectable() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    // error[E0277]: `Rectangle` doesn't implement `std::fmt::Display`
    // = help: the trait `std::fmt::Display` is not implemented for `Rectangle`
    // = note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead
    // = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)
    println!("rect1 is {:?}", rect1);
    println!("rect1 is {:#?}", rect1);

    println!(
        "The area of the rectangle is {} square pixels.",
        refactored_with_struct_area(&rect1)
    );
}

fn refactored_with_struct_area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
