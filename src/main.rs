use rand::Rng;
use std::{io,cmp::Ordering,io::ErrorKind}; //Nested Paths. Instead of defining two use statements,we can combine into one.
use std::collections::*; //Global operator. Need to be careful when declaring glob operator. what if one of its modules overlap with the defined one here in the file.
use std::path::Path;
//use std::intrinsics::prefetch_read_instruction;
pub mod collectionprograms;
mod quickpractice;
mod generictypes;
mod tests;
//use quickpractice;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::fs::OpenOptions;
use std::io::BufWriter;
extern crate RustPractice;
use crate::collectionprograms::vectorCollectionProgram;

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
    RustPractice::front_house::hosting::do_seating(); //Library crate. Calling do_seating inside lib.rs file
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
    var =30; //Here, var is mutable.
    println!("Old var is:{}",var);
    let var = "Manju"; //This is called Shadowing. It can be nay type later.
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
    let arr1 = [3;5]; //Declaring array of size 5 with initial value as 3/
    //Accessing Array values.
    let arrVal = arr[2];
    let index =10;
    //let arrval1 = arr[index];//Error thrown at run-time as out of bounds exception.

    let arrVal = arr.get(4);
    let arrValue;
    if let Some(val)=arrVal{
        arrValue = val;
        println!("Array value is:{}",arrValue);
    }
    else {
        println!("Array value not found");
    }


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
            counter*2 //It will execute the next statement after break as well.
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
    let mut strRef = String::from("String Reference");
    let mut strRef1 = String::from("Mutable String Reference");
    let retLen = passByReference(&strRef); //Passing immutable reference.
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
    let sliceStr = String::from("Slice String");//Creating String type from string slice.
    let slice = &sliceStr[0..5]; //Here, type is "str"

    let arr=[1,2,3,4,5]; //Static Array.
    let arrSlice = &arr[2..4]; //Starting from index 2 to 3.Not including 4
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
      email:String::from("prathyusha.talluri10@gmail.com"),
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
    println!("Area of rectangle is:{}",dimensions.length); //Error if struct is moved as value.
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
    let mut index:i32 = 0;
    index = index -1;
    println!("Index value is:{}",index);
    //Strings.
    let mut striing = String::new(); //Allocated on Heap.
    let strLit = "Manjunath";
    let striing = strLit.to_string();
    println!("To String is:{}",striing);
    let mut striing = "Malepati".to_string(); //Directly converting string slice to String type.
    striing.push_str("Manjunath"); //Concatenating using push_str
    striing.push('M'); //With Push you can only push character.
    let mut strin = String::from("Porumamilla");
    let striin =  String::from("Malepati");
    strin.push_str(&striin); //It is called deref coercion. Here, &String type is converted to &str type.
    println!("To String is:{} {}",striing,strin);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let sres = s1+&s2; //Here, we need to specify & in second string in +. + uses fn add signature here. s1 ownership is transferred here.
    println!("String result of s1,s2 and s3 is:{}",sres);
    //Alternative is to use format! macro.
    let formatStr = format!("{} {}",s2,s3);
    println!("String concatenation using format macro is:{}",formatStr);
    //let ch = formatStr[0]; //Indexing in String is not possible in Rust.

    //Accessing Individual chars from String.
    for c in "नमस्ते".chars() { //print each character value.
        println!("{}", c);
    }
    for b in "नमस्ते".bytes() { //print each byte value.
        println!("{}", b);
    }
    for ch in formatStr.chars(){
        println!("{}",ch);
    }
    for by in formatStr.bytes(){
        println!("{}",by);
    }
    let length = "नमस्ते".len();
    println!("Length of string in bytes is:{}",length);//Length always gives size of string in bytes.

    //Hash Maps. Hash table with key and value mapping.
    let mut gameScores = HashMap::new();
    gameScores.insert(String::from("TeamA"),20);
    gameScores.insert(String::from("TeamB"),30);
    gameScores.insert(String::from("TeamB"),50);//Rust overrides the key value if it is already present. Check below for loops output.

    //Using 'entry' method to check if key is present already and if yes,it just returns the mutable reference.Else,it inserts new and returns mutable reference.
    gameScores.entry(String::from("TeamB".to_string())).or_insert(100);//Does not overwrite the key here.
    gameScores.entry(String::from("TeamC".to_string())).or_insert(100);

    for ele in gameScores.iter(){
        println!("Ele is:{:#?}",ele);
    }

    //Extracting Hash Map using tuple and for loop.
    for (team,score) in &gameScores{
        println!("{} {}",*team,*score);
    }
    //Alternative way to create HashMap Using collect() method.
    let teamNames = vec!["TeamA".to_string(),"TeamB".to_string()];
    let scores = vec![50,60];
    let hashTeamScores:HashMap<_,_> = teamNames.iter().zip(scores.iter()).collect(); //Type needs to explicitly mentioned here as collect() can turn into any type.
    //Retrieving value from Key.
    let resultScore =  gameScores.get(&"TeamB".to_string()); //It is giving latest score.
    if let Some(value) = resultScore{
        println!("TeamA score found {}",value);
    }
    else {
        println!("No TeamA score found");
    }
    //2nd Approach.
    let score = match gameScores.get(&"TeamC".to_string()){
        Some(value)=>value,
        _=> &-1 //We can specify _ as well here . Which is similar to default case in switch in C++.
        //None=>&-1
    };
    println!("Score of TeamA is:{}",score);

    //Updating already Hash Map key with a different value.
    let textStr = "Hello world world is not saying Hello as world is in danger";
    let mut wordCounthash:HashMap<_,_> = HashMap::new();
    for word in textStr.split_whitespace(){
        let mut count = wordCounthash.entry(word).or_insert(0);
        *count+=1;
    }
    println!("Occurrences of word are:");
    for (word,counter) in wordCounthash{
        //wordCounthash.get(&word); //Cannot use hashmap again here.It is moved already.
        println!("{} {}",word,counter);
    }

    //Practicing some programs.
    vectorCollectionProgram(); //You can call either way.Preferable is below way.
    collectionprograms::stringtopglatin();

    //Error Handling.
    //panic!("Panicking here");
    /* let file = match file::open("../../cargo.toml"){
        ok(file)=>file,
        err(error)=>match error.kind(){
            errorkind::notfound=>match file::create("hello.txt"){
                ok(fi)=>fi,
                err(error)=>panic!("cannot create a file{}",error)
            }
            other_error=>panic!("some other error:{:?}",other_error)
        }
    };
    */

    //Alternative to above instead of uasing many match expressions.Using closures.
    //closures?? Chapter:13
    let file = File::open("sample.txt").unwrap_or_else(|error|{
        if error.kind()==ErrorKind::NotFound{
            File::create("sample.txt").unwrap_or_else(|error|{
                panic!("Error creating file{:#?}",error)
            })
        }
        else{
            panic!("Error opening file{:#?}",error)
        }
    });
    //using unwrap that calls default panic macro.
    //let fhand = File::open("dingdong.txt").unwrap();//This prints default panic message.
    //let fhand = File::open("dingdong.txt").expect("No such file or directory exists");//it prints user given message instead of default panic one. Suggestion is to use this.

    //Propogating errors to calling functions.
    /* let result = read_from_file();
    match result {
        Ok(str)=>println!("UserName from file is:{}",str),
        Err(error)=>if let ErrorKind::NotFound = error.kind(){
            let mut file = File::create("writetofile.txt").expect("Creation of a file failed");
            let mut file = File::open("writetofile.txt").expect("Opening of file failed");
            let contents="Manjunath is working".to_string();
            file.write_all(b"Hello World!");
            let mut fileContent = String::new();
            file.read_to_string(&mut fileContent);
            println!("File Content is:{}",fileContent);
        }
        else{
            panic!("Some other weird error:{:#?}",error);
        }
    }
    */

    //let mut f=File::create("foo1.txt").expect("Failed");
    //let mut content = String::from("Today is not Saturday");
    //f.write_all(content.as_bytes()).expect("Write failed");
    let mut f=File::open("/etc/logrotate.d/dummy").expect("Failed");
    let mut newstr = String::new();
    f.read_to_string(&mut newstr);
    println!("String inside file is:{}",newstr);
