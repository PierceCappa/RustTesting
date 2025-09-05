
//Ownership tests
//1 - Each value in rust has a variable that it's owner
//2 - There can be only one owner at a time
//3 - When the owner goes out of scope the value will be dropped.




fn main() {
    let s1: String = String::from("Hello World!");

    let s2 = s1;

    //This will cause errors
    //println!("{}", s1);

    //This is correct
    println!("{}", s2);

    //print_str(s2);

    //This will fail because of the print_str method going out of scope.
    //println!("{}", s2);


    let mut ref_string: &String = &s2;
    println!("{} == {}", s2, ref_string);


    let mut _x: i32 = 15;

    let _r: &mut i32 = &mut _x;

    *_r += 1;

    println!("value of x = {}", _x);






    let mut account: BankAccount = BankAccount{
        owner: "Pierce".to_string(),
        balance: 150000.56,
    };

    account.checkbalance();

    account.withdraw(100.0);
}


fn print_str(value: String)
{
    println!("{}", value);

    //Once this method ends, the value string is out of scope, and the memory will be freed.
}


fn print_str_ref(_value: &String)
{

}


struct BankAccount
{
    owner: String,
    balance: f64,
}

impl BankAccount
{
    //If a class needs to modify the underly data of a class, than
    fn withdraw(&mut self, amount: f64) -> f64
    {
        self.balance -= amount;
        println!("Withdrawing ${}, new balance is now ${}", amount, self.balance);

        amount
    }

    //The lack of the mut in this function definition prevents changes to the underlying values of the class
    fn checkbalance(&self)
    {
        println!("Current balance == ${}", self.balance);
    }
}