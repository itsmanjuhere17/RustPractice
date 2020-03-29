pub fn quickVerifications(){
    println!("#### INSIDE quickVerification function ####");
    let mut v = [5, 4, 1, 3, 2];
    v.sort_by(|a, b| a.cmp(b));
    println!("Vector elements are:");
    for ele in &v{
        println!("{}",ele);
    }

    let mut x=10;
    let mut y=&mut x;
    *y=100;
    //println!("x is {} , y is {}",x,y); Note: I need to check this.
    //let mut z=&mut y;

    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }

    println!("{}",format!("{:<11}","Manjunath"));
    println!("#### EXITING quickVerification function ####");
}