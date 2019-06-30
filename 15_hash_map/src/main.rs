use std::collections::HashMap;

fn main() {
    // ########################################### HASHMAP ###########################################
    // The type HashMap<K, V> stores a mapping of keys of type K to values of type V. It does 
    // this via a hashing function, which determines how it places these keys and values into memory.
    // Hash maps are useful when we want to look up data not by using an index, as we can with
    // vectors, but by using a key that can be of any type. Note that we need to first use the HashMap 
    // from the collections portion of the standard library.
    // Just like vectors, hash maps store their data on the heap. Like vectors, hash maps are homogeneous:
    // all of the keys must have the same type, and all of the values must have the same type.

    // *********************************** Creating a New Hash Map ***********************************
    // We can create an empty hash map with new and add elements with insert.
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    println!("{:?}", scores);

    // Another way of constructing a hash map is by using the collect method on a vector of tuples, 
    // where each tuple consists of a key and its value. The collect method gathers data into a number 
    // of collection types, including HashMap.  For example, if we had the team names and initial scores
    // in two separate vectors, we could use the zip method to create a vector of tuples where “Blue”
    // is paired with 10, and so forth. Then we could use the collect method to turn that vector of 
    // tuples into a hash map, as shown below
    let teams  = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
    // The type annotation HashMap<_, _> is needed here because it’s possible to collect into many 
    // different data structures. For the parameters for the key and value types, however, we use 
    // underscores, and Rust can infer the types that the hash map contains based on the types of the 
    // data in the vectors.

    // *********************************** Hash Maps and Ownership ***********************************
    // For types that implement the Copy trait, like i32, the values are copied into the hash map. 
    // For owned values like String, the values will be moved and the hash map will be the owner of 
    // those values.
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // println!("{}, {}", field_name, field_value); // compile time error: "value borrowed here after move"
    // We aren’t able to use the variables field_name and field_value after they’ve been moved into 
    // the hash map with the call to insert.
    // If we insert references to values into the hash map, the values won’t be moved into the hash 
    // map. The values that the references point to must be valid for at least as long as the hash
    // map is valid.

    // ******************************* Accessing Values in a Hash Map *******************************
    // We can get a value out of the hash map by providing its key to the get method:
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
    // the result will be Some(&10). The result is wrapped in Some because get returns an Option<&V>;
    // if there’s no value for that key in the hash map, get will return None.

    // We can iterate over each key/value pair in a hash map in a similar manner as we do with vectors,
    // using a for loop:
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // ************************************* Updating a Hash Map *************************************
    // Although the number of keys and values is growable, each key can only have one value associated 
    // with it at a time. When we want to change the data in a hash map, we have to decide how to 
    // handle the case when a key already has a value assigned.

    // ------------------------------------- Overwriting a Value -------------------------------------
    // If we insert a key and a value into a hash map and then insert that same key with a different 
    // value, the value associated with that key will be replaced.
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);
    println!("{:?}", scores); // This code will print {"Blue": 25}.The original value of 10 has been overwritten.

    // ------------------------ Only Inserting a Value If the Key Has No Value ------------------------
    // It’s common to check whether a particular key has a value and, if it doesn’t, insert a value for
    // it. Hash maps have a special API for this called entry that takes the key you want to check as a 
    // parameter. The return value of the entry method is an enum called Entry that represents a value 
    // that might or might not exist.
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);
    println!("{:?}", scores);

    // --------------------------- Updating a Value Based on the Old Value ---------------------------
    // Another common use case for hash maps is to look up a key’s value and then update it based on 
    // the old value.
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
    // The or_insert method actually returns a mutable reference (&mut V) to the value for this key. 
    // Here we store that mutable reference in the count variable, so in order to assign to that value, 
    // we must first dereference count using the asterisk (*). The mutable reference goes out of scope 
    // at the end of the for loop, so all of these changes are safe and allowed by the borrowing rules.
}
