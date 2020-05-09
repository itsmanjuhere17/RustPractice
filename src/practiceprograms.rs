use std::thread;
use std::time::Duration;
use std::sync::{Arc,Mutex};
const RANGE:u32 = 20;
use std::io;
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



    println!("@@@@@@@@@@@@@@@@@@ EXITING PRACTICE PROGRAMS @@@@@@@@@@@@@@@@@@@@@@@@@@@");

}