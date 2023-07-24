use num::Complex;

fn main() {
    let a = Complex { re: 2.1, im: -1.2};
    let b = Complex::new(11.1, 22.2);
    let result = a + b;

    println!(" {} + {} ", result.re, result.im);

    let xx = 13.14f32.round();
    println!("{}", xx);
}