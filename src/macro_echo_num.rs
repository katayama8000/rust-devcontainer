// make simple macro to echo number
macro_rules! echo_num {
    ($num:expr) => {
        println!("{}", $num);
    };
}

// 複数の引数を取るマクロ
#[macro_export]
macro_rules! echo_nums {
    ($($num:expr), *) => {
        $(
            println!("{}", $num);
        )*
    };
}
pub fn run() {
    println!(
        "{}{}{}macro_echo_num.rs{}{}{}",
        "🦀", "🦀", "🦀", "🦀", "🦀", "🦀"
    );

    echo_num!(1);
    echo_num!(2);
    echo_num!(3);
    echo_num!(4);

    echo_nums!(1, 2, 3, 4);
}
