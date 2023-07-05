fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    // 变量解构：从一个相对复杂的变量中，匹配出该变量的一部分内容
    // 和 Python Go 中的一个意思
    let (a,mut b):(bool, bool) = (true, false);
    println!("a = {:?}, b = {:?}", a, b);
    b = true;
    assert_eq!(a,b);

}

fn out(){
    // 1.59 版本后，我们可以在赋值语句的左式中使用元组、切片和结构体模式了。
    struct Struct{
        e: i32
    }
    let (a,b,c,d,e);

    (a,b) = (1,2);

    [c,.., d,_] = [1,2,3,4,5];
    Struct{e,..} = Struct{e:5};
    assert_eq!([1,2,1,4,5], [a,b,c,d,e]);
    // 这里用到了模式匹配的一些语法，如果大家看不懂没关系，可以在学完模式匹配章节后，再回头来看。

}
