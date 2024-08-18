fn main() {

    let word = "hello".to_string();
    println!("New Word: {:?}", encrypt(2, word));
    println!("Old Word: {:?}", decrypt(2, "jgnnq".to_string()))
}


fn decrypt(shift : usize, word : String) -> String{
    let alphabet: Vec<char> = ('a'..='z').collect();
    let mut new_word = String::new();

    for ch in word.chars() {
        if let Some(index) = alphabet
            .iter()
            .position(|&c| c == ch) 
            {
            let new_index = (index - shift) % alphabet.len();
            new_word.push(alphabet[new_index]);

        } else {
            new_word.push(ch);
        }
    }

    new_word
}

fn encrypt(shift: usize, word: String) -> String {
    let alphabet: Vec<char> = ('a'..='z').collect();
    let mut new_word = String::new();

    for ch in word.chars() {
        if let Some(index) = alphabet
            .iter()
            .position(|&c| c == ch) 
            {
            let new_index = (index + shift) % alphabet.len();
            new_word.push(alphabet[new_index]);

        } else {
            new_word.push(ch);
        }
    }

    new_word
}

