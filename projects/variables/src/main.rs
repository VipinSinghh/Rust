fn main() {
    println!("Hello, world!");
    let mut x  = 5;

    println!("the value of x is {x}");

    x = 6;

    println!("the value of x is {x}");

    let y = 5;

    let y = y + 1;

    {
        let y = y * 2;
        println!("The value of y in the inner scope is: {y}");
    }

    println!("The value of y is: {y}");
    main2();
}

fn main2() {
    println!("yooooo");
    let a = {
        let b = 1;
        b+1
    };
    println!("the value of a : {a}");
}

// fn main() {
//     let x = plus_one(6);

//     println!("The value of x is: {x}");
// }

// fn plus_one(x: i32) -> i32 {
//     x + 1
// }