use std::collections::HashMap;
// A HashMap<K, V> is a collection of key<K>: value<V> pairs mapped and stored on the heap
pub fn print_hash_maps() {
    let mut scores = HashMap::new(); // instantiate a new hash map

    // insert values
    scores.insert(String::from("Blue"), 10);

    // data can be retreived using the get() method
    let blue = String::from("Blue");
    let score = scores
        .get(&blue) // data of the same type and value as the key must be provided
        .copied() // copy the value
        .unwrap_or(0); // unwrap the option/return 0 if None
    println!("{}: {}\n", blue, score);

    // update value
    scores.insert(String::from("Blue"), 30);

    scores
        .entry(String::from("Yellow"))
        .or_insert(50); // if no value pair exists for key, insert it with the default: value

    // data can also be iterated and operated on
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    let text = "Hiya Chuck Chuck Chuck Chuck Chuck it's Johnnnnn";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:#?}", map);
}
