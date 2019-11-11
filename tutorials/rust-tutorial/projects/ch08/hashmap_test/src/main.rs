fn main() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    println!("scores is {:?}", scores);

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
    println!("scores is {:?}", scores);

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    println!("field_name: {}, field_value: {}", field_name, field_value);

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    println!("map: {:?}", map);
    // println!("field_name: {}, field_value: {}", field_name, field_value);

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
    println!("score: {:?}", score);

    for (key, value) in scores {
        println!("{}: {}", key, value);
    }

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    println!("Blue's score: {:?}", scores);
    scores.insert(String::from("Blue"), 50);
    println!("Blue's score: {:?}", scores);

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    println!("scores: {:?}", scores);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);
    println!("scores: {:?}", scores);

    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("map: {:?}", map);
}
