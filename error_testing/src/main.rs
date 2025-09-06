//2 ways of handling errors


fn main() {
    


    //Approach 1
    //The option value is always available, and it is essentially an enum.
    // enum Option<T>
    // {
    //     Some(T),//some value
    //     None,//no value
    // }


    //Approach 2
    //Second approach is similar to first but now it can return the cause.
    // enum Result<T, E>{
    //     Ok(T),
    //     Err(E),
    // }



    
    
    
    //first option in practice
    let result = divide_option(10.0, 0.0);
    let second_result = divide_option(10.0, 2.0);

    println!("First Option, First result = {:?}, Second result = {:?}", result, second_result);

    //second option in practice
    let result = divide_result(10.0, 0.0);
    let second_result = divide_result(10.0, 2.0);

    println!("Second Option, First result = {:?}, Second result = {:?}", result, second_result);
}



fn divide_option(numerator: f64, denominator: f64) -> Option<f64>
{
    if denominator == 0.0
    {
        None
    }
    else
    {
        Some(numerator / denominator)
    }
}


fn divide_result(numerator: f64, denominator: f64) -> Result<f64, String>
{
    if denominator == 0.0
    {
        Err(String::from("Cannot divide number by 0"))
    }
    else
    {
        Ok(numerator / denominator)
    }
}


