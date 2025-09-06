


fn main() {
    
    //Rust vector;
    //Vectors must be mutable so we can add to it.
    let mut vector: Vec<f64> = Vec::new();



    //create a vector with default values
    let second_vector: Vec<i32> = vec![1, 5, 1];

    //Iterating over a range.
    for i in 0..5
    {
        vector.push(i as f64)
    }


    println!("Vector values = {:?}", vector);
    println!("Second Vector values = {:?}", second_vector);


    println!("The third element of the first vector is {}", vector[3]);

    //The get value will return the value at the given index, or it will return the None option if it does not exist.
    let second_value: Option<&f64> = vector.get(2);
    //Match is the Rust version of a switch statement
    match second_value
    {
        Some(second_value) => println!("The second element of the first vector is {}", second_value), //by using match, we can cast back out of option to the typed value.
        None => println!("the get value could not find a value"),
    }


    //UTF-8

    let first_string = "Funny string ".to_string();
    let second_string = String::from("I am a new string");
    let mut s = String::from("Hello ");
    s.push_str("World!");

    let mut s2 = String::from("Foo ");
    s2 += &String::from("Bar");

    let full_string: String = format!("{} name is {}", String::from("My"), String::from("James"));


    println!("Then value of full_string is {}", full_string);
    println!("The value of s is {}", s);
    println!("The value of s2 is {}", s2);


    




}
