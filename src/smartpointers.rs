use crate::smartpointers::List::{Cons,Nil};
use std::ops::Deref;
use std::rc::Rc;
use std::cell::RefCell;
#[derive(Debug)]
enum List{
    Cons(i32,Rc<List>),
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
        println!("Dropping==> {:?}",*self);
    }
}

//RefCell and Interior mutability
enum List1{
    Cons(Rc<RefCell<i32>>,Rc<List>), //Declaring RefCell here so that we can mutate it thought Rc has benn declared outward.
    Nil,
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
    //let list = Cons(1,Box::new(Cons(2,Box::new(Cons(3,Box::new(Nil))))));
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
    drop(intref); //Calling drop before hand. Early drop. Defined in std::mem::drop
    let strref = MyBox::new("Manjugadu");
    dummy(&strref);

    //Reference Counting.
    let lista = Rc::new(Cons(1,Rc::new(Cons(2,Rc::new(Nil)))));
    println!("Reference count is:{}",Rc::strong_count(&lista));
    {
        let listb = Cons(2,Rc::clone(&lista)); //Here, Rc::clone() does not clone the data. It just increments the reference counting.
        println!("Reference count is:{}",Rc::strong_count(&lista));
        //println!("Reference count is:{}",Rc::strong_count(&listb));

    }
    let listc = Cons(3,Rc::clone(&lista));
    println!("Reference count is:{}",Rc::strong_count(&lista));
    println!("Lista is:{:#?}",lista);

    //RefCell<T> and Interior Mutability Pattern
    //let value = Rc::new(RefCell::new(5));
    //let lista = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));



    println!("############### EXITING SMART POINTERS #####################");
}

fn dummy(param:&str){
    println!("Inside dummy:{}",param);
}