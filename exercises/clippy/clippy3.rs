// clippy3.rs
// 
// Here's a couple more easy Clippy fixes, so you can see its utility.
//
// Execute `rustlings hint clippy3` or use the `hint` watch subcommand for a hint.

// I AM NOT DONE

#[allow(unused_variables, unused_assignments)]
fn main() {
    let mut my_option: Option<()> = None;

    // 使用 is_some() 来检查 Option 是否为 Some
    if my_option.is_some() {
        // 如果是 Some，可以进行相关处理
        println!("Option is Some, but no need to unwrap.");
    } else {
        println!("Option is None.");
    }

    // 修复数组语法错误
    let my_arr = &[
        -1, -2, -3, // 添加逗号
        -4, -5, -6,
    ];
    println!("My array! Here it is: {:?}", my_arr);

    // 使用 clear() 来清空 Vec
    let mut my_empty_vec = vec![1, 2, 3, 4, 5];
    my_empty_vec.clear(); // clear() 更适合清空 Vec
    println!("This Vec is empty, see? {:?}", my_empty_vec);

    // 交换两个值
    let mut value_a = 45;
    let mut value_b = 66;
    // 使用 std::mem::swap 来交换值
    std::mem::swap(&mut value_a, &mut value_b);
    println!("value a: {}; value b: {}", value_a, value_b);
}
