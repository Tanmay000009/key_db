fn main() {
    let mut args = std::env::args().skip(1);

    // Unwrap makes sure that we pass a 2nd param, if not it'll crash the program
    let key = args.next().unwrap();
    let value = args.next().unwrap();
    let contents = format!("{}\t{}\n", key, value);
    std::fs::write("kv.db", contents).unwrap();

    let db = Database::new().expect("Creating DB failed!");
}

use std::collections::HashMap;
struct Database {
    row: HashMap<String, String>,
}

// impl means implmentations, so adding few implementations for the struct
impl Database {
    fn new() -> Result<Database, std::io::Error> {
        let mut map = HashMap::new();

        //  1) Read the kv.db file

        // let contents = match std::fs::read_to_string("kv.db") {
        //     Ok(c) => c,
        //     Err(e) => return Err(e),
        // };
        // Above can be re-written as
        let contents = std::fs::read_to_string("kv.db")?;

        for line in contents.lines() {
            let (key, value) = line.split_once('\t').expect("Corrupt database");
            map.insert(key.to_owned(), value.to_owned());
        }

        // parse the string

        // populate our map
        Ok(Database { row: map })
    }
}
