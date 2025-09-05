
// rust has the following int types
//i8, i16, i32, i64, i128, isize
//u8, u16, u32, u64, u128, usize
//the isize and usize types depend on the architecture of the computer, they are the size of a standard memory address. Or the size of a pointer.

//the following are floating point types
//f32, f64

//Also booleans exist with the type bool

//we also have the char type

fn main()
{
    let num1: i32 = 10;
    let num2: isize = 2000000;

    let num3: u32 = 14;

    u32::MIN; //gives the minimum value of a u32
    u32::MAX;


    println!("num1 is {}, num2 is {}, num3 is {}", num1, num2, num3);

    let bool_val: bool = true;
    println!("bool_val is {}", bool_val);

    let almost_pi: f32 = 3.14159;
    let pi: f64 = 3.141592653589793;
    println!("almost_pi is {}, pi is {}", almost_pi, pi);

    let character: char = 'a';
    println!("this is a char {}", character);


}
