fn main() {
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);

    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length fo '{}' is {}.", s1, len);


    let mut s11 = String::from("hello");
    change(&mut s11);
    println!("s1 is {}.", s11);


    borrowing_twice();
}

// 不可变引用
fn calculate_length(s: &String) -> usize{  // s 是对 String 的引用
    s.len()
}

fn change(some_string: &mut String){
    some_string.push_str(", world!");
}

fn borrowing_twice(){
    let mut s = String::from("hello");

    let r1 = &mut s;
    // let r2 = &mut s;

    println!("{}", r1);
    let r2 = &mut s;
    println!("{}", r2);
    // println!("{}, {}", r1, r2);
}