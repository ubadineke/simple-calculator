use dialoguer::{Select, Input, theme::ColorfulTheme};
mod operations;

use operations::Inputs;


fn main() {
    let options = vec![ "Addition", "Subtraction", "Multiplication", "Division" ];
    println!("MINI-CALCULATOR");
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Choose an option")
        .items(&options)
        .default(0)
        .interact()
        .unwrap();

    // println!("You selected: {}", options[selection]);

    //TAKE IN ARGUMENTS FOR CALCULATION 

    let inputs = Inputs{
        number1: get_input("Enter first number"),
        number2: get_input("Enter second number")
    };
    

    let result = match options[selection]{
        "Addition" => inputs.add(),
        "Subtraction" => inputs.subtract(),
        "Multiplication"=> inputs.multiply(),
        "Division"=> inputs.divide(),
        _ => None,
    };


    println!("The result is: {}", result.unwrap_or(0));



}



//addition




//subtraction
//multiplication
//divisions


// fn contains_only_integers(s: &str) -> bool{
//     s.parse::<i32>().is_ok()
// }

fn get_input(prompt: &str) -> i32{
loop{
    let input = Input::<String>::new()
        .with_prompt(prompt)
        .interact_text()
        .unwrap();

        if let Ok(number) = input.parse::<i32>() {
            return number;
        } else {
            println!("Invalid input, please enter an integer.");
        }
    }
}
