fn main() {
    // ###################################### STRING ######################################
    // Rust has only one string type in the core language, which is the string slice str 
    // that is usually seen in its borrowed form &str.
    // String slices, which are references to some UTF-8 encoded string data stored elsewhere.
    // String literals, for example, are stored in the program’s binary and are therefore 
    // string slices.
    // The String type, which is provided by Rust’s standard library rather than coded into
    // the core language, is a growable, mutable, owned, UTF-8 encoded string type.
    // “strings” in Rust, usually mean the String and the string slice &str types, not just 
    // one of those types.

    // ****************************** Creating a New String ******************************
    let mut s = String::new(); // creates a new empty string called s, which we can then load data into.
    // Often, we’ll have some initial data that we want to start the string with. For that, 
    // we use the to_string method
    let data = "initial contents";
    let s = data.to_string(); // This code creates a string containing initial contents.
    // We can also use the function String::from to create a String from a string literal.
    let s = String::from("initial contents");

    // ******************************** Updating a String ********************************
    // A String can grow in size and its contents can change

    // ------------------- Appending to a String with push_str and push -------------------
    // We can grow a String by using the push_str method to append a string slice
    let mut s = String::from("foo");
    s.push_str("bar"); // s = "foobar"
    println!("{}", s);
    // The push_str method takes a string slice because we don’t necessarily want to take 
    // ownership of the parameter.

    // The push method takes a single character as a parameter and adds it to the String.
    let mut s = String::from("lo");
    s.push('l');
    println!("{}", s);

    // ------------- Concatenation with the + Operator or the format! Macro -------------
    // Often, you’ll want to combine two existing strings. One way is to use the + operator
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
    // this statement actually takes ownership of s1, appends a copy of the contents of s2, 
    // and then returns ownership of the result.

    // For more complicated string combining, we can use the format! macro:
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{}-{}-{}", s1, s2, s3);
    // The format! macro works in the same way as println!, but instead of printing the output 
    // to the screen, it returns a String with the contents. The version of the code using 
    // format! is much easier to read and doesn’t take ownership of any of its parameters.

    // ******************************* Indexing into Strings *******************************
    // if we try to access parts of a String using indexing syntax, we’ll get an error.
    let s1 = String::from("hello");
    // let h = s1[0]; // compile time error: `std::string::String` cannot be indexed by `{integer}`
    // The error and the note tell the story: Rust strings don’t support indexing. But why not? 
    // To answer that question, we need to discuss how Rust stores strings in memory.

    // ------------------------------ Internal Representation ------------------------------
    // A String is a wrapper over a Vec<u8>. Let’s look at some of our properly encoded UTF-8 
    // example strings
    let len = String::from("Hola").len();
    println!("Length of 'Hola' word is: {}", len);
    // In this case, len will be 4, which means the vector storing the string “Hola” is 4 bytes
    // long. Each of these letters takes 1 byte when encoded in UTF-8.
    // But what about the following line?
    let len = String::from("السلام عليكم").len();
    println!("Length of 'السلام عليكم' is {}", len);
    // In this case, len will be 23, which means the vector storing the string "السلام عليكم" is
    // 23 bytes long. Each of these letters takes 2 bytes while space takes one byte.
    // Because each Unicode scalar value in that string takes 2 bytes of storage. Therefore, an
    // index into the string’s bytes will not always correlate to a valid Unicode scalar value.

    // --------------- Bytes and Scalar Values and Grapheme Clusters! Oh My! ---------------
    // Another point about UTF-8 is that there are actually three relevant ways to look at 
    // strings from Rust’s perspective: as bytes, scalar values, and grapheme clusters.
    // If we look at the arabic word "السلام", it is stored as a vector of u8 values that 
    // looks like this:
    // [216, 167, 217, 132, 216, 179, 217, 132, 216, 167, 217, 133]
    let mut Bytes = Vec::new();
    for b in "السلام".bytes(){
        Bytes.push(b);
    }
    println!("{:?}", Bytes);
    // That’s 12 bytes and is how computers ultimately store this data. If we look at them 
    // as Unicode scalar values, which are what Rust’s char type is, those bytes look like this:
    // ['ا', 'ل', 'س', 'ل', 'ا', 'م']
    let mut Scalar = Vec::new();
    for c in "السلام".chars(){
        Scalar.push(c);
    }
    println!("{:?}", Scalar);
    // A final reason Rust doesn’t allow us to index into a String to get a character is 
    // that indexing operations are expected to always take constant time (O(1)).

    // ********************************** Slicing Strings **********************************
    // Indexing into a string is often a bad idea because it’s not clear what the return type 
    // of the string-indexing operation should be: a byte value, a character, a grapheme cluster,
    // or a string slice. Therefore, Rust asks you to be more specific if you really need to 
    // use indices to create string slices.
    let word = "عليكم";
    let s = &word[0..2];
    println!("{}", s);
    // Here, s will be a &str that contains the first 4 bytes of the string. Earlier, we 
    // mentioned that each of these characters was 2 bytes, which means s will be ع.
    // What would happen if we used &word[0..1]? The answer: Rust would panic at runtime 
    // in the same way as if an invalid index were accessed in a vector:

    // If we need to perform operations on individual Unicode scalar values, the best way 
    // to do so is to use the chars method.
    for c in word.chars(){
        println!("{}", c);
    }

    // The bytes method returns each raw byte
    for b in word.bytes(){
        println!("{}", b)
    }
}