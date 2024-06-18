#[derive(Debug)]
struct File;

trait Read {
    fn readj(self: &Self, save_to: &mut Vec<u8>) -> Result<usize, String>;
}


impl Read for File {
    fn readj(self: &Self, _save_to: &mut Vec<u8>) -> Result<usize, String> {
        println!("reading data from file");
        Ok(0)
    }
}

fn main(){
    let f = File{};
    let mut buffer: Vec<u8> = vec![];
    let n_byte = f.readj(&mut buffer).unwrap();
    println!("{} byte(s) readj from {:?}", n_byte, f);

}