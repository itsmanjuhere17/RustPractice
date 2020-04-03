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

    println!("#### EXITING quickVerification function ####");
}

#[allow(dead_code)]
pub fn deadFun(){
    println!("I am dead function");
}