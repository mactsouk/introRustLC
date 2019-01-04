fn main() {
    // Define a str
    let s_1: &str= "Defining strings";
    // Or
    let s_2: &'static str = "Another String";
    // Use format! to create a new String
    let my_str = format!("This is {} {}!", s_1, s_2);
    println!("my_str L:{} C:{}", my_str.len(), my_str.capacity());
}
