use std::process;
use std::io;
use std::collections::HashMap;
#[derive(Debug)]
pub struct Sample{
    name:String,
    age:u8
}

struct strTuple(u32,u32);

impl strTuple{
    fn create(x:u32,y:u32)->strTuple{
        let tup_str = strTuple(x,y);
        tup_str
    }
}

impl std::fmt::Debug for strTuple{
    fn fmt(&self,format:&mut std::fmt::Formatter<'_>)->Result<(),std::fmt::Error>{
        println!("Values of struct tuple are:{} {}",self.1,self.0);
        Ok(())
    }
}

impl Sample{
    pub fn new(index:u8)->Sample{
        let samp = Sample{
            name:if index==0{
                "Manjunath".to_string()
            }
            else {
                "Prathyusha".to_string()
            },
            age:30
        };
        samp
    }
}

//Ref struct.
#[derive(Debug)]
struct ref_struct<'a>{
    data:&'a str,
    other_data:&'a str,
    value:u32
}

/* impl<'a> ref_struct<'a>{
    fn create()-> ref_struct{
        ref_struct {
            data: "balayya",
            other_data: "bul bul",
            value: 100
        }
    }
}
*/

#[derive(Debug)]
enum Message{
    Quit,
    message(String),
    code(i32)
}

impl Message{
    fn call(&mut self){
        if let Message::message(mess) = &mut *self{
            println!("Message is:{}",mess);
        }
    }
}

struct Dummy_str<T>
    where T:Fn(u32)->u32
{
    closure: T,
    value:Option<u32>,
}

impl<T> Dummy_str<T> where T:Fn(u32)->u32{

    fn new(closure:T)->Dummy_str<T>{
        Dummy_str{
            closure,
            value:None
        }
    }

    fn value(&mut self,value:u32)->u32{
        match self.value {
            Some(value)=>value,
            None =>{
                let v=(self.closure)(value);
                self.value = Some(v);
                v
            }
        }
    }
}


//Function entry point.
pub fn quickVerifications(){
    println!("################ INSIDE quickVerification function #########################");
    let mut v = [5, 4, 1, 3, 2];
    v.sort_by(|a, b| b.cmp(a)); //b.cmp(a) return list in decreasing order and vice-versa.
    println!("Vector elements are:");
    for ele in &v{
        println!("{}",ele);
    }

    let mut x=10;
    let mut y=&mut x;
    *y=100;

    //println!("x is {} , y is {}",x,*y); //Note: I need to check this.
    //let mut z=&mut y;

    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }

    println!("{}",format!("{:<18}{}","Manjunath","is a good boy"));//Including the string Manjunath it takes 18 spaces and then print next string.

    //let ret = process::Command::new("sudo").arg("vi /etc/logrotate.d/dummy").output();
    //println!("{:#?}",ret);

    let samp = Sample::new(0);
    println!("Sample struct is:{:#?}",samp);

    let ((),y)=((),true);
    if y{
        println!("It is true");
    }
    else{
        println!("It is false");
    }
    let samp = "manju";
    println!("Length of samp is:{}",samp.len());
    let samp1 = "manju%?_@$";
    println!("Length of samp1 is:{} {}",samp1.len(),samp1.chars().count());

    let trial = "Manjunathisgood";
    let new_coll = trial.chars().all(|c| c!=' ');
    println!("Trim white spaces:{}",new_coll);

    //struct tuples.
    let tup_str = strTuple(5,6);
    println!("Tuple struct is:{:#?}",tup_str);

    let mut list_str = vec![String::from("Manju"),String::from("Nath")];
    for mut item in list_str{ //Moved here
        item.remove(0);
        println!("Iterator is:{}",item);
    }
    //println!("Printing vector again:{:#?}",list_str); //Error if you un-comment it.

    let mut int_vec = vec![3,2,5,1,6];
    let mut new_int_vecm :Vec<i32> = int_vec.iter().map(|ele| (*ele)*2).collect();

    for new_ele in new_int_vecm.iter(){
        println!("New vector ele is:{}",new_ele);
    }

    let slice_vec = &new_int_vecm[..4];
    println!("Slice vector is:{:#?}",slice_vec);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Reading failed");
    //let input_integ = input.trim().parse().expect("Parsing failed");
    //let sum = 30;

    for item in input.split_whitespace(){
        println!("word is:{}",item);
    }

    //Refernce inside struct.
    /* let struct_ref = ref_struct{
        data: "Hello World",
        other_data: "Its Vappu",
        value:1,
    };
    */

    //let new_struct:_ = ref_struct::create();
    //println!("refernce struct is:{:#?}",struct_ref);

    let mut mess = Message::message(String::from("Manjunath"));
    mess.call();

    let s1 = String::from("Manju");
    let s2 = String::from("Nath");
    //let s3 = s1 + &s2; //Value will be moved here, if you use +.
    //println!("Strings are:{} {} {}",s1,s2,s3); //Throws error.
    println!("{}",format!("{} {}",s1,s2));
    println!("Strings are:{} {}",s1,s2);

    let mut comp_data: HashMap<String,Vec<String>> = HashMap::new();
    let students=comp_data.entry(String::from("Science")).or_insert(Vec::new());
    students.push(String::from("Manju"));
    let eng_students = comp_data.entry(String::from("Engineering")).or_insert(Vec::new());
    eng_students.push(String::from("Josnika"));
    let sci_students = comp_data.entry(String::from("Science")).or_insert(Vec::new());
    sci_students.push(String::from("Prathyusha"));

    for (dept,studs) in &mut comp_data{
        (*studs).sort();
        println!("Dept:{} , Students:{:#?}",dept,studs);
    }

    let mut struc_dummy = Dummy_str::new(|ele| {println!("Inside closure");ele});
    let mut struc_dummy = Dummy_str::new(|ele| {ele*2});
    let value = struc_dummy.value(10);
    let value1 = struc_dummy.value(15); //return is 10 only
    println!("Values are:{} {}",value,value1);

    let vect = vec![2,1,3,4];
    let mut vec_iter = vect.into_iter();
    let sum:u32 = vec_iter.sum();
    println!("Sum is:{}",sum);
    //println!("vector is:{:?}",vect); //into_iter moves the element inside vector.
    //println!("Vec Iter is:{:#?}",vec_iter);

    let mut v1: Vec<i32> = vec![1, 2, 3];
    for iter in v1.iter_mut(){
        *iter*=2;
        println!("iter is:{}",iter);
    }
    let res:i32= v1.iter().map(|ele| *ele*5).sum();
    println!("Sum is:{}",res);
    println!("v1 is:{:?}",v1);


    println!("################# EXITING quickVerification function #############################");
}

#[allow(dead_code)]
pub fn deadFun(){
    println!("I am dead function");
}