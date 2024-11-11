use dialoguer::{Select, Input, theme::ColorfulTheme};

fn main() {
    let options = vec![ "Addition", "Subtraction", "Multiplication", "Division" ];
    println!("MINI-CALCULATOR");
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Choose an option")
        .items(&options)
        .default(0)
        .interact()
        .unwrap();

    println!("You selected: {}", options[selection]);

    let num1 = Input::<String>::new()
        .with_prompt("Enter first number")
        .interact_text()
        .unwrap();

    //check if input is all numbers if not repeat the prompt
    let name2: String = Input::new()
        .with_prompt("Enter second number")
        .interact_text()
        .unwrap();

        //check if input is all numbers if not repeat the prompt
    println!("The~ numbers are: {} {}",name, name2);

}



//addition




//subtraction
//multiplication
//divisions