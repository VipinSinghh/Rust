#[derive(Debug)]
struct Rectangle {
    width : u32,
    height : u32,
}

fn main() {
    let width1 = 30;
    let height1 = 50;

    let rec = (30,50);

    let rect = Rectangle {
        width : 30,
        height : 50,
    };
    println!("the value of rect: {:?}", rect);
    dbg!(&rect);
    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );
    println!(
        "The area of the rectangle is {} square pixels.",
        area1(rec)
    );
    println!(
        "The area of the rectangle is {} square pixels.",
        area2(&rect)
    );
}

fn area(width: u32, height:u32) -> u32 {
    width*height
}

fn area1(dimensions:(u32,u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area2(rectangle:&Rectangle) -> u32{
    rectangle.width * rectangle.height
}
