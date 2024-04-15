// move_semantics6.rs
//
// You can't change anything except adding or removing references.
//
// Execute `rustlings hint move_semantics6` or use the `hint` watch subcommand
// for a hint.



fn main() {
    let data = "Rust is great!".to_string();

    get_char(data.clone());

    string_uppercase(data);
}

// Should not take ownership
fn get_char(data: String) -> char {
    data.chars().last().unwrap()
}
// 在 Rust 中，String 是一个堆分配的可变数据类型，因此它拥有所有权。当函数参数声明为 &String 时，它只是一个对 String 的引用，而不是拥有 String 的所有权。因此，你不能修改传入的引用本身以更新数据。
// 如果想要接受一个参数并拿到它的所有权，可以使用 String 类型作为参数类型，然后再打印大写字母版本的字符串。
// Should take ownership
fn string_uppercase(mut data: String) {
    data = data.to_uppercase();

    println!("{}", data);
}
