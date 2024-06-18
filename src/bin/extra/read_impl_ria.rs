#[derive(Debug)]
struct File {
    name: String,
    data: Vec<u8>,
}

impl File {
    fn new(name: &str) -> File {
        File {
            name: String::from(name),
            data: Vec::new(),
        }
    }
}

fn main() {
    let f1 = File::new("f1.txt");

    let f1_name = &f1.name;
    let f1_length = f1.data.len();

    println!("{:?}", f1);
    println!("{} is {} bytes long", f1_name, f1_length );
}