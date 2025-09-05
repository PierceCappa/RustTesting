



fn main() {


    let flag: bool = false;

    let age:i32 = 21;

    if flag 
    {
        println!("Flag is true, yipee! age is {}", age);
    }
    else if age > 10
    {
        println!("age is more than 10 but flag is false, its ok");
    }
    else
    {
        println!("age is less than 10 and flag is false, what is even the point");
    }

    //the expressions in the if else statement must return the same type
    let other_age = if flag && age > 10
    {
        19
    }
    else {
        10
    };

    let age_flag = flag && age > 10;


    println!("other age is {} and age flag is {}", other_age, age_flag);


}
