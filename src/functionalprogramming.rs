struct Cacher<T>
 where T: Fn(u32) -> u32
{
    closure:T,
    value:Option<u32>
}

impl<T> Cacher<T>
    where T: Fn(u32) -> u32
{
    pub fn new(clos:T)->Cacher<T>{
        Cacher{
            closure:clos,
            value:None,
        }
    }

    pub fn getvalue(&mut self,arg:u32)->u32{
        match self.value{
            Some(v)=>v,
            None=>{
                let v= (self.closure)(arg);
                self.value = Some(v);
                v
            }
        }

    }
}

pub fn functionprog(){
    println!("########### Inside Function Programming #######################");
    println!("Dummy value using closure is:{}",dummy());
    let x=10;
    let equal_to= move |z| {z==x};
    println!("Value of x is:{}",x); //Seems, it does not cause problem to access primitive data types after move.
    let y=10;
    assert!(equal_to(y));
    println!("############### Exiting Function Programming ################");
}

pub fn dummy()->u32{
    let mut closure = Cacher::new(|arg|{
       arg
    }); //Declaring a closure with Cacher type.
    closure.getvalue(10) //Calling with a closure.
}