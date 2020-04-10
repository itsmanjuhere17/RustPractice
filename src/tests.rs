use std::cmp::Ordering;
pub fn strCompare(first:String , second:String)->bool{
    if let Ordering::Equal =first.cmp(&second){
        true
    }
    else {
        false
    }
}

#[cfg(test)]
#[test]
pub fn test_Unequality(){
    let testStr1 = String::from("Manjunath");
    let testStr2 = String::from("Manjunath1");
    assert_eq!(strCompare(testStr1,testStr2),false,"Strings passed are equal"); //we can pass custom message as well.
}

#[test]
pub fn test_Equality(){
    let testStr1 = String::from("Manjunath");
    let testStr2 = String::from("Manjunath");
    assert_eq!(strCompare(testStr1,testStr2),true);
}