use std::io;

fn main() {
    let max = get_number("Enter maximal length to show:");
    print_fibonacci(max as u32);
}

fn get_number(message: &str) -> i32 {
    loop {
        println!("{}", message);

        let mut number = String::new();

        io::stdin()
            .read_line(&mut number)
            .expect("Failed to read line");

        match number.trim().parse() { 
            Ok(num) => { break num },
            Err(_) => continue,
        };
    }
}

fn print_fibonacci(max: u32) {
    let mut last: u32 = 0;
    let mut current: u32 = 1;

    print!("{} {} ", last, current);
    let mut counter = 0;

    while counter < max - 2 {
        let next_last: u32 = current; 
        
        if last < (u32::MAX - current) {  
            current = last + current;
        } else {
            print!("...");
            break;
        }
        last = next_last;
        print!("{} ", current);

        counter += 1;
    } 
}
