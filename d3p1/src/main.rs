use regex::Regex;

fn main() {
    let data =
        String::from_utf8(std::fs::read("input")
            .expect("unexpected error"))
            .expect("file not UTF-8")
            .replace("\r", "");
    let regex = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();
    let result = regex.captures_iter(&data);
    let mut out: u32 = 0;
    for capture in result {
        let c1 = capture.get(1).expect("cannot find capture group 1").as_str();
        let c2 = capture.get(2).expect("cannot find capture group 2").as_str();
        
        let n1 = c1.parse::<u32>().unwrap();
        let n2 = c2.parse::<u32>().unwrap();
        out += n1*n2;
    }
    println!("{}", out);
}
