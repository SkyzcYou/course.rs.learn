fn main() {
    // let my_name = "Pascal";
    // greet(my_name);

    let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];

    // 
    let mut ss = String::from("hello world");
    let word = first_word(&ss);
    // ss.clear(); // error!

    println!("the first word is : {}", word);

    let s1 = String::from("hello");
    // let h = s1[0];

    push_string();
    replace_str();
    replace_range_str();
    delete_str();
    remove_str();
    concatenate_str();
    format_str();
    unicode_str();
}

fn greet(name: String){
    println!("Hello, {}", name);
}

fn first_word(s: &String) -> &str{
    &s[..1]
}

fn push_string(){
    let mut s = String::from("Hello ");
    s.push_str("rust");
    println!("追加字符串 push_str() -> {}", s);

    s.push('!');
    println!("追加字符 push() -> {}", s);

    s.insert(5,',');
    println!("插入字符 insert() -> {}", s);

    s.insert_str(6, "I Like ");
    println!("插入字符串 insert_str() -> {}", s);
}

fn replace_str(){
    let string_replace = String::from("I like rust, Learning rust is my favorite!");
    let new_string_replace = string_replace.replace("rust", "RUST");
    dbg!(new_string_replace);
}
 fn replace_range_str(){
    let mut string_replace_range = String::from("i like rust");
    string_replace_range.replace_range(7..8, "R");
    dbg!(string_replace_range);
 }

 fn delete_str(){
    let mut string_pop = String::from("rust pop 中文!");
    let p1 = string_pop.pop();
    let p2 = string_pop.pop();
    dbg!(p1);
    dbg!(p2);
    dbg!(string_pop);
 }

 fn remove_str(){
    let mut string_remove = String::from("测试remove方法");
    println!("string_remove 占用了 {} 个字节",
            std::mem::size_of_val(string_remove.as_str()));
    string_remove.remove(0);
    // 下面代码会报错
    // string_remove.remove(1);
    // string_remove.remove(3);
    dbg!(string_remove);
 }

 fn concatenate_str(){
        let string_append = String::from("hello ");
        let string_rust = String::from("rust");
        // &string_rust 会自动解引用为 &str
        let result = string_append + &string_rust;
        // `result + "!" ` 中的 `result` 是不可变的
        let mut result = result + "!";
        result += "!!!";

        println!("连接字符串 + -> {}", result);

 }

 fn format_str(){
    let s1 = "hello";
    let s2 = String::from("rust");

    let s = format!("{} {}!", s1, s2);
    println!("format!() -> : {}", s);
 }

 fn unicode_str(){
    // 通过 \ + 字符的十六进制表示， 转义输出一个字符
    let byte_escape = "I'm writing \x52\x75\x73\x74";
    println!("What are you doing\x3F (\\x3F means ?) {} ", byte_escape);

    // \u 可以输出一个 unicode 字符
    let unicode_codepoint = "\u{211D}";
    let character_name = "\"DOUBLE-STRUCK CAPITAL R\"";
    println!("Unicode character {} (U+211D) is called {}. ", unicode_codepoint, character_name);

    // 换了行也会保持之前的格式
    // 使用 \ 忽略换行符
    let long_str = "String literals
                    can span multiple lines.
                    The linebreak and indentation here ->\
                    <- can be escape too!";
    println!("{}", long_str);
 }