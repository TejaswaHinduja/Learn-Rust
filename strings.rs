/* Lets understand String vs &str 
    But first we should know what is str? So str is a sequence of utf-8 bytes , it is unsized
    String is the most commonly used type for strings, It is stored in the heap and has the ownership over the actual text.
    String has three components -> (pointer, length , capacity)
    Length is the no. of bytes in buffer, capacity is the size of the buffer
*/
fn main(){
    movedirecton(String::from("up"));
}

fn movedirecton(dir:String){
    if dir=="up"{
        print!("move up");
    }
}
/*Now lets get to &str, 
    The actual raw text  or "str" is not usable directly bcz it is unsized
    &str has two components ->(pointer,length)
    Unlike String it does not have the ownership over the actual text,it is borrowed 
    Think of it like a read only view into the string data(utf-8 bytes)
*/
fn main(){
    movedirecton("up");
}

fn movedirecton(dir:&str){
    if dir=="up"{
        print!("move up");
    }
}


/*Resources I reffered to
https://users.rust-lang.org/t/understanding-when-to-use-string-vs-str/103746/2
https://doc.rust-lang.org/std/string/struct.String.html#representation
https://doc.rust-lang.org/book/ch04-03-slices.html 
 */