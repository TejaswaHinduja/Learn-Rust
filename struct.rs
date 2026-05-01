struct Person{
    name: String,
    age: u32
}


fn AgeTwice(p:&Person)->u32{
    return p.age*2;
}

fn main(){
    let user=Person{
        name:String::from("tejas"),
        age:20
    };
    let doubleage=AgeTwice(&user);
    println!("{}",doubleage);
    println!("{}",user.name);
    
}
