use regex::Regex;

fn main() {
    let re = Regex::new("picture").unwrap();
    let quote = "Every face, every shop, bedroom window, public-house, and 
    dark square is a picture feverishly turned--in search of what?
    It is the same with books. What do we seek through millions of pages?";

    for (i,line) in quote.lines().enumerate() {
        match re.find(line) {
            Some(_) => println!("{} and the line number '{}'", line, i+1),
            None => (),
        }
    }
}


/*fn main() {
    let search_term = "picture";
    let quote = "Every face, every shop, bedroom window, public-house, and
    dark square is a picture feverishly turned--in search of what? 
    It is the same with books, what do we seek through millions of pages?";

    for line in quote.lines() {
        if line.contains(search_term) {
            println!("{}", line)
        }
    }
}*/