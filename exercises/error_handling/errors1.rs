// errors1.rs
//
// This function refuses to generate text to be printed on a nametag if you pass
// it an empty string. It'd be nicer if it explained what the problem was,
// instead of just sometimes returning `None`. Thankfully, Rust has a similar
// construct to `Result` that can be used to express error conditions. Let's use
// it!
//
// Execute `rustlings hint errors1` or use the `hint` watch subcommand for a
// hint.
/*
 这段注释描述了一个函数的行为：当传入空字符串作为参数时，
 该函数拒绝生成用于打印姓名标签的文本，并仅简单地返回 None 来表示出现了问题。
 然而，这样做并不理想，因为用户无法得知具体发生了什么错误。幸运的是，Rust 提供了一种类似于 Result 的构造，
 可以用来更清晰地表达错误情况。因此，建议使用这个构造改进函数，使其在遇到问题时能提供更具说明性的反馈。

在 Rust 中，Result 类型常用于表示可能成功并返回值（Ok(T)）或失败并返回错误信息（Err(E)）的计算过程。将函数返回类型改为 Result 而非 Option，可以使函数在发生错误时传递具体的错误详情（如自定义错误类型、错误消息等），帮助用户更好地理解问题所在，并据此进行适当的处理。
 */

pub fn generate_nametag_text(name: String) -> Result<String,String> {
    if name.is_empty() {
        // Empty names aren't allowed.
        Err("`name` was empty; it must be nonempty.".to_string())
    } else {
        Ok(format!("Hi! My name is {}", name))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generates_nametag_text_for_a_nonempty_name() {
        assert_eq!(
            generate_nametag_text("Beyoncé".into()),
            Ok("Hi! My name is Beyoncé".into())
        );
    }

    #[test]
    fn explains_why_generating_nametag_text_fails() {
        assert_eq!(
            generate_nametag_text("".into()),
            // Don't change this line
            Err("`name` was empty; it must be nonempty.".into())
        );
    }
}
