fn main(){

    println!("Hello Rust")
}
/* why is the print statement like that ?
Bcz this is a macro and not a standard function , the ! is a indicator which shows that we are calling a macro instead of a normal function
Normal functions only handle a fixed number of arguments with specific types
but for the print thing we may have to print N number of things , so macros can handle variable number of arguments
*/

/* Binding Values to names 
fn main() {
    let x = 42;
    let name = "Rust";
    println!("x = {}", x);
    println!("name = {}", name);
}*/