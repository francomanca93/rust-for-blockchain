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

fn conditionals(){
    // Take the age from the terminal
    println!("Please enter your age: ");
    let mut age: String = String::new();
    std::io::stdin().read_line(&mut age).unwrap();
    // Turn into that age to an integer
    let age_int: u8 = age.trim().parse().unwrap();
    
    // Conditional if/else
    if age_int >= 18 {
        println!("You can enter the club")
    } else {
        println!("You cannot enter the club")
    }
}

fn matrix_conditionals(){
    println!("You must choose beetwen these two pill");
    println!("1. red 🔴");
    println!("2. blue 🔵");
    
    let mut pill: String = String::new();
    std::io::stdin().read_line(&mut pill).unwrap();
    pill = pill.trim().to_string();

    if pill == "red" || pill == "1" {
        println!("You take the red pill... you stay in Wonderland, and I show you how deep the rabbit hole goes")
    } else if pill == "blue" || pill == "2" {
        println!("You take the blue pill... the story ends, you wake up in your bed and believe whatever you want to believe")
    } else {
        println!("BOOM!")
    }
}

fn ciclo_loop(){
    let number_1: i32 = 123;
    let number_2: i32 = 321;

    let sum: i32 = number_1 + number_2;

    loop {
        println!("Please enter the sum of {} and {}: ", number_1, number_2);
        
        let mut user_sum:String = String::new();
        std::io::stdin().read_line(&mut user_sum).unwrap();
    
        let user_sum_int: i32 = user_sum.trim().parse().unwrap();
    
        if user_sum_int == sum {
            println!("You're right, the answer is {}.", sum);
            break;
        } else {
            println!("You're wrong, your answer was {}", user_sum_int);
        }
    }

}

fn main() {
    //take_data_from_user()
    //conditionals()
    //matrix_conditionals()
    ciclo_loop()
}
