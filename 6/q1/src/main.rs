#[derive(Debug)]

// 1. define a custom datatype using Struct
struct Person {
    name: String,
    age: u8,
    email: String,
}

// 3. define user defined function
fn return_instance(name: &str, age: u8, email: &str) -> Person {
    Person {
        name: String::from(name),
        age,
        email: String::from(email),
    }
}

fn main() {
    // 2A. Create an instance of the above defined struct
    let person1 = Person {
        name: String::from("Usman Zaheer"),
        age: 24,
        email: String::from("usmanzaheer1995@gmail.com"),
    };

    // 2B. Print complete instance
    println!("Person 1: {:?}", person1);

    // 2C. Print instance by calling its fields.
    println!("\nPerson name: {0}\nPerson age: {1}\nPerson email: {2}", person1.name, person1.age, person1.email);

    let person2 = Person {
        ..person1
    };

    // 2D. create another instance by using fields of first instance
    println!("\nPerson 2: {:?}", person2);

    // 4. in main function call user defined function
    let person3 = return_instance("Ahmad Zaheer", 27, "ahmadzaheer@gmail.com");
    println!("\nPerson 3: {:?}", person3);
}
