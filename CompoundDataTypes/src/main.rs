
//Compound data types
//Arrays, tuples, slices, and strings



fn main() {
    
    let number_array: [i32; 4] = [13, 41, 21, -9];

    println!("Array = {:?}", number_array);

    //To create a string, we create a reference to a string type. I don't know why I cant make a str type
    let word: &str = "Hello World!";

    let fruits: [&str; 3] = ["apples", "oranges", "banana"];

    println!("Word = {}, Fruits = {:?}", word, fruits);

    //Tuples
    //name, age, alive
    let user: (String, i32, bool) = ("Bruce".to_string(), 24, true);

    println!("User info {:?}", user);

    let users: [(String, i32, bool); 2] = [("Pierce".to_string(), 26, true), ("Wade".to_string(), 25, true)];
    println!("Users = {:?}", users);


    //slices, a slice is a segment of continuous memory
    let number_slice: &[i32] = &[15, 11, 12, 01234];

    println!("here is a number slice = {:?}", number_slice);

    //string vs string slices (&str)
    //strings are growable, mutable and owned.

    //we have to add the mut to the type to make it changeable.
    let mut funny_string: String = String::from("Hello");
    println!("string part 1, {}", funny_string);

    funny_string.push_str(" World!");

    println!("string part 2, {}", funny_string);

    let funny_slice: &str = &funny_string;
    println!("string as a slice, {}", funny_slice);

    //This gets a subset of the original string from indexes 0 - 5. This is a slice.
    let funny_subset: &str = &funny_string[0..5];
    println!("string as a slice, {}", funny_subset);

}
