fn main() {
    let my_name = "Pascal";
    // greet(my_name);

    let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];

    // 
    let mut ss = String::from("hello world");
    let word = first_word(&ss);
    // ss.clear(); // error!

    println!("the first word is : {}", word);
}

fn greet(name: String){
    println!("Hello, {}", name);
}

fn first_word(s: &String) -> &str{
    &s[..1]
}