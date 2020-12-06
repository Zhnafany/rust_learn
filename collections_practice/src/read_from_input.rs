use std::io::stdin;

pub fn read_string(message: &str) -> String {
    let mut input = String::new();
    println!("{}", message);

    match stdin().read_line(&mut input) {
        Err(error) => {
            println!("Error: {}", error);
            "".into()
        },
        _ => {input}
    }
}
