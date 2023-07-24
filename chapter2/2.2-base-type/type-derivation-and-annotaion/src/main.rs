fn main() {
    println!("Hello, world!");
    // let guess = "42".parse().expect("Not a number");
    let a:u8 = 255;
    let b = a.wrapping_add(20);
    println!("{}", b);

    let x = 2.0; // f64
    let y:f32 = 3.0; // f32

    // assert!(0.1+0.2==0.3);
    let _ = (0.1_f64 + 0.2 -0.3).abs() < 0.0000001;

    let abc: (f32, f32, f32) = (0.1, 0.2, 0.3);
    let xyz: (f64, f64, f64) = (0.1, 0.2, 0.3);

    println!("abc (f32)");
    println!("    0.1 + 0.2: {:x}", (abc.0 + abc.1).to_bits());
    println!("          0.3: {:x}", (abc.2).to_bits());

    numberOperations();
    bitsOperations();
    rangeLearn();
    stringLearn();

}

fn numberOperations(){
    let twenty = 20;
    let twenty_one: i32 = 21;
    let twenty_two = 22i32;

    let addition = twenty + twenty_one + twenty_two;

    println!("{} + {} + {} = {}", twenty,twenty_one,twenty_two,addition);

    let one_million: i64 = 1_000_000;
    println!("{}", one_million.pow(2));

    let forty_tows = [
        42.0,
        42f32,
        42.0_f32,
    ];

    println!("{:2}", forty_tows[0]);
}

fn bitsOperations(){
    let a:i32 = 2;
    let b:i32 = 3; // 11
    
    println!("(a&b) = {}", a&b);

    let mut a = a;
    a <<= b;
    println!("(a<<b) = {}", a);

    a >>= b;
    println!("(a>>b) = {}", a);
}

fn rangeLearn(){
    // Rust 提供了一个非常简洁的方式，用来生成连续的数值，例如 1..5，生成从 1 到 4 的连续数字，不包含 5 ；
    // 1..=5，生成从 1 到 5 的连续数字，包含 5，它的用途很简单，常常用于循环中：
    for i in 1..=5{
        println!("{}", i);
    }

    for i in 'a'..='z'{
        println!("{}", i);
    }
}

fn add_with_extra(x:i32, y:i32) -> i32{
    let x = x + 1;
    let y = y + 1;
    x+y
}

fn ret_unit_type() {
    let x = 1;
    // if 语句块也是一个表达式，因此可以用于赋值，也可以直接返回
    // 类似三元运算符，在Rust里我们可以这样写
    let y = if x % 2 == 1 {
        "odd"
    }else {
        "even"
    };
    
    let z = if x % 2 == 1 { "odd" } else { "even" };

}
fn stringLearn(){
    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{}", s);
}