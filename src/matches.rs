pub fn run() {
    println!("matches.rs");
    let x = 1;
    let result = matches!(x, 1);

    let val = Some(4);
    let result = matches!(val, Some(4));

    let num = 10;
    let val: bool = 10 == num;
    let some = Some(10);
    let x = some == Some(10);

    let option = Option::Some(10);
    let result = matches!(option, None);
    println!("result: {}", result);
    let result = Some(10) == None;
    println!("result: {}", result);
}

// CS担当者を引き継いだ、
// 施設がパスワード発行したのか確認したい
// google 認証
// admin 表示
