const VOWEL: [char; 5] = ['a', 'e', 'i', 'o', 'y']; 
const SUFICS: &str = "ay";
const VOWEL_CHAR: char = 'h';

pub fn to_pig_latin(latin: &str) -> String {
    let mut pig_latin = String::new();
    for word in latin.split_whitespace() {
        pig_latin.push_str(&change_word(word)[..]);
    }
    pig_latin
}

fn change_word(word: &str) -> String {
    let mut pig_word = word.to_string();
    let first_char = pig_word.remove(0);
    add_suffics(pig_word, first_char)
}

fn is_vowel(c: char) -> bool{
    let mut answer = false;    
    for v in VOWEL.iter() {
        if *v == c {
            answer = true;
        }
    }
    answer
}

fn add_suffics(word: String, ch: char) -> String {
    format!("{}-{}{} ", word,
        if is_vowel(ch) {VOWEL_CHAR} else {ch},
        SUFICS
    )
}
