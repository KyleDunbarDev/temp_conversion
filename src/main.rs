use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} [temperature]", args[0]);
        println!("Enter exactly one argument (a temperature) for conversion.");
        return;
    } else if &args[1] == "help" {
        println!("Usage: {} [temperature]", args[0]);
        println!("Enter a temperature value in Celsius or Fahrenheit to convert.");
        return;
    }

    let str_query = &args[1];

    match str_query.parse::<f32>() {
        Ok(float_query) => {
            let cels_to_fahr: f32 = float_query * 9.0 / 5.0 + 32.0;
            let fahr_to_cels: f32 = (float_query - 32.0) / 1.8;
            println!("Celsius to Fahrenheit: {}", cels_to_fahr);
            println!("Fahrenheit to Celsius: {}", fahr_to_cels);
        }
        Err(_) => {
            println!("Could not parse argument to float: {}", str_query);
        }
    }
}
