use crate::smartpointers::List::{Cons,Nil};
use std::ops::Deref;
enum List{
    Cons(i32,Box<List>),
    Nil,
}

#[derive(Debug)]
struct MyBox<T:std::fmt::Debug>(T);

impl<T:std::fmt::Debug> MyBox<T>
{
    fn new(x:T)->MyBox<T>{
        MyBox(x) //It not a heap type. Its the pre-condition as per example.
    }
}

impl<T:std::fmt::Debug> Deref for MyBox<T>
{
    type Target = T;
    fn deref(&self)->&T{
        &self.0
    }

}

impl<T:std::fmt::Debug> Drop for MyBox<T>
{
    fn drop(&mut self){
        println!("Dropping T{:?}",*self);
    }
}

pub fn smart_pointers(){
    println!("############### INSIDE SMART POINTERS #####################");
    //Box type.
    let b;
    {
        b= Box::new(10);
        println!("Box value is:{}",b);
    }//Data on heap gets deleted after this.
    println!("Outer Box value is:{}",b); //b still holds the value.
    //Box on recursive types.
    let list = Cons(1,Box::new(Cons(2,Box::new(Cons(3,Box::new(Nil))))));
    //References.
    let x=10;
    let y=&x;
    println!("x is:{} , ref y is:{}",x,*y);

    let y = MyBox::new(100);
    println!("Y deref is:{}",*y); //Internally called as *(y.deref())
    let strin = MyBox::new(String::from("Manjunath"));
    dummy(&strin); //This is called "Deref coercion". Rust automatically converts ref to type that matches the fun signature. This works on the types that implement Deref trait.
    //No runt time penalty.It resolves at compile time.
    let intref = MyBox::new(121); //First it will be dropped.
    //dummy(&intref); //Throws error as types are in-compatible.
    drop(intref); //Calling drop before hand. Early drop. Defined in std::me,::drop
    let strref = MyBox::new("Manjugadu");
    dummy(&strref);
    println!("############### EXITING SMART POINTERS #####################");
}

fn dummy(param:&str){
    println!("Inside dummy:{}",param);
}