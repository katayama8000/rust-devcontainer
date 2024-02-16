pub struct Node {
    data: i64,
    next: Option<Box<Node>>,
}

fn node(v: i64, link: Option<Box<Node>>) -> Option<Box<Node>> {
    Some(Box::new(Node {
        data: v,
        next: link,
    }))
}

pub fn run() {
    println!(
        "{}{}{}box_list.rs{}{}{}",
        "🦀", "🦀", "🦀", "🦀", "🦀", "🦀"
    );

    let c = node(3, node(20, node(30, None))).unwrap();

    let mut p = &c;
    loop {
        println!("{}", p.data);
        match p.next {
            Some(ref n) => p = n,
            None => break,
        }
    }

    // sample Box::pin
    let pin = Box::pin("hello".to_string());
    let val = pin.as_ref();
    println!("{}", val);
}
