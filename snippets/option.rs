fn main() {
    let number = Some(7);

    match number {
        Some(i) => println!("Matched {:?}!", i),
        None => panic!("Wat")
    }

    let foo = Some("hello");
    let bar:&str = foo.unwrap();

    let yolo = Some("yolo");
    let bar:&str = yolo.expect("Fatal error");
}
