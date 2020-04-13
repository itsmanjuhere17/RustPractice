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

#[derive(Debug,PartialEq)]
struct Shoe{
    size:u32,
    model:String
}

pub fn functionprog(){
    println!("########### Inside Function Programming #######################");
    println!("Dummy value using closure is:{}",dummy());
    let x=10;
    let equal_to= move |z| {z==x};
    println!("Value of x is:{}",x); //Seems, it does not cause problem to access primitive data types after move.
    let y=10;
    assert!(equal_to(y));
    //iterators.
    let eleVec = vec![2,3,4,1,5];
    let mut iter_ = eleVec.iter();
    for ele in iter_{
        println!("vector ele is:{}",ele);
    }
    //Methods that consume iterators..
    let eleVec = vec![1,3,6,8];
    let mut newiter = eleVec.iter();
    let resSum:u32 = newiter.sum(); //Here, sum() consumes the iterator by calling next internally
    println!("Sum of vector is:{}",resSum);
    //println!("Using iter_ after sum:{:?}",iter_);

    //Methods that produce other iterators.
    let list = vec![10,30,20,15];
    let newList:Vec<_> = list.iter().map(|x|{
        x*2
    }).collect();
    for ele in newList{
        println!("New list is:{}",ele);
    }

    //  Closures that capture environment.
    let mut shoes = Vec::new();
    shoes.push(Shoe{size:10,model:"sneaker".to_string()});
    shoes.push(Shoe{size:12,model:"flats".to_string()});
    shoes.push(Shoe{size:10,model:"heels".to_string()});
    let filter_shoe = filter_on_shoe_size(shoes,10);
    for shoe in filter_shoe{
        println!("Shoe is:{:?}",shoe);
    }

    let sampList = vec![11,21,2,26,4,18,20,13];
    let mut new_samp_list:Vec<_> = sampList.iter().map(|ele| ele+1 as u32).filter(|ele| *ele%2 == 0).collect();
    println!("New elements which are even numbers are:");
    for new_ele in new_samp_list.iter(){
        println!("{}",new_ele);
    }
    let resSum:u32 = new_samp_list.iter().sum(); //Here,onto_iter will take ownership of vector.
    //println!("vector is:{:#?}",new_samp_list); //Error.
    println!("New Sum is:{}",resSum);
    println!("############### Exiting Function Programming ################");
}

pub fn dummy()->u32{
    let mut closure = Cacher::new(|arg|{
       arg
    }); //Declaring a closure with Cacher type.
    closure.getvalue(10) //Calling with a closure.
}

fn filter_on_shoe_size(shoes:Vec<Shoe>,size:u32)->Vec<Shoe>{
    shoes.into_iter().filter(|s| s.size == size).collect()
}
