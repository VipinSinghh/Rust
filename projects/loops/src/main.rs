fn main() {
    let mut counter = 0;
    'countering : loop {
        println!("the counter : {counter}");

        let mut remaining = 10;
        loop {
            if remaining < 4 {
                break;
            }

            println!("the remaining : {remaining}");
            if counter == 5 {
                break 'countering;
            }
            remaining -= 1;


        }
        counter += 1;
    }
    println!("counter at end : {counter}");
}
