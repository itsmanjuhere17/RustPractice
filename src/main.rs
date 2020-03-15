use std::io;
use rand::Rng;
use std::cmp::Ordering;
//use std::intrinsics::prefetch_read_instruction;

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

    //Basic Data Types.
    let x=5+5; //Integer addition
    let fx = 2.30; //Floating.
    let boolean = true; //Boolean.
    let bul= false;
    let ch = 'p'; //charcter.

    //Compound Data Types.
    //Tuple
    let tup = (100,5.5,true);
    let (x,y,z)=tup;
    println!("Value of y in tuple is:{}",y);
    let tupValue1 = tup.1; //Accessing through . operator.
    println!("tupValue1 is:{}",tupValue1);
    //Arrays.
    let arr=[1,2,3,4,5];
    let arr1 = [3;5];
    //Accessing Array values.
    let arrVal = arr[2];
    let index =10;
    //let arrval1 = arr[index];//Error thrown at run-time as out of bounds exception.

    //Functions:
    dummy_function(10,20);
    let retVal = dummy_function1();
    println!("return value is:{}",retVal);
    let retval1 = dummy_function2(5);
    println!("Returned Value from dummy2 is:{}",retval1);

    //Loops and Control Flows:
    let dummyNumber =  10;
    let number = if dummyNumber==10{ //Note you can return the value from if block.
        5
    }
    else{
        6
    };
    println!("Number from if condition is:{}",number);

    //loop
    let mut counter = 0;
    let loopValue = loop{
        counter=counter+1;
        if counter==10{
            break
            counter*2
        }
    };
    println!("Return value from loop is:{}",loopValue);

    //while loop:
    let arr=[1,2,3,4,5];
    let mut index=0;
    while index<5{
        println!("Elements of array are:{}",arr[index]);
        index=index+1;
    }
    //for loop.
    for element in arr.iter(){
        println!("Retrieving elements from array using for loop:{}",element);
    }
    //count down using for loop. A new approach.
    for number in 1..4{ //Note:Here, 4 will not be considered in the range.
        println!("for-loop:Printing range:{}",number);
    }
    for number in (1..5).rev(){
        println!("for-loop:Printing range in reverse order:{}",number); //Printing the range reverse.
    }
    //Practice:Fibonacci
    let mut fib_range = String::new();
    io::stdin().read_line(&mut fib_range).expect("Failed to read line");
    let mut fib_range:i32 = match fib_range.trim().parse(){
        Ok(num)=>num,
        Err(_)=>{
            -1
        }
    };
    if fib_range <=0{
        println!("Enter a valid positive range");
        return;
    }
    let result = fibonacci(fib_range);
    fib_range = 15; //Can use, fib_range afterwards also. As i32 has copy trait. Check ownership concept in Rust.
    println!("Fibonacci value of {}th range is:{}",fib_range,result);
    //Rust Ownership:
    let mut x=5; //Allocated on Stack.
    let y=x; //Allocated on Stack. Hence, no ownership problem.
    x=10; //x is mutable and can be changed.
    println!("X value is:{} and Y value is:{}",x,y);
    let mut s=String::from("Hello");//It allocates string on Heap.
    //Here, variable s is on Stack and points the data to Heap.
    let mut s1 = s;//By doing this, 's' ownership is transferred to s1. Similar to move concept in C++.
    //println!("Value of s is:{}",s); Error. Can't use s after move.
    s=String::from("World"); //But, we can re-initialise s with other String type.
    println!("Value of s is:{}",s);
    let str=String::from("Manjunath");
    let str1=str.clone(); //If we want to have copy of the String, we need to clone it.
    println!("Using str value again after clone():{}",str);
    let mut stri = String::from("Good Morning");
    let (stri1,n) = return_Mutliple(stri);
    println!("Returned value of String and length of it is:{} {}",stri1,n);

}

fn return_Mutliple(s:String)->(String,usize)
{
    let len = s.len();
    return (s,len);
}

fn fibonacci(fib_range:i32)->i32{
    let mut x=0;
    let mut y=1;
    let mut z= -1;
    if fib_range ==0{
        z=0;
    }
    else if fib_range==1 {
        z=1;
    }
    else {
        for ele in 2..fib_range+1{ //Here, fibrange is not included in the loop.So, traverse to fibrange+1
            z=x+y;
            x=y;
            y=z;
        }
    }
    return z;
}

fn dummy_function2(x:i32)->i32
{
    //return x+5;  //1st flavor of returning. If return is specified explicitly, we can use semicolon.
    x+5 //Second flavor of returning. NOTE:Here without semicolon meaning it is expression
    //x+5; //Note:This print error as here semicolon mean it is statement and not expression. This is rust-way :(
}
fn dummy_function1()->i32
{
    return 10;
}
fn dummy_function(x:i32,y:i32)
{
    println!("The value of the param's x and y are:{} {}",x,y);
}