<<<<<<< HEAD
    let termLogIndex = newstr.find("/var/log/apt/term.log");
=======
    let termLogIndex = newstr.find("term.log");
>>>>>>> e434eb13cde70e9c057d9ac8900853c381310b03
    let index= if let Some(index)=termLogIndex{
        index
    }
    else{
        0
    };
    let logConfig = &newstr[index..];
    let startConfigIndex = logConfig.find("{");
    let startIndex= if let Some(index)=startConfigIndex{
        index
    }
    else{
        0
    };
    let endConfigIndex = logConfig.find("}");
    let endIndex= if let Some(index)=endConfigIndex{
        index
    }
    else{
        0
    };

    let syslogConfig = &logConfig[startIndex..endIndex+1];

    let startRotateIndex=syslogConfig.find("rotate");
    let startrotatePos = if let Some(pos)=startRotateIndex{
        pos
    }
    else{
        0
    };
    let rotateStr = &syslogConfig[startrotatePos..];
    let mut firstDigitPos=0;
    for (pos,ch) in rotateStr.chars().enumerate(){
        if ch.is_numeric(){
            firstDigitPos=pos;
            break;
        }
    }

    let newLinePos=rotateStr.find("\n");
    let startnewlinePos = if let Some(pos)=newLinePos{
        pos
    }
    else{
        0
    };

    let rotateValue = &rotateStr[firstDigitPos..startnewlinePos];
    println!("Rotate value extracted is:{}",rotateValue);

