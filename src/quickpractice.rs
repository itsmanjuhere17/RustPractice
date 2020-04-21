use std::process;
#[derive(Debug)]
pub struct Sample{
    name:String,
    age:u8
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
pub fn quickVerifications(){
    println!("#### INSIDE quickVerification function ####");
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

    let ret = process::Command::new("sudo").arg("vi /etc/logrotate.d/dummy").output();
    println!("{:#?}",ret);

    let samp = Sample::new(0);
    println!("Sample struct is:{:#?}",samp);

    let ((),y)=((),true);
    if y{
        println!("It is true");
    }
    else{
        println!("It is false");
    }

    println!("#### EXITING quickVerification function ####");
}

#[allow(dead_code)]
pub fn deadFun(){
    println!("I am dead function");
}