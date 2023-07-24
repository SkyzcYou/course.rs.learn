fn main() {
    let x = 5;  // 将 5 绑定到变量 x
    let _y = x;  // 拷贝 x 的值赋给 y
    // 都是基本数据类型，固定大小简单值，通过自动拷贝方式赋值，保存在栈中，无需在堆上分配内存
    // =》 基本数据类型都是通过自动拷贝的方式来赋值的

    let s1 = String::from("hello");
    let _s2 = s1; // 浅拷贝
    // String 是一个复杂类型，包含 {存储在栈中的堆指针、字符串长度、字符串容量}
    // s1 赋予给 s2 之后，Rust 认为 s1 不再有效，也就是 String 的所有权转移给了 s2
    // println!("{}, world!",s1);

    str_prt();
    str_clone();

    let ss = String::from("hello");
    takes_ownership(ss);
    // println!("{}", ss);

    let x = 5;
    makes_copy(x);
    println!("{}", x);

}

fn str_prt(){
    let x : &str = "hello, world.";
    let y = x;
    println!("{}, {}", x, y);
}

fn str_clone(){
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);
}

fn takes_ownership(some_string:String){
    println!("{}", some_string);
}

fn makes_copy(some_integer:i32){
    println!("{}", some_integer);
}