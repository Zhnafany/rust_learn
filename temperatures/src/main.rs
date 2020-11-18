use std::io;

fn main() {

    let temp: i32 = loop {  
        println!("Enter temperature value(Fahrenheit and Celsius):");

        let mut temp = String::new();

        io::stdin().read_line(&mut temp).expect("Failed to read line!"); // read_line append to buffer temp

        match temp.trim().parse() { 
            Ok(num) => { break num }, // break return num from loop
            Err(_) => continue,
        };
    };

    let celsium: f32 = (temp as f32 - 32.0) / 1.8; // was 1.8 but 

    let fahrenheit: f32 = temp as f32 * 1.8 + 32.0; // converst result of multiplaing to i32

    println!("If you enter temperature in fahrenheit, than you got: {} C", celsium);

    println!("If you enter temperature in celsius, than you got: {} F", fahrenheit);
}
