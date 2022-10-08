fn main() {
    let age: i16 = 24;
    let name: &str = "Peter Parker";
    let mut year: u16 = 2022;

    year += 1;

    println!("Hello, world!");
    println!("My age is {} and my name is {}", age, name);
    println!("Next year will be {}", year);
}
