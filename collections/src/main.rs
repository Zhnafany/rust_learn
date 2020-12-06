fn main() {
    // vectors
    // create new empty with type specify
    let v: Vec<i32> = Vec::new();
    // create new with macro, values types get from default i32
    let v = vec![1, 2, 3, 4, 5];

    //will panic if value by index not exist
    //slice, reference to object
    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    let mut v = vec![1, 2, 3, 4, 5];

    let first = &v[0];

    //immutable variable "first" borrow ownership
    //v.push(6); //ERROR!!!

    println!("The first element is: {}", first);

    // for in of vector
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }

    //mutable iterator
    // * dereference operator - get value by reference
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }

    // enum as type of vector to store different types
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];


    //strings
    //new empty string
    let mut s = String::new();

    //to_string from std::fmt::Display trait
    //impl to convert to string
    let data = "initial contents";

    let s = data.to_string();

    // the method also works on a literal directly:
    let s = "initial contents".to_string();

    //similar to to_string
    let s = String::from("initial contents");

    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שָׁלוֹם");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");


    let mut s = String::from("foo");
    s.push_str("bar");

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {}", s2);

    let mut s = String::from("lo");
    s.push('l');

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    //same fn add(self, s: &str) -> String {
    //
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
    println!("s3 is {}", s3);

    //concatenate multiple strings
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3;

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);

    ///////////////////////////////////
    let s1 = String::from("hello");
    //let h = s1[0];  // error can't get char by index; Rust don't support string indexing
    
    let hello = "Здравствуйте";
    let s = &hello[0..4]; // s will be only Зд, because 1 chacacter here(UTF-8) 2 bytes
    //let s = &hello[0..1]; // error slice outside of char boundary, 2 bytes char 
    println!("s is {}", s);

    let hello = "hello";
    let s = &hello[0..4]; // s will be only hello, because 1 chacacter here(ASCII) 1 bytes
    println!("s is {}", s);

    //iterating over chars in string
    for c in "नमस्ते".chars() {
        println!("{}", c);
    }

    //iterating over bytes in string
    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }

    /////////////////////////////////////////////
    //hashes hashMap
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    //convert using iterators and collect function - vec of tuples to hashMap
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let mut scores: HashMap<_, _> = // infer the types from based on vec types
    teams.into_iter().zip(initial_scores.into_iter()).collect();

    /////////////////////
    //for types that impl Copy trait - insert will use copy
    //without Copy, like String, take ownership
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point, try using them and
    // see what compiler error you get!

    /////////////
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name); //return Option<&V>; V - value

    ///////////////////
    //iterate hashMap by key,value
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    ///////////////////
    //replace the value of key Blue
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    println!("{:?}", scores);

    ///////////////////////
    //or_insert - insert when no such key
    // entry return Entry enum with exist value or not
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);
    
    //////////////////
    //or_insert return mut reference to value
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1; //dereference
    }

    println!("{:?}", map);
}