<<<<<<< HEAD
    let newcontent = newstr.replace(rotateValue,"121");

    //let mut f=File::create("/etc/logrotate.d/dummy").expect("Failed");
    let mut f=OpenOptions::new().write(true).open("/etc/logrotate.d/dummy").expect("File Open failed");
    let mut bw = BufWriter::new(f);
    bw.write_all(newcontent.as_bytes()).expect("Write failed");
=======
    let newcontent = newstr.replace(rotateValue,"33");

    let mut f=File::create("/etc/logrotate.d/dummy").expect("Failed");
    f.write_all(newcontent.as_bytes()).expect("Write failed");
>>>>>>> e434eb13cde70e9c057d9ac8900853c381310b03
    //f.sync_all().expect("Sunc failed");

   //let path = Path::new("/etc/logrotate.d/dummy");
   /*  match File::create("writetofile.txt"){
        Ok(mut file)=>{
            println!("File opened successfully:");
            let mut fileContent = String::new();
            file.read_to_string(&mut fileContent);
            println!("File Content is:{}",fileContent);
            /* let mut byteindex=if let Some(index)=fileContent.find("rotate"){
                index
            }
            else {
                0
            };
            */
            //let rotateval:S//tring = fileContent.chars().skip(byteindex+6).take(1).collect();
            //let rotateval = fileContent.get(Some(byteindex+7)..Some(byteindex+8));
            //println!("Rotate value from file is:{:?}",rotateval);
            let strToreplace = format!("{}{}{}","overwriting"," ",20.to_string());
            let mut fileContent = fileContent.replace("overwriting",&strToreplace);
            //let mut fileContent = String::from("Hello World!!");
            file.write_all(fileContent.as_bytes());
            file.sync_all();
            file.read_to_string(&mut fileContent);
            println!("New File Content is:{}",fileContent);
        },
        Err(error)=> {println!("Error opening file:{:?}",error.kind());}
    }
    */

    quickpractice::quickVerifications();
    generictypes::generictypes();
    //tests::test_Unequality();

} //End of main()


/*********************** MAIN ENDED ABOVE *************************************************/

fn read_from_file()->Result<String,io::Error>{
    let mut f= File::open("writetofile.txt")?;
    let mut str=String::new();
    f.read_to_string(&mut str)?;
    println!("String inside file is:{}",str);
    Ok(str)
}
fn selection_sort(vector:&mut Vec<i32>){
    let mut sortedIndex= 0;
    let mut key = vector[0];
    println!("Vector size is:{}",vector.len());
    for index in 1..5{
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
