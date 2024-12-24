fn main() {
    println!("Hello, world!");

    another_function();
    another_function_1(5);
    print_labeled_measurement(5, 'm');
    let x = five();
    println!("The value of five is: {x}");

    let x = plus_one(5);
    println!("The value of x is: {x}");
}

fn five() -> i32 {
    5 // NO SEMICOLON!!
}

fn plus_one(x: i32) -> i32 {
    x + 1 // NO SEMICOLON!!
}

fn another_function() {
    println!("Another function.");
}

fn another_function_1(x: i32) {
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}