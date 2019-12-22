use std::io;

fn check_num(num: i32) {
    if num < 0 {
        println!("{} is negative", num);
    } else if num > 0 {
        println!("{} is positive", num);
    } else {
        println!("Number equals to zero");
    }
}

fn main() {
    let mut number: i32;
    println!("Please enter a number:");
    loop {
        let mut flag = false;
        let mut temp_num = String::new();
        io::stdin().read_line(&mut temp_num).expect("Failed to read number input");
        number = match temp_num.trim().parse::<i32>() {
            Ok(num) => {
                flag = true;
                num
            },
            Err(_) => {
                println!("Please enter a valid number");
                0
            }
        };
        if flag == true {
            break;
        }
    }

    check_num(number);
}
