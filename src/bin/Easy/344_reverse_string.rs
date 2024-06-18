
fn reverse_string(s: &mut Vec<char>) {
    let length = s.len();

    for i in 0..s.len() {
        if i >= length / 2 {
            break;
        }
        s.swap(i, length - i - 1);
    }

}

fn main() {
    let mut s = vec!['h', 'e', 'l', 'l', 'o'];
    reverse_string(&mut s);
    println!("{:?}", s);
}