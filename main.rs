//A tutorial calculator built using Rust programming.

use std::io; //importing the input and output libary.

fn main () {
    println!("Welcome to Rust Calculator");

    let mut _first_prompt = String::new();

    println!("Enter first number:");

    io::stdin().read_line(&mut _first_prompt).expect("Failed to read line");

    let _first_prompt_number: f32 = _first_prompt.trim().parse().expect("Please input a number");

    let mut _operator_calculating = String::new();

    println!("Enter operator sign(+, -, /, * , %):");

    io::stdin().read_line(&mut _operator_calculating).expect("Failed to read line");

    let _operator_calculation: String = _operator_calculating.trim().parse().expect("Please enter operator:");

    let mut _second_prompt =String::new();

    println!("Enter second number:");

    io::stdin().read_line(&mut _second_prompt).expect("Failed to read line");
    
    let _second_prompt_number: f32 = _second_prompt.trim().parse().expect("Please enter a number:");

    let _result_cal:f32 = match & _operator_calculation as &str {
        "+" => _first_prompt_number + _second_prompt_number,
        "-" => _first_prompt_number - _second_prompt_number,
        "*" => _first_prompt_number * _second_prompt_number,
        "%" => _first_prompt_number % _second_prompt_number,
        "/" => {
            if _second_prompt_number != 0.0 {
                _first_prompt_number / _second_prompt_number
            } else {
                println!("Error: E");
                return;
            }
        }
        _ => {
            println!("Error: Inavlid operator. please use +, -, /, *, %");
            return;
        }
    };

    println!("Ans = {}", _result_cal);
}