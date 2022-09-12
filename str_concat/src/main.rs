fn main(){
    let first_name: &str="John";
    let last_name:&str="Doe";
    let full_name:String=[first_name.to_string()," ".to_string(),last_name.to_string()].concat();  //converting string slice to String
    println!("Full Name is {}",full_name);
}