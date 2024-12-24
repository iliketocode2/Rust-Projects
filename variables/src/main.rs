fn main() {
    let x = 5;
    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    let y: (i32, f64, u8) = (500, 6.4, 1);
    let one = y.0;
    let two = y.1;
    let three = y.2;

    println!("The value of one is: {one}\nThe value of two is: {two}\nThe value of three is: {three}");

    let a = [1, 2, 3, 4, 5];
    let first = a[0];
    let second = a[1];

    println!("The value of first is: {first}\nThe value of second is: {second}");
}