struct Foo {
    x: (u32, u32),
    y: u32
}

fn main() {
    let foo = Foo { x: (1, 2), y: 3 };
    let Foo { x: (a, b), y } = foo;

    println!("a = {}, b = {},  y = {} ", a, b, y);

    let Foo { y: i, x: j } = foo;
    println!("i = {:?}, j = {:?}", i, j);

    let Foo { y, .. } = foo;
    println!("y = {}", y);
}
