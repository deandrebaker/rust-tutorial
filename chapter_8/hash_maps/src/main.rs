use std::{collections::HashMap, hash::Hash};

fn main() {
    {
        let mut scores = HashMap::new();

        scores.insert("Blue".to_string(), 10);
        scores.insert("Yellow".to_string(), 50);

        println!("Initial scores: {:?}", scores);
    }

    {
        let mut scores = HashMap::new();

        scores.insert("Blue".to_string(), 10);
        scores.insert("Yellow".to_string(), 50);

        let team_name = "Blue".to_string();
        let score = scores.get(&team_name).copied().unwrap_or(0);

        println!("Score for {}: {}", team_name, score);
    }

    {
        let mut scores = HashMap::new();

        scores.insert("Blue".to_string(), 10);
        scores.insert("Yellow".to_string(), 50);

        for (key, value) in &scores {
            println!("{key}:{value}");
        }
    }

    {
        let field_name = String::from("Favorite color");
        let field_value = String::from("Blue");
        let mut map = HashMap::new();

        map.insert(field_name, field_value);
        println!("Map after insertion: {:?}", map);
        // println!("Field name: {}", field_name);
        // println!("Field value: {}", field_value);
    }

    {
        let mut scores = HashMap::new();

        scores.insert("Blue".to_string(), 10);
        scores.insert("Blue".to_string(), 50);

        println!("Initial scores: {:?}", scores);
    }

    {
        let mut scores = HashMap::new();

        scores.insert("Blue".to_string(), 10);
        scores.entry("Yellow".to_string()).or_insert(25);
        scores.entry("Blue".to_string()).or_insert(50);

        println!("Initial scores: {:?}", scores);
    }

    {
        let text = "hellow world wonderful world";

        let mut map = HashMap::new();

        for word in text.split_whitespace() {
            let count = map.entry(word).or_insert(0);
            *count += 1;
        }

        println!("Word counts: {:?}", map);
    }
}
