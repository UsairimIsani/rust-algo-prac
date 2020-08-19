use std::collections::HashMap;
fn main() {
    let mut new_hashmap = HashMap::new();
    new_hashmap.insert("hello",2);
    let val = new_hashmap.values();
    println!("Hello, world! {:?} ",val);
    new_hashmap.insert("world",1);
    let _chin = Hello{
        name : String::from("Usairim Isani")
    };
    Hello::hello_world();
    
}

struct Hello{
    name:String

}
impl Hello{
            fn hello_world(){
        println!("Hello World")
        }   
}