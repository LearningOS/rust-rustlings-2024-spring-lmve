// variables4.rs
//
// Execute `rustlings hint variables4` or use the `hint` watch subcommand for a
// hint.



fn main() {
    /* 不能将 x=3 隐藏起来 ，x 是可变的才能改变 */
    let mut x = 3;
    println!("Number {}", &x);
    x = 5; // don't change this line
    println!("Number {}", x);
}
