// macros3.rs
//
// Make me compile, without taking the macro out of the module!
//
// Execute `rustlings hint macros3` or use the `hint` watch subcommand for a
// hint.




    macro_rules! my_macro {
        () => {
            println!("Check out my macro!");
        };
    }
  


fn main() {
    // 现在可以正常调用宏了
    my_macro!();  // 调用宏，应该输出 "Check out my macro!"
}
