use std::collections::HashMap;
use std::{fs, io};

fn do_stuff() -> Result<(), io::Error> {
    let mut s = fs::read_to_string("src/info.txt")?;
    s = s.to_lowercase();

    let mut m = HashMap::<&str, i32>::new();

    for word in s.split(|c: char| c.is_ascii_whitespace() || c.is_ascii_punctuation()) {
        if word == "" {
        } else if m.contains_key(word) {
            m.entry(word).and_modify(|x| {
                *x = *x + 1;
            });
        } else {
            m.insert(word, 1);
        }
    }

    let mut s: Vec<(&&str, &i32)> = m.iter().collect();
    s.sort_by(|x,y| y.1.cmp(&x.1));


    let mut s1 = s.clone();
    s1.sort_by_key(|x| x.0.len());
    let max_len_word = s1.iter().next_back();


    for i in s {
        println!("{:<number$} => {}", i.0, i.1, number = max_len_word.unwrap().0.len());
    }

    Ok(())
}

fn main() {
    let _ = do_stuff();
}
