use std::io;
use rand::Rng;
use std::cmp::Ordering;
fn main() {
    println!("Hello, world!");
    println!("I'm a Rustacean!");
    //println!("Printable is: {:?}",DebugDummy(10));

    println!("Guessing Number Game");
    let mut count=0;
    loop {
        if count==3 {
            println!("### EXITING GAME ###");
            break;
        }
        let random=rand::thread_rng().
            gen_range(1,101); //Here, upper bound is exclusive i.e range is from 1 to 100 and not 101.

        println!("Please Input your guess");
        let mut  guess= String::new();
        let bytes = io::stdin().
            read_line( &mut guess).
            expect("Failed to read line");//Read Line if success, prints no of bytes user has entered.
        //println!("No of bytes is:{}",bytes);

        //let guess:i32 = guess.trim().parse().expect("Enter a number"); //This is needed to compare guess and random which should be of same types.
        let guess:u32 = match guess.trim().parse(){ //Here, we are making user to give correct parsable input. Till, then we are looping instead of crashing.
            Ok(num)=>num,
            Err(_)=>{
                println!("Please give a valid number");
                continue;
            }
        };
        match guess.cmp(&random) {
            Ordering::Less => {
                count+=1;
                println!("Too Small");
            },
            Ordering::Greater => {
                count+=1;
                println!("Too big")},
            Ordering::Equal => {
                println!("You Win!!");
                break;
            }
        }
    }//End of Loop.

    //Mutability and Shadowing.
    let mut var = 10;
    var =30;
    println!("Old var is:{}",var);
    let var = "Manju"; //This is called Shadowing.
    println!("New var is:{}",var);
}
