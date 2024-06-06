//NOTE: this will overflow with a high enough input. Should add checker

fn main() {
    println!("{0}", fib_calc(accept_input()));

}


fn accept_input() -> i32 {
    let mut input = String::new();
    loop {
        println!("Please enter which fibonacci number to print");

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

fn fib_calc(nth : i32) -> i32 {
    let mut prev: i32 = 0;
    let mut sum: i32 = 1;

    for _ in 0..nth {
        let temp: i32 = sum;
        sum += prev;
        prev = temp;
    }
    sum
}
