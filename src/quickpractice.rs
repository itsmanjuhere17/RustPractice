pub fn quickVerifications(){
    println!("#### INSIDE quickVerification function ####");
    let mut v = [5, 4, 1, 3, 2];
    v.sort_by(|a, b| a.cmp(b));
    println!("Vector elements are:");
    for ele in &v{
        println!("{}",ele);
    }

}