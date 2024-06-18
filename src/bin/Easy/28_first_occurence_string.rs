fn str_str(haystack: String, needle: String) -> i32 {
    if needle.is_empty() {
        return 0;
    }
    let haystack = haystack.as_bytes();
    let needle = needle.as_bytes();
    for i in 0..haystack.len() {
        if i + needle.len() > haystack.len() {
            return -1;
        }
        let mut j = 0;
        while j < needle.len() && haystack[i + j] == needle[j] {
            j += 1;
        }
        if j == needle.len() {
            return i as i32;
        }
    }
    -1 as i32
}


fn main(){
    let haystack = "hello".to_string();
    let needle = "ll".to_string();
    let result = str_str(haystack, needle);
    println!("{:?}", result);
    let haystack = "aaaaa".to_string();
    let needle = "bba".to_string();
    let result = str_str(haystack, needle);
    println!("{:?}", result);
    let haystack = "".to_string();
    let needle = "".to_string();
    let result = str_str(haystack, needle);
    println!("{:?}", result);
    let haystack = "a".to_string();
    let needle = "".to_string();
    let result = str_str(haystack, needle);
    println!("{:?}", result);
    let haystack = "mississippi".to_string();
    let needle = "issip".to_string();
    let result = str_str(haystack, needle);
    println!("{:?}", result);
    let haystack = "mississippi".to_string();
    let needle = "issipi".to_string();
    let result = str_str(haystack, needle);
    println!("{:?}", result);
}