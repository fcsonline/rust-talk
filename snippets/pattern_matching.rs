struct Badger {
    pub age: u8
}

fn main() {
    let badger_john = Badger { age: 8 };

    match badger_john.age {
        baby_age @ 0...1 => println!("John is {} years old baby", baby_age),
        young_age @ 2...4 => println!("John is {} years old young", young_age),
        adult_age @ 5...10 => println!("John is {} years old adult", adult_age),
        old_age => println!("John is {} years old", old_age),
    }
}
