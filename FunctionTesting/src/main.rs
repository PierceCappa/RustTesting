

//to make a global expression, it should be const or static.
const expression_test:i32 = {
    15 + 12
};


fn main() {
    hello_world();
    multiply_height(15);
    println!("Added nums of 15 and 12 = {}", add_numbers(15, 12));


    //In expresisons, the last line without semi colons will be the return.
    let main_expression: i32 = {
        let price: i32 = 5;
        let qty: i32 = 12;
        price * qty
    };

    println!("price * qty is {}", main_expression);
    println!("{}", expression_test);


    
}

fn hello_world()
{
    println!("Hello World from function");
}


fn multiply_height(height: i32)
{
    println!("New height {}", height * 2);
}

fn add_numbers(num1: i32, num2: i32) -> i32
{
    return num1 + num2;
}



