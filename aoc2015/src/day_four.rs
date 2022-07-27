use md5::{Md5,Digest};  

pub fn check_hash(h:&str,diff:i32) -> bool {
    let mut count = 0;
    for c in h.chars() {
        match c {
            '0' => {
                count+=1;
                if count == diff {return true}
            },
            _ => return false,
        }
    }
    false
}

pub fn find_hash(path:&str) -> String {
    let secret_key = std::fs::read_to_string(path).unwrap();
    let mut counter = 1;
    let mut needle = secret_key.clone();
    loop {
        let mut hasher = Md5::new();
        hasher.update(needle.as_bytes());
        let result = format!("{:x}",hasher.finalize());
        if check_hash(&result,6) {
            return needle;
        }
        counter += 1;
        needle = secret_key.clone();
        needle.push_str(&counter.to_string());
    }




}