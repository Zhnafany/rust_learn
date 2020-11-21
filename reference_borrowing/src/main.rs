//At any given time, you can have either one mutable reference or any number of immutable references.
//References must always be valid.
fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);

    let mut s = String::from("hello");

    let r1 = &mut s;
    //let r2 = &mut s;                            //error only one mutable refference can be

/////////////////////////////////////////
    change(&mut s);

    let mut s = String::from("hello");

    {
        let r1 = &mut s;
    } // r1 goes out of scope here, so we can make a new reference with no problems.

    let r2 = &mut s;
/////////////////////////////////////////

    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    //let r3 = &mut s; // BIG PROBLEM //cant create mutable ref if immut exist

    println!("{}, {}, and {}", r1, r2, r3);
///////////////////////////////////////

    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // r1 and r2 are no longer used after this point

    let r3 = &mut s; // no problem
    println!("{}", r3);

    let reference_to_nothing = dangle();

/////////////////////////////////////////
    //word is immutabble referesnce and clear take muttable ref - so error
    let mut s = String::from("hello world");

    let word = first_word(&s);

    s.clear(); // error!

    println!("the first word is: {}", word);


/////////////////////////////////////////

    let my_string = String::from("hello world");

    // first_word works on slices of `String`s
    let word = first_word(&my_string[..]);

    let my_string_literal = "hello world";

    // first_word works on slices of string literals
    let word = first_word(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word(my_string_literal);

}

//slices
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];      //reference(slice) to part of string 
        }
    }

    &s[..]                        //slice to all string
}


fn dangle() -> &String { // dangle returns a reference to a String

    let s = String::from("hello"); // s is a new String

    &s // we return a reference to the String, s
} // Here, s goes out of scope, and is dropped. Its memory goes away.
  // Danger!

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

//fn change(some_string: &String) {
//    some_string.push_str(", world");           //error: some_string is immutabble
//}

