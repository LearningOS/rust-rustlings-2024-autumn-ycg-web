// macros4.rs
//
// Execute `rustlings hint macros4` or use the `hint` watch subcommand for a
// hint.



#[rustfmt::skip]
#[rustfmt::skip]
macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
    ($val:expr) => {
        println!("Look at this other macro: {}", $val);
    }
}


fn main() {
    my_macro!();       // 调用无参数宏
    my_macro!(7777);    // 调用带参数宏
}
