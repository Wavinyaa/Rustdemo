use std::io;

fn main() {
    println!("Welcome to Mitchelle Calc");
    println!("Enter an operator (+, -, *, /) and two numbers:");   // displaying introduction text to users

    let mut input = String::new();

    io::stdin().read_line(&mut input)
        .expect("Failed to read line");  //  Reading data from the command line 




    let input_vec: Vec<&str> = input.trim().split_whitespace().collect();


    // kindly note this data is read from array. Array starts from zero (0) as first number
    let operator = input_vec[0];  //this is the first input
    let num1 = input_vec[1].parse::<f64>().unwrap();  //this is the second input
    let num2 = input_vec[2].parse::<f64>().unwrap();  //this is the third input 

    let result = match operator {
        "+" => num1 + num2,
        "-" => num1 - num2,
        "*" => num1 * num2,
        "/" => num1 / num2,
        _ => {
            println!("Invalid operator. Only +, -, *, / are allowed."); // Proccessing the data from the the above intialized varriables
            return;
        },
    };

    println!("Result of : {}", result); //displaying the result to users
    
}


//end
