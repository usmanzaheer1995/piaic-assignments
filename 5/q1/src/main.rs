use std::io::stdin;

fn get_user_name() -> String {
    let mut name = String::new();
    println!("PLease enter your name");
    stdin().read_line(&mut name).expect("Failed to read input");
    name
}

fn get_marks() -> u32 {
    let mut marks: u32;
    loop {
        let mut flag = false;
        let mut temp_marks = String::new();
        stdin().read_line(&mut temp_marks).expect("Failed to read input");
        marks = match temp_marks.trim().parse::<u32>() {
            Ok(num) => {
                if num <= 100 {
                    flag = true;
                } else {
                    println!("Please enter a value less than equal to 100");
                }
                num
            },
            Err(_) => {
                println!("Please enter a valid marks value");
                0
            }
        };
        if flag == true {
            break;
        }
    }
    marks
}

fn calculate_percentage(name: String, subject_one_marks: u32, subject_two_marks: u32) -> f64 {
    let percentage = ((subject_one_marks as f64 + subject_two_marks as f64) / 200.0) * 100.0;
    println!("Percentage of {0} is {1}", name, percentage);
    percentage
}

fn main() {
    let name = get_user_name();
    println!("Please enter marks for subject 1");
    let subject_one_marks = get_marks();
    println!("Please enter marks for subject 2");
    let subject_two_marks = get_marks();

    let percentage = calculate_percentage(name, subject_one_marks, subject_two_marks);

    if percentage >= 70.0 {
        println!("Pass");
    } else {
        println!("Fail");
    }
}
