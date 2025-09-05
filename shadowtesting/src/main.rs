
//What is shadowing
//You can create a new variable with the same name as another variable.
//When calling a variable, the compiler will use the one last initialized.


fn main() {
    let x = 9;
    let x: i32 = x + 1;
    println!("After shadowing, x = {}", x);

    {
        let x: i32 = x * 2;
        println!("In the inner scope, x = {}", x);

    }

    println!("In the outer scope, x = {}", x);

    //we can change the variables type
    let x = String::from("Hello World");

    println!("Now, x = {}", x);


    //shadowing is different than marking a variable as mutable as it creates a new memory address for each variable, a mut variable is in the same memory.

    /*
    this is also a comment.
    
     */
}
