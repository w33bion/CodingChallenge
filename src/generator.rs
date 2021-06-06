use rand::Rng;
use std::fs::File;
use std::io::prelude::*;

const TOKEN_LENGTH: usize = 7;


pub fn write_tokens() -> std::io::Result<()> {
    //pre-generate all tokens so that they can be written to the file in one batch:
    let tokens = get_n_tokens(10); 

    let mut file = File::create("tokens.txt")?;
    file.write_all(tokens.as_bytes())?;
    Ok(())
}

fn get_n_tokens(n: i32) -> String {
    let mut tokens = String::from("");
    for _ in  0..n {
       tokens += &format!("{}\n", get_token())[..];
    }

    return tokens;
}

fn get_token() -> String {
    let mut chars : [char; TOKEN_LENGTH] = ['a'; TOKEN_LENGTH];
    let mut rng = rand::thread_rng();

    for i in 0..TOKEN_LENGTH {
        let num: u8 = rng.gen_range(97..122);
        chars[i] = num as char;
    }

    return chars.iter().collect::<String>();
}