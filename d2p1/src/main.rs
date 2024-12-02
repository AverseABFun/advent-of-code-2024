fn main() {
    let mut data =
        String::from_utf8(std::fs::read("input")
            .expect("unexpected error"))
            .expect("file not UTF-8");
    data = data.replace("\r", ""); // just in case
    let mut num_unsafe: u32 = 0;
'TopLoop:
    for ele in data.split("\n") {
        let mut data: Vec<u32> = vec![];
        for ele in ele.split(" ") {
            data.push(ele.parse::<u32>().unwrap());
        }
        let mut ascending = false;
        for i in 1..data.len() {
            let d0 = data[i-1];
            let d1 = data[i];
            if i == 1 {
                ascending = d1>d0;
            }
            if ascending && d1 <= d0 {
                num_unsafe += 1;
                continue 'TopLoop;
            } else if !ascending && d1 >= d0 {
                num_unsafe += 1;
                continue 'TopLoop;
            }
            if ascending {
                if d1-d0 > 3 {
                    num_unsafe += 1;
                    continue 'TopLoop;
                }
            } else {
                if d0-d1 > 3 {
                    num_unsafe += 1;
                    continue 'TopLoop;
                }
            }
        }
    }
    println!("{}",1000-num_unsafe);
}
