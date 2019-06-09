let path = Path::new("hello.txt");

let mut file = match File::open(&path) {
    Err(why) => {
        panic!("couldn't open file: {}", why.description())
    },
    Ok(file) => file,
};
