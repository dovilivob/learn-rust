use std::collections::HashMap;

fn main() {
    let team_1 = String::from("Blue");
    let team_2 = String::from("Yellow");

    let mut scores = HashMap::new();

    scores.insert(team_1, 10); // hashmap.insert(key, value);
    scores.insert(team_2, 12);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
    println!("scores.get(&team_name): {:?}", score); // output: Some(10)

    // iterate through hashmap
    for (key, value) in &scores {
        println!("key: {}, value: {}", key, value);
    }
    println!();

    {
        let mut scores = HashMap::new();

        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Blue"), 18); // overwrite the last modifying

        scores.entry(String::from("Yellow")).or_insert(30); // if key not exist, insert
        scores.entry(String::from("Yellow")).or_insert(40); // if key exist, do nothing

        println!("{:?}", scores)
    }

    // this block below can count the words in a string
    {
        let text: &str = "hello world wonderful world";
        let mut map = HashMap::new();

        for word in text.split_whitespace() {
            let word_count = map.entry(word).or_insert(0); // return a mutable reference to the value, pretty useful
            *word_count += 1; // directly modify the reference
        }
        println!("{:?}", map);
    }
}
