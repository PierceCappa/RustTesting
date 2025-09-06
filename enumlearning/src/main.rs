
//I don't yet know what this derive line does, but it allows me to print the enum, so who cares
#[derive(Debug)]
enum Nationality
{
    American,
    British,
}


fn main() {

    let nationality: Nationality = Nationality::American;

    println!("My nationality is {:?}", nationality);
}
