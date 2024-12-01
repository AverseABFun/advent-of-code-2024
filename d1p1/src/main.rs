use std::fs::read;

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
    numbers0.sort_unstable();
    numbers1.sort_unstable();
    let mut sum = 0;
    for i in 0..numbers0.len() {
        let (n0, n1) = (numbers0[i], numbers1[i]);
        if n0 < n1 {
            sum += n1 - n0
        } else {
            sum += n0 - n1
        }
    }
    println!("{}", sum)
}
