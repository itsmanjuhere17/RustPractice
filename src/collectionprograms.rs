use std::collections::*;
pub fn vectorCollectionProgram()
{
    let mut vecInt = Vec::new();
    vecInt.push(10);
    vecInt.push(20);
    vecInt.push(30);
    vecInt.push(5);
    vecInt.push(15);
    let mut SumVect=0;
    for ele in &vecInt{
        SumVect+=ele;
    }
    let average = SumVect/vecInt.len();
    println!("Average value of vector is:{}",average);
    vecInt.sort();
    println!("After Sorting vector elements:");
    for ele in &vecInt{
        println!("{}",ele);
    }
    println!("Median value is:{:#?}",vecInt.get(vecInt.len()/2));
    vecInt.push(10);
    vecInt.push(10);
    vecInt.push(5);
    vecInt.push(10);
    let mut hashMapInt = HashMap::new();
    for ele in &vecInt{
        let counter = hashMapInt.entry(ele).or_insert(0);
        *counter+=1;
    }
    let mut maxValue=0;
    let mut mode =0;
    for (key,value) in &hashMapInt{ //Immutable reference.
        if value>&maxValue{
            maxValue = *value;
            mode=**key;
        }
    }
    println!("Mode value of the integers is:{} with mode count as: {}",mode,maxValue);
}