// macros4.rs
//
// Execute `rustlings hint macros4` or use the `hint` watch subcommand for a
// hint.


#[rustfmt::skip]
macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
    ($val:expr) => {
        println!("Look at this other macro: {}", $val);
    };
}

fn main() {
    my_macro!();
    my_macro!(7777);
}


/*

在 Rust 中，分号用于结束语句。在宏定义中，每个分支都可以看作是一个语句。添加分号有两个主要目的：

明确结束语句：分号表示宏定义中的语句结束，这样在宏展开时，每个语句都能够被正确地分隔开来。

避免出现意外的行为：如果不添加分号，那么在宏展开时，可能会出现语句之间意外地被连接在一起的情况，导致编译错误或者意料之外的行为。
*/