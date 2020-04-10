//use RustPractice;

#[test]
fn testString(){
    let testStr1 = String::from("Manjunath");
    let testStr2 = String::from("Manjunath1");
    assert_eq!(strCompare(testStr1,testStr2),false,"Strings passed are equal"); //we can pass custom message as well.
}