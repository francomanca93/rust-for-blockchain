
use regex::Regex;


fn div(mut expression: String) -> String{
    // div
    let re_div = Regex::new(r"(\d+)\s?(/)\s?(\d+)").unwrap();
    
    loop{
        // Apply Operations
        let caps = re_div.captures(expression.as_str());
    
        if caps.is_none(){
            break;
        }
    
        let caps = caps.unwrap();
    
        let cap_expression = caps.get(0).unwrap().as_str();
        let left_value: i32 = caps.get(1).unwrap().as_str().parse().unwrap();
        let sign_value = caps.get(2).unwrap().as_str();
        let right_value: i32 = caps.get(3).unwrap().as_str().parse().unwrap();
        println!("{}", sign_value);
        println!("{}", cap_expression);
        let div_value: i32 = left_value / right_value;
    
        expression = expression.replace(cap_expression, &div_value.to_string());
        println!("{}", expression);
    }

    expression
}


fn mul(mut expression: String) -> String{
    // mul
    let re_mul = Regex::new(r"(\d+)\s?(\*)\s?(\d+)").unwrap();
    
    loop{
        // Apply Operations
        
    
        let caps = re_mul.captures(expression.as_str());
    
        if caps.is_none(){
            break;
        }
    
        let caps = caps.unwrap();
    
        let cap_expression = caps.get(0).unwrap().as_str();
        let left_value: i32 = caps.get(1).unwrap().as_str().parse().unwrap();
        let sign_value = caps.get(2).unwrap().as_str();
        let right_value: i32 = caps.get(3).unwrap().as_str().parse().unwrap();
        println!("{}", sign_value);
        println!("{}", cap_expression);
        let mul_value: i32 = left_value * right_value;
    
        expression = expression.replace(cap_expression, &mul_value.to_string());
        println!("{}", expression);
    }

    expression
}


fn add(mut expression: String) -> String{
    // add
    let re_add = Regex::new(r"(\d+)\s?(\+)\s?(\d+)").unwrap();
    
    loop{
        // Apply Operations
    
        let caps = re_add.captures(expression.as_str());
    
        if caps.is_none(){
            break;
        }
    
        let caps = caps.unwrap();
    
    
        let cap_expression = caps.get(0).unwrap().as_str();
        let left_value: i32 = caps.get(1).unwrap().as_str().parse().unwrap();
        let sign_value = caps.get(2).unwrap().as_str();
        let right_value: i32 = caps.get(3).unwrap().as_str().parse().unwrap();
        println!("{}", sign_value);
        println!("{}", cap_expression);
        let add_value: i32 = left_value + right_value;
    
        expression = expression.replace(cap_expression, &add_value.to_string());
        println!("{}", expression);
    
    }
    expression
}


fn sus(mut expression: String) -> String{
    //sustraction
    
    let re_sus = Regex::new(r"(\d+)\s?(\-)\s?(\d+)").unwrap();
    
    loop{
        // Apply Operations
        
        let caps = re_sus.captures(expression.as_str());
        
        if caps.is_none(){
            break;
        }
        
        let caps = caps.unwrap();
        
        
        let cap_expression = caps.get(0).unwrap().as_str();
        let left_value: i32 = caps.get(1).unwrap().as_str().parse().unwrap();
        let sign_value = caps.get(2).unwrap().as_str();
        let right_value: i32 = caps.get(3).unwrap().as_str().parse().unwrap();
        println!("{}", sign_value);
        println!("{}", cap_expression);
        let sus_value: i32 = left_value - right_value;
    
        expression = expression.replace(cap_expression, &sus_value.to_string());
        println!("{}", expression);
    
    }
    expression
}


fn main() {
    println!("Hello, world!");

    // Regex
    // (\d+) \s? \+ \s? (\d+)

    // Get user data
    println!("Please, enter your expression: ");
    let mut expression = String::new();
    std::io::stdin().read_line(&mut expression).unwrap();
    
    expression = div(expression);
    expression = mul(expression);
    expression = add(expression);
    expression = sus(expression);
    // Show results
    println!("Result: {}", expression);
}
