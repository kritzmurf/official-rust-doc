

fn main() {
    //println!("{0}", accept_input());
    println!("{0}", convert_to_celsius(accept_input()));
}

fn accept_input() -> i32 {
    let mut input = String::new();
    loop {
        println!("Please enter a value to convert from Fahrenheit to Celsius");

        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let parsed_input: i32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please input an integer");
                input.clear();
                continue;
            },
        };
        break parsed_input
    }
}

fn convert_to_celsius(temp : i32) -> i32 {
    ((temp - 32) * 5) / 9
}
