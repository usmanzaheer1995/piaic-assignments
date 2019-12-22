use std::io;

fn main() {
    let mut height: f32;
    let mut weight: f32;

    println!("Enter your height (cm)");

    // Make sure the input is a valid f32 value
    loop {
        let mut flag = false;
        let mut temp_height = String::new();
        io::stdin().read_line(&mut temp_height).expect("Failed to read height input");
        height = match temp_height.trim().parse::<f32>() {
            Ok(num) => {
                flag = true;
                num
            },
            Err(_) => {
                println!("Please enter a valid height value");
                0.0
            }
        };
        if flag == true {
            break;
        }
    }

    println!("Enter your weight (kg)");

    // Make sure the input is a valid f32 value
    loop {
        let mut flag = false;
        let mut temp_weight = String::new();
        io::stdin().read_line(&mut temp_weight).expect("Failed to read weight input");
        weight = match temp_weight.trim().parse::<f32>() {
            Ok(num) => {
                flag = true;
                num
            },
            Err(_) => {
                println!("Please enter a valid weight value");
                0.0
            }
        };
        if flag == true {
            break;
        }
    }

    // Formula for BMI calculation taken from:
    // https://www.thecalculatorsite.com/articles/health/bmi-formula-for-bmi-calculations.php
    let bmi = weight / ((height/100.0) * (height/100.0));

    if bmi < 18.5 {
        println!("Your BMI is {:.1}, which means you're underweight.", bmi);
    } else if bmi > 18.5 && bmi < 25.0 {
        println!("Your BMI is {:.1}, which means you're normal weight.", bmi);        
    } else {
        println!("Your BMI is {:.1}, which means you're overweight.", bmi);        
    }
}
