fn main() {
    println!("Hello, world!");

    let x = 6;

    if x<5 {
        println!("the condition is true");
    } else {
        println!("the condition is false");
    }

    let z = 5;
    let number = if z == 5 {1} else {0};
    println!("the value of number is : {number}");
}
