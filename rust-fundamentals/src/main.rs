fn take_data_from_user(){
    // Use trim() method to delete spaces, tab or line break
    
    // Take the age from the terminal
    println!("Please enter your name: ");
    let mut name: String = String::new();
    std::io::stdin().read_line(&mut name).unwrap();
    name = name.trim().to_string();
    
    // Take the age from the terminal
    println!("Please enter your age: ");
    let mut age: String = String::new();
    std::io::stdin().read_line(&mut age).unwrap();
    // Turn into that age to an integer
    let age_int: u8 = age.trim().parse().unwrap();
    
    println!("Hello, welcome {}, I know that you're {} years old", name, age_int);

}

fn main() {
    take_data_from_user()
}
