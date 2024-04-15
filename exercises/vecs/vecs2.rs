// vecs2.rs
//
// A Vec of even numbers is given. Your task is to complete the loop so that
// each number in the Vec is multiplied by 2.
//
// Make me pass the test!
//
// Execute `rustlings hint vecs2` or use the `hint` watch subcommand for a hint.



fn vec_loop(mut v: Vec<i32>) -> Vec<i32> {
    for element in v.iter_mut() {
        // TODO: Fill this up so that each element in the Vec `v` is
        // multiplied by 2.
        *element *= 2; 
    }

    // At this point, `v` should be equal to [4, 8, 12, 16, 20].
    v
}
// 这段代码是一个函数 vec_map，接受一个不可变的 Vec<i32> 引用作为参数，并返回一个新的 Vec<i32>。在函数中，使用了 iter() 方法来遍历原始向量的每个元素，然后通过 map 方法对每个元素进行转换操作。
// 在 map 的闭包中，每个元素都会被乘以2，表示对原始向量中的每个元素进行了加倍处理。最后，collect() 方法将经过处理后的元素收集到一个新的向量中，并将其作为函数的返回值。
// 因此，这段代码的作用是将输入的整数向量中的每个元素都加倍，并返回一个包含所有加倍后元素的新向量。
fn vec_map(v: &Vec<i32>) -> Vec<i32> {
    v.iter().map(|element| {
        // TODO: Do the same thing as above - but instead of mutating the
        // Vec, you can just return the new number!
        2 * element
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec_loop() {
        let v: Vec<i32> = (1..).filter(|x| x % 2 == 0).take(5).collect();
        let ans = vec_loop(v.clone());

        assert_eq!(ans, v.iter().map(|x| x * 2).collect::<Vec<i32>>());
    }

    #[test]
    fn test_vec_map() {
        let v: Vec<i32> = (1..).filter(|x| x % 2 == 0).take(5).collect();
        let ans = vec_map(&v);

        assert_eq!(ans, v.iter().map(|x| x * 2).collect::<Vec<i32>>());
    }
}
