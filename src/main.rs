fn main() {
    let mut  args  = std::env::args().skip(1);

    // Unwrap makes sure that we pass a 2nd param, if not it'll crash the program
    let key = args.next().unwrap();
    let value = args.next().unwrap();
    println!("Key:  {}, value:  {} ",key,value);
}
