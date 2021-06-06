use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;

pub fn read_tokens() {
    // Store the tokens in a hashmap with the format <token, hits>,
    // so they can be stored and accessed quickly. (Hits means how many times the token occured in the list).
    let mut map: HashMap<String, i32> = HashMap::with_capacity(10_000_000);

    if let Ok(lines) = read_lines("tokens.txt") {
        for line in lines {
            let mut token = match line {
                Ok(line) => line,
                Err(error) => panic!("Problem reading token: {}", error),
            };
        
            token.pop(); //remove newline
            let hits: i32 = match map.contains_key(&token) {
                true => match map.get(&token) {
                    Some(v) => *v + 1,
                    None => panic!("unexpected case")
                },
                false => 1
            };

            if (hits > 1) {
                print!("The token {} has {} hits\n", token, hits); //print all non-unique tokens in the console 
            }

            map.insert(token, hits);
        }
    }

    // Now, all the tokens are in the hashmap.
    // Next, I'd filter all the tokens which have exactly one hit and insert them
    // into MySQL in batches of 100,000 rows. That way only 100 requests to the database will be done.
    // And finally I'd filter all tokens with more than one hit and put it into a separate list.
}




fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines()) //read file using a line-iterator instead of reading the whole file at once 
}