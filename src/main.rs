use rand::Rng;
use std::{io,cmp::Ordering}; //Nested Paths. Instead of defining two use statements,we can combine into one.
use std::collections::*; //Global operator. Need to be careful when declaring glob operator.

//use std::intrinsics::prefetch_read_instruction;

struct User{
    name:String,
    age:u32,
    email:String,
    address:String
}

#[derive(Debug)]
enum Fruit{
    Mango,
    Orange,
    Apple
}

#[derive(Debug)]
struct Rect{
    length:u32,
    breadth:u32
}

impl Rect{
    fn rectArea(&self)->u32{
        return self.length * self.breadth;
    }
    fn getSquare(dim:u32)->Rect{
        let rect=Rect{
            length:dim,
            breadth:dim
        };
        rect
    }
}

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
    println!("Enter the input fibonacci range");
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
    //fib_range = 15; //Can use, fib_range afterwards also. As i32 has copy trait. Check ownership concept in Rust.
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
    //We are passing stri by value.It means ownership is transferred to param of "return_Multiple" function.
    let (stri1,n) = return_Mutliple(stri); //Using tuple to return multiple values.
    println!("Returned value of String and length of it is:{} {}",stri1,n);

    //References.
    let strRef = String::from("String Reference");
    let mut strRef1 = String::from("Mutable String Reference");
    let retLen = passByReference(&strRef);
    println!("String value and it's returned length by reference is:{} {}",strRef,retLen);
    let retLen = passByReferenceMutable(&mut strRef1);//Passing mutable reference as parameter.
    println!("String value and it's returned length by mutable reference is:{} {}",strRef1,retLen);
    dummy(&mut strRef1);//It works. Scope has changed from above.
    //dummy1(&mut strRef1,&mut strRef1); NOTE: Throws error as you can't pass mutable reference to two entities at the same scope.
    println!("String after dummy call is:{}",strRef1);
    //let retStr = returnbyRef(); //Error: Cannot return dangling references.

    //Slice Type.
    let  s="Manjunath"; //By default literals are slices.
    //s="Manjugadu";
    println!("String literal is:{}",s);
    let sliceStr = String::from("Slice String");
    let slice = &sliceStr[0..5]; //Here, type is "str"

    let arr=[1,2,3,4,5];
    let arrSlice = &arr[2..4];
    for ele in arrSlice.iter(){
        println!("Array Slice elements are:{}",ele);
    }

    //Structures:
    //Instance of struct
    let instUser = User{
        name:String::from("Manjunath"),
        age:29,
        email:String::from("manjunath.malepati17@yahoo.com"),
        address:String::from("Kohmankaari 9A")
    };
    //Note: In structure, you cannot have individual fields as mutable or immutable. Either entire struct is mutable or immutable.
    //instUser.email="manjumalepati17@gmail.com";//Error as instUser is immutable.
    println!("User name is:{}",instUser.name);
    //Creating a new User instance from the existing user instance but with limited data using "Struct Update" syntax.
    let instUser1=User{
      name:String::from("Prathyusha"),
      age:28,
      ..instUser
    };
    //println!("First User address is:{}",instUser.address);//Note: Error. Here, Struct is moved already.
    println!("Second User address is:{}",instUser1.address);
    struct Dummy{
        name:String,
        pincode:u32
    };
    //Tuple structs.
    struct Color(i32,i32,i32);//No Named fields.Only types are present in tuple structs.
    struct Point(i32,i32,i32);
    let color=Color(0,1,2);
    let origin=Point(0,0,1);
    //Note:Here both are different types.
    //Rectangle Program
    let dimensions=Rect{
        length:10,
        breadth:5
    };
    let result=area(&dimensions); //pass by reference.
    println!("Area of rectangle is:{}",result);
   //println!("Area of rectangle is:{}",dimensions.length); //Error as struct is moved already.
    println!("Rectangle dimensions are:{:#?}",dimensions); //Note:This works only if #debug is added at struct declaration.

    println!("Area of rectangle defined inside 'Rect struct' is:{}",dimensions.rectArea());//Method defined in struct.
    let square=Rect::getSquare(25);
    println!("Square dimensions are:{:#?}",square);
    println!("Area of square is:{}",square.rectArea());

    //Enumerations.
    #[derive(Debug)]
    enum IpAddType{
        ipV4(String),
        ipV6(String)
    };
    let ip4 = IpAddType::ipV4(String::from("127.0.0.1"));
    println!("Ip Add type is:{:#?}",ip4);

    #[derive(Debug)]
    enum IpAddr{
        ipv4(u8,u8,u8,u8),
        ipv6(String)
    };
    let ipv4 = IpAddr::ipv4(127,0,0,1);
    println!("Ip Add type is:{:#?}",ipv4);
    //We can declare, multiple types inside enum.
    enum Message{
        Quit,
        Move(u8,u8),
        Write(String),
        ipAddress(IpAddr) //Defining an enum Inside enum.
    };
    //Option<T>
    let some_int=Some(5);
    let some_str=Some("Manjunath");
    let nothing:Option<u32>=None;

    //Match expression with enums.
    let someVal:Option<u32>=Some(1); //If Some value is passed,we return somevalue else, None if None is passed.
    let result=increment(someVal);
    println!("Result is:{:?}",result);

    let y=returnFruitSeason(Fruit::Mango);
    println!("Season of the passed fruit {:#?} is {}",Fruit::Mango,y);

    //Selection Sort Practice.
    let mut vector = vec![2,1,5,3,4];
    //selection_sort(&mut vector);
    // for (vecIndex,vecElement) in vector.iter_mut().enumerate(){ //Note: Using enumerate we cannot modify the vector contents.
    //     vector[vecIndex]=vecIndex *10;
    // }
    println!("Elements of vector after Selection sort are:");
    for ele in vector.iter(){
        println!("{}",ele);
    }
} //End of main()

