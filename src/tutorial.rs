#[derive(Debug)]
struct Bar {
    s: String
}

#[derive(Debug)]
struct Foo<'a> {
    a: i64,
    b: &'a Bar
}

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T
}

pub fn generics() {
    let int_pt = Point { x: 0, y: 0};
    let dbl_pt = Point { x: 3.2, y: -9.012 };

    println!("{:?}", (int_pt.x, int_pt.y));
    println!("{:?}", (dbl_pt.x, dbl_pt.y));
}

pub fn lib() {
    let ss = "Meh?".to_string();
    let b = Bar { s: ss };
    let f = Foo { a: 0xffffff, b: &b };
    println!("{}", f.b.s);
    println!("{}", f.a);

    let a : [i64; 3] = [1, 2, 3];
    for (index, n) in a.iter().enumerate() {
        println!("a[{}] = {}", index, n);
    }

    let mut v = Vec::new();
    v.push(12);
    v.push(-438);

    match v.pop() {
        Some(top) => println!("top: {}", top),
        None => ()
    }

    let point = (3, 5);
    match point {
        (0, 0) => println!("origin"),
        (_, 0) => println!("x-axis"),
        (0, _) => println!("y-axis"),
        _ => println!("{:?}", point)
    }

    generics();
}