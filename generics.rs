use std::ops::Mul;


fn main(){
    println!("{}",multiplygen(20.0,30.0));
    println!("{}",multiplygen(-1,-3));
}

fn multiplygen<T:Mul<Output=T>>(a:T,b:T)->T{
    return a*b;
}