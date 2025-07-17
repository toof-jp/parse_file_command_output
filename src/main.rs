use std::{collections::HashMap, io::Read};

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    
    let map = parse(&input.trim());
    dbg!(map);
}

fn parse(s: &str) -> HashMap<String, String> {
    let mut map = HashMap::new();
    
    for line in s.lines().map(str::trim) {
        if let Some(idx) = line.rfind(":") {
            let (file_name, mime_part) = line.split_at(idx);
            
            let mime_part = mime_part[1..].trim_start(); // skip ':'
                                                          
            dbg!(file_name);
            dbg!(mime_part);
            map.insert(file_name.into(), mime_part.into());
        }
    }
    map
}