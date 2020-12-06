mod integer_list_stats;
mod pig_latin;
mod departments_control;
mod read_from_input;

use read_from_input::read_string;
use pig_latin::to_pig_latin;
use integer_list_stats::print_all_stats;
use departments_control::Company;

fn main() {
    
    //let vec = vec![1, 2, 7, 7, 8];

    let vec = read_vec_from_input();

    print_all_stats(vec);

    let text = read_string("Enter string line: ");
    println!("{}", to_pig_latin(&text));

    Company::company_cli();
}

fn read_vec_from_input() -> Vec<i32>{
    convert_string_to_ints(
        read_string("Enter intergers seperated by space: ")
    )
}

fn convert_string_to_ints(string: String) -> Vec<i32> {
    let mut nums = vec![];
    for word in string.split_whitespace() {
        match word.parse::<i32>() {
            Ok(num) => nums.push(num),
            Err(err) => println!("Error: {}", err),
        };
    }
    nums
}

