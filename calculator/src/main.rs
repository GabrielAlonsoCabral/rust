use std::env::{args, Args};

fn main() {
    let  mut args:Args= args();
    println!("Total of args received: {:#?}", args);

    println!("Args before nth: {:?}",args);

    let first_arg:String = args.nth(1).unwrap();
    println!("Args after nth(1): {:?}",args);

    let operator:char = args.nth(0).unwrap().chars().next().unwrap();
    let second_arg:String = args.nth(0).unwrap();
    println!("{:?} {:?} {:?}",first_arg, operator, second_arg);

    
    let first_number:f32 = first_arg.parse().unwrap();
    let second_number= second_arg.parse::<f32>().unwrap();

    println!("{:?} {:?} {:?}",first_number, operator, second_number);

    let result = operate(operator, first_number, second_number);

    println!("{}", output(first_number, operator, second_number, result))    
}

fn operate(operator: char, first_number:f32, second_number:f32) -> f32{
    match operator {
        '+' => first_number + second_number,
        '-' => first_number - second_number,
        '^' => f32::powf(first_number, second_number),
        '*' | 'x' | 'X' => first_number * second_number,
        '/' => first_number / second_number,
        _ => panic!("Invalid operator used.")
    }
}

fn output(first_number:f32, operator: char, second_number:f32, result:f32)->String{
    format!("{} {} {} = {}", first_number, operator, second_number, result)
}
