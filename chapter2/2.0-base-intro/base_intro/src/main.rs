// Rust 程序入口函数 main()， 无返回值
fn main() {
    // 使用 let 来声明变量， 进行绑定， a 是不可变的
    // 此处没有指定 a 的类型， 编译器会自动推断
    let a = 10;
    let b: i32 = 20;
    
    // 这里有两点值得注意：
    // 1. 可以在数值中带上类型:30i32表示数值是30，类型是i32
    // 2. c是可变的，mut是mutable的缩写
    let mut c = 20i32;
    let d = 30_i32;  // 加下划线 提高可读性
    let e = add(add(a,b), add(c,d));

    println!("(a+b) + (c+d) = {}", e);
}

fn add(i:i32, j:i32) -> i32{
    // 注意 在上面的 add 函数中，不要为 i+j 添加 ;，这会改变语法导致函数返回 () 而不是 i32，具体参见语句和表达式。
    i + j
}
