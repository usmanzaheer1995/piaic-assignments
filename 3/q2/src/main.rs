fn main() {
    let mut index = 1;
    while index <= 10 {
        if index == 3 || index == 7 || index == 10 {
            println!("{}. Special Security check", index);
        } else {
            println!("{}", index);
        }
        index += 1;
    }
}
