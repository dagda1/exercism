// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

use std::collections::HashMap;

fn get_word_count<'a>(words: &[&'a str]) -> HashMap<&'a str, u16> {
    let mut frequency: HashMap<&'a str, u16> = HashMap::new();

    for word in words.iter() {
        *frequency.entry(word).or_insert(0) += 1;
    }

    
    frequency
}

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let mut magazine_frequency = get_word_count(magazine);
    let mut note_frequency = get_word_count(note);
    
    for (word, word_frequency) in note_frequency {
        let magazine = magazine_frequency.get(word);

        if magazine.is_none() {
            return false;
        }

        let freq = magazine.unwrap();

        if(freq < &word_frequency) {
            return false;
        }
    }

    true
}