fn selection_sort(vector:&mut Vec<i32>){
    let mut sortedIndex= 0;
    let mut key = vector[0];
    println!("Vector size is:{}",vector.len());
    for index in 1..vector.len(){
        println!("Index is:{}",index);
        sortedIndex = index-1;
        key=vector[index];
        println!("Key is{}",key);
        while sortedIndex >= 0{
            if  key < vector[sortedIndex]{
                vector[sortedIndex+1]=vector[sortedIndex];
            }
            else {
                vector[sortedIndex + 1] = key;
                println!("Break is about to encounter{} {} {} {}",index,sortedIndex,vector[index],vector[sortedIndex]);
                break;
            }
            println!("Sorted Index is:{}",sortedIndex);
            sortedIndex = sortedIndex-1;
        }
    }
    if sortedIndex < 0{
        vector[sortedIndex+1]=key;
    }
}

fn returnFruitSeason(fruit:Fruit)->String
{
    //Using match expression.
    // match fruit{
    //     Fruit::Mango=>String::from("Summer"),
    //     Fruit::Orange=>String::from("All Seasons"),
    //     Fruit::Apple=>String::from("Winter")
    // }
    // Or
    if let Fruit::Mango = fruit{
        String::from("Summer")
    }
    else {
        String::from("All Seasons")
    }
}
fn increment(param:Option<u32>)->Option<u32>{
    // match param{
    //     Some(i)=>{
    //         //println!("Something is:{:?}",Some(i+1));
    //         Some(i+1)
    //     },
    //     None=>None,//Note:match is exhaustive search pattern. if you don't explicitly specify all the matching patterns, compiler will throw an error unless you specify _plcae holder like below.
    //     _=>None //Like a default specifier in C++ in switch.
    // }

    //using
    if let Some(i)=param{
        Some(i+1)
    }
    else {
        None
    }
}

fn area(rect:&Rect)->u32
{
    return rect.length * rect.breadth;
}
// fn returnbyRef()->&String
// {
//     let str=String::from("Returning from here");
//     return &str; //You should not return by reference to a local. Dangling reference. Compiler throws error.
//     //Solution is to return by value.Doing so, it is moved to destination.
// }
fn dummy(s:&mut String){
    s.push_str(" Gully Boys");
}
fn dummy1(s:&mut String,s1:&mut String)
{
    s.push_str(" Gully Boys");
}
fn passByReferenceMutable(s:&mut String)->usize //Passing by reference.
{
    s.push_str(" Rustacean");
    s.len()
}

fn passByReference(s:&String)->usize //Passing by reference.
{
    //s.push_str("Rustacean"); You can't modify the reference if it is declared as immutable.
    s.len()
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
