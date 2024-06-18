use rand::{self, Rng};

static mut ERROR: isize = 0;

struct File;


#[allow(unused_variables)]
fn read(f: &File, save_to: Vec<u8>) -> usize {
    if rand::thread_rng().gen_bool(10_000 as f64) {
        unsafe {
            ERROR = 1;
        }
    }
    0
}

#[allow(unused_mut)]
fn main() {
    let mut f = File;
    let mut buffer: Vec<u8> = vec![];

    read(&f, buffer);
    unsafe {
        if ERROR != 0 {
            println!("An error has occurred");
        }
    }

}