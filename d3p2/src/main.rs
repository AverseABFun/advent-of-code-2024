use regex::Regex;

fn main() {
    let data =
        String::from_utf8(std::fs::read("input")
            .expect("unexpected error"))
            .expect("file not UTF-8")
            .replace("\r", "");
    let regex = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();
    let mut cache = String::from("");
    let mut mul_enabled = true;
    let mut out = 0;
    for c in data.chars() {
        cache = cache+&String::from(c);
        if c == ')' {
            if cache.ends_with("do()") {
                mul_enabled = true;
            } else if cache.ends_with("don't()") {
                mul_enabled = false;
            } else if regex.is_match(&cache) && mul_enabled {
                let capture = regex.captures(&cache).unwrap();
                let c1 = capture.get(1).expect("cannot find capture group 1").as_str();
                let c2 = capture.get(2).expect("cannot find capture group 2").as_str();
                
                let n1 = c1.parse::<u32>().unwrap();
                let n2 = c2.parse::<u32>().unwrap();
                out += n1*n2;
            }
            cache = String::from("");
        }
    }
    println!("{}", out)
}
