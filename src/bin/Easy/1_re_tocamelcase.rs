
fn to_camel_case(text: &str) -> String {
    let mut result = String::new();
    let mut cap_flag = false;

    for c in text.chars() {
        if c == '_' || c == '-' {
            cap_flag = true;
        } else {
            if cap_flag {
                result.push(c.to_ascii_uppercase());
                cap_flag = false;
            } else {
                result.push(c);
            }
        }
    }
    result
}

fn main() {
    let text = "the-stealth-warrior";
    let result = to_camel_case(text);
    println!("{:?}", result);
    let text = "the_stealth_warrior";
    let result = to_camel_case(text);
    println!("{:?}", result);
    let text = "the-stealth_warrior";
    let result = to_camel_case(text);
    println!("{:?}", result);
}