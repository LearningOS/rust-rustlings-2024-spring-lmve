// options3.rs
//
// Execute `rustlings hint options3` or use the `hint` watch subcommand for a
// hint.



struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let y: Option<Point> = Some(Point { x: 100, y: 200 });

    match y {
        Some(ref p) => println!("Co-ordinates are {},{} ", p.x, p.y),
        _ => panic!("no match!"),
    }
    /*
     * ref 是rust的一个关键字，在模式匹配（match、if let、while let等）中，
     * 创建对匹配项内部某个值的引用。
     * 1. 避免值复制
     * 2. 保持借用生命周期
     * 3. 多层嵌套引用的解引用
     * 
     */
    y; // Fix without deleting this line.
}
