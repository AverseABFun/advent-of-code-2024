fn search_neighbors(data: String, x: usize, y: usize, line_length: usize, letter: char) -> Vec<(usize, usize, isize, isize)> {
    let mut neighbors = Vec::new();
    
    let num_rows = data.len() / line_length;

    let directions = [
        (-1, -1), (0, -1), (1, -1),  
        (-1,  0),          (1,  0),     
        (-1,  1), (0,  1), (1,  1),      
    ];

    for (dx, dy) in directions.iter() {
        let nx = x as isize + *dx;
        let ny = y as isize + *dy;
        if nx<0 || ny<0 {
            continue;
        }
        
        if nx >= 0 && nx < line_length as isize && ny >= 0 && ny < num_rows as isize {
            let nx = nx as usize;
            let ny = ny as usize;
            if nx>500000 || ny>500000 {
                continue;
            }

            let index = ny * line_length + nx;
            if index < data.len() {
                
                if data.chars().nth(index).unwrap_or('\0') == letter {
                    if nx>500000 || ny>500000 {
                        continue;
                    }
                    neighbors.push((nx, ny, *dx, *dy));
                }
            }
        }
    }
    neighbors
}


fn main() {
    let data =
        String::from_utf8(std::fs::read("input")
            .expect("unexpected error"))
            .expect("file not UTF-8")
            .replace("\r", "");
    let line_length = data.split("\n").next().unwrap().len();
    
    let locations = data.match_indices('X');
    let mut out: usize = 0;
    for (loc, _) in locations {
        let y = loc / line_length;
        let x = loc % line_length;
        let neighbors = search_neighbors(data.clone(), x, y, line_length, 'M');
        for (mut x0, mut y0, x_offset, y_offset) in neighbors {
            y0 = (y0 as isize + y_offset) as usize;
            x0 = (x0 as isize + x_offset) as usize;
            if x0>500000 || y0>500000 {
                continue;
            }
            let loc = (y0 * line_length) + x0;
            if loc>data.chars().collect::<Vec<char>>().len() {
                continue;
            }
            if data.chars().collect::<Vec<char>>()[loc] == 'A' {
                y0 = (y0 as isize + y_offset) as usize;
                x0 = (x0 as isize + x_offset) as usize;
                if x0>500000 || y0>500000 {
                    continue;
                }
                let loc = (y0 * line_length) + x0;
                if loc>data.chars().collect::<Vec<char>>().len() {
                    continue;
                }
                if data.chars().collect::<Vec<char>>()[loc] == 'S' {
                    out += 1;
                }
            }
        }
    }
    println!("{}", out);
}
