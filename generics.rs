use std::ops::Mul;

fn main(){
    println!("{}",multiplygen(20.0,30.0));
    println!("{}",multiplygen(-1,-3));
}

fn multiplygen<T:Mul<Output=T>>(a:T,b:T)->T{
    return a*b;
}

/*What is the need of generics?
Suppose we have to make a function that takes in two arguments and multiplies them and returns the value.
In ts/js you can just do -> 
   function multiply(a,b){
      console.log(a*b)
}
This function works just fine until the input passed is wrong what if we pass a=23 and b="stringg"?This would result in NaN
Well rust is defined for compile time safety and wont let you do this mistake

So Rust says just gimme the type of inputs, 
Sure we can do that sounds simple right?
fn multiply(a:u32,b:32){
return a*b;
}
Can you spot the issue we would face in the future?
We would have to define multiple functions which run the same logic for every data type
Well Generics solve this issue
The compiler doesnt care about the concrete type as long as the inputs satisifies some conditions
