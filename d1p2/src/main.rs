use std::fs::read;

fn number_times_in(val: u32, vec: Vec<u32>) -> u32 {
    let mut out = 0;
    for ele in vec {
        if ele == val {
            out += 1;
        }
    }
    out
}

fn main() {
    let mut data =
        String::from_utf8(read("input").expect("unexpected error")).expect("file not UTF-8");
    data = data.replace("\r", ""); // just in case
    let mut numbers0: Vec<u32> = [].to_vec();
    let mut numbers1: Vec<u32> = [].to_vec();
    for ele in data.split("\n") {
        let mut it = ele.split("   ");
        let s0 = it.next().unwrap();
        let s1 = it.next().unwrap();

        let n0 = s0.parse::<u32>().unwrap();
        let n1 = s1.parse::<u32>().unwrap();
        numbers0.push(n0);
        numbers1.push(n1);
    }
    let mut similarity = 0;
    for n0 in numbers0 {
        let s = n0 * number_times_in(n0, numbers1.clone());
        similarity += s;
    }
    println!("{}", similarity);
}
