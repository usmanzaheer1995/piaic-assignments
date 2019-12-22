use std::io::stdin;

fn number_and_square(num: i32) -> (i32, i32) {
    (num, num * num)
}

fn main() {
    let mut number: i32;
    println!("Enter number:");
    loop {
        let mut flag = false;
        let mut temp_number = String::new();
        stdin().read_line(&mut temp_number).expect("Failed to read number input");
        number = match temp_number.trim().parse::<i32>() {
            Ok(num) => {
                flag = true;
                num
            },
            Err(_) => {
                println!("Please enter a valid number value");
                0
            }
        };
        if flag == true {
            break;
        }
    }

    let (num, square) = number_and_square(number);
    println!("Entered number: {}", num);
    println!("Square of entered number: {}", square);
}
