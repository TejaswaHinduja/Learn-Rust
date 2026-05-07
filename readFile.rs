/*Lets read a file in rust
So we need to build a function that opens a file and then reads it*/

use std::fs;

fn main(){
    let content=fs::read_to_string("hello.txt");
    println!("{}",content)
}
/* why does this give out this error
error[E0277]: `Result<String, std::io::Error>` doesn't implement `std::fmt::Display`
 --> readFile.rs:8:19
  |
8 |     println!("{}",content)
  |               --  ^^^^^^^ `Result<String, std::io::Error>` cannot be formatted with the default formatter
  |               |
  |               required by this formatting parameter
  |
  = help: the trait `std::fmt::Display` is not implemented for `Result<String, std::io::Error>`
  = note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead


So instead lets do matching then?
    let content=fs::read_to_string("hello.txt");
    match content{
    Ok(c)=>{
    println!("{}",c)};
    }
    Err(e)=>{
    println!("{}",e)}
/*
Another way of doing this can be let content=fs::read_to_string("hello.txt").unwrap();
println!("{}",content)

/*fn main(){
    let content=fs::read_to_string("hello.txt").expect("should read it ");
    println!("{}",content)
}*/
