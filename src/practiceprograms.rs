use std::thread;
use std::time::Duration;
use std::sync::{Arc,Mutex};
const RANGE:u32 = 20;
use std::io;
use std::num;
use std::convert::TryInto;
use std::cmp::Ordering;

//Binary Search Area.
#[derive(Eq)] //Its mandatory.
struct  Employee{
    emp_id:u32,
    emp_name:String,
    dept: String
}

impl Employee{
    fn new(emp_id:u32,emp_name:String,dept:String)->Self{
        let emp = Employee{
            emp_id,
            emp_name,
            dept
        };
        emp
    }
}

impl Ord for Employee{
    fn cmp(&self, other:&Self)->Ordering{
        self.emp_id.cmp(&other.emp_id)
    }
}

impl PartialOrd for Employee{
    fn partial_cmp(&self,other:&Self)->Option<Ordering>{
        Some(self.cmp(other))
    }
}

impl PartialEq for Employee{
    fn eq(&self,other:&Self)->bool{
        self.emp_id == other.emp_id
    }
}

pub fn practice_programs(){
    println!("@@@@@@@@@@@@@@@@@@ INSIDE PRACTICE PROGRAMS @@@@@@@@@@@@@@@@@@@@@@@@@@@");
    let thread1 = thread::spawn(||{
        for i in 0..RANGE{
            if i%2 == 0 {
                println!("{} is Even number", i);
            }
            thread::sleep(Duration::from_millis(1));
        }
    });
    let thread2 = thread::spawn(||{
        for i in 0..RANGE{
            if i%2 == 1 {
                println!("{} is Odd number", i);
            }
            thread::sleep(Duration::from_millis(1));
        }
    });
    thread1.join().unwrap();
    thread2.join().unwrap();

    //Leap year.
    println!("Enter the year");
    let mut input_year = String::new();
    io::stdin().read_line(&mut input_year).expect("Reading input failed");

    loop{
        let mut year:u32 = if let Ok(year) = input_year.trim().parse::<u32>(){
            year
        }
        else{
            println!("Could not parse the input. Please give a valid year");
            continue;
        };
        if year%4 == 0{
            if year <100 {
                println!("{} is a leap year",year);
            }
            else {
                if year % 100 == 0 {
                    if year < 400 {
                        println!("{} is not a leap year", year);
                    } else if year % 400 != 0 {
                        println!("{} is not a leap year",year);
                    }
                    else{
                        println!("{} is a leap year",year);
                    }
                }
                else{
                    println!("{} is a leap year",year);
                }
            }
        }
        else{
            println!("{} is not a leap year",year);
        }
        break;
    }

    //String reversal.
    println!("Enter Input String");
    let mut input_str = String::new();
    let mut reverse_str =  String::new();
    io::stdin().read_line(&mut input_str).expect("Failed to read a line");
    let mut vecChars = Vec::new();
    for ch in input_str.chars(){
        vecChars.push(ch);
    }
    vecChars.reverse();
    for ch in vecChars{
        reverse_str.push(ch);
    }
    println!("Reversed string is:{}",reverse_str);

    //Armstrong number
    println!("Enter a input number");
    let mut input_number = String::new();
    io::stdin().read_line(&mut input_number).expect("Failed to read the input");
    let len = input_number.trim().chars().count();
    println!("Length of string is:{}",len);
    let mut Sum:u32 = 0;
    for ch in input_number.trim().chars(){
        let digit = ch.to_digit(10).unwrap();
        Sum = Sum + digit.pow(len.try_into().unwrap());
    }
    let armstrong_number = Sum.to_string();
    if armstrong_number == input_number.trim(){
        println!("Given Number is armstrong number");
    }
    else{
        println!("Given Number is not a armstrong number");
    }

    //Binary Search.
    println!("Binary Serach Algorithm");
    let mut int_list = vec![2,1,4,3,7,5,6];
    if find_element_bin_search(&mut int_list,6){;
        println!("Element found");
    }
    else{
        println!("Element not found");
    }
    let mut str_list = vec!["Manjunath"];
    if find_element_bin_search(&mut str_list, "Manjunath"){
        println!("Element found");
    }
    else{
        println!("Element not found");
    }
    let mut str_list = vec![String::from("Manjunath"),String::from("Pawan Kalyan"),String::from("Mahesh babu"),String::from("Rajani Kanth")];
    if find_element_bin_search(&mut str_list, String::from("Rajani Kanth")){
        println!("String found");
    }
    else{
        println!("String not found");
    }
    let mut emp_list = vec![Employee::new(100,"Manjunath".to_string(),"computers".to_string()),Employee::new(103,"Prathyusha".to_string(),"electronics".to_string()),Employee::new(102,"Josnika".to_string(),"computers".to_string()),Employee::new(101,"Geethu".to_string(),"arts".to_string())];
    if find_element_bin_search(&mut emp_list, Employee::new(105,"Josnika".to_string(),"computers".to_string())){
        println!("Employee found");
    }
    else{
        println!("Employee not found");
    }

    println!("@@@@@@@@@@@@@@@@@@ EXITING PRACTICE PROGRAMS @@@@,@@@@@@@@@@@@@@@@@@@@@@");

}

fn find_element_bin_search<T>(list:&mut Vec<T>,ele:T)->bool
    where T:std::cmp::Ord{
    list.sort();
    let mut start_index:u32 = 0;
    let mut end_index = list.len() as u32-1;
    while start_index<= end_index{
        if start_index == end_index{
            if ele == list[start_index as usize]{
                return true;
            }
            return false;
        }
        else{
            let mut mid_index:u32 = (start_index + end_index)/2;
            if ele > list[mid_index as usize]{
                start_index = mid_index + 1;
            }
            else if ele < list[mid_index as usize]{
                end_index = mid_index-1;
            }
            else{
                return true;
            }
        }
    }
    false
}