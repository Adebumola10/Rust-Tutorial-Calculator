//A tutorial calculator built using Rust programming.

fn main() {
    println!("Welcome To Rust Tutorial Calculator.");

    let first_calculation:f32 = 50.6;
    let second_calculation: f32 = 30.6;
    let _result: f32 = first_calculation / second_calculation;
    
    let _type_casted_first_calcualtion = first_calculation as i32;
    let _type_casted_second_calculation = second_calculation as i32;
    let _result_type_casted = _result as i32;

    println!("Result = {}", _result);
    println!("Type_Casted_Result = {}", _result_type_casted);

}