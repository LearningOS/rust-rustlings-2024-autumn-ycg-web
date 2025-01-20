// clippy1.rs
//
// The Clippy tool is a collection of lints to analyze your code so you can
// catch common mistakes and improve your Rust code.
//
// For these exercises the code will fail to compile when there are clippy
// warnings check clippy's suggestions from the output to solve the exercise.
//
// Execute `rustlings hint clippy1` or use the `hint` watch subcommand for a
// hint.


use std::f32;

fn main() {
    let pi = f32::consts::PI;  // 使用更精确的 PI 常量
    let radius: f32 = 5.0;  // 明确指定类型为 f32

    let area = pi * radius.powi(2);  // 使用 powi 方法计算平方

    println!(
        "The area of a circle with radius {:.2} is {:.5}!",
        radius, area
    );
}
