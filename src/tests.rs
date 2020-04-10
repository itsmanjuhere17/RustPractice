use std::cmp::Ordering;
<<<<<<< HEAD
pub fn strCompare(first:String , second:String)->bool{
=======
fn strCompare(first:String , second:String)->bool{
>>>>>>> e434eb13cde70e9c057d9ac8900853c381310b03
    if let Ordering::Equal =first.cmp(&second){
        true
    }
    else {
        false
    }
}

<<<<<<< HEAD
#[cfg(test)]
#[test]
pub fn test_Unequality(){
    let testStr1 = String::from("Manjunath");
    let testStr2 = String::from("Manjunath1");
=======
#[test]
pub fn test_Unequality(){
    let testStr1 = String::from("Manjunath");
    let testStr2 = String::from("Manjunath");
>>>>>>> e434eb13cde70e9c057d9ac8900853c381310b03
    assert_eq!(strCompare(testStr1,testStr2),false,"Strings passed are equal"); //we can pass custom message as well.
}

#[test]
pub fn test_Equality(){
    let testStr1 = String::from("Manjunath");
    let testStr2 = String::from("Manjunath");
    assert_eq!(strCompare(testStr1,testStr2),true);
}