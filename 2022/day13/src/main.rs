use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let mut lines = input.lines();
    let mut correct_pairs: Vec<usize> = vec![];
    let mut current_pair = 0;
    loop {
        let left = match lines.next() {
            Some(line) => line,
            None => break,
        };
        let right = lines.next().unwrap();
        lines.next();

        parse_line(left);

    }

}

fn parse_line(line: &str) -> Vec<i32> {
    let current_list = &mut list;
    let n_lists = line.chars().clone().filter(|c| c == &'[').count();
    let mut list: Vec<i32> = vec![vec![]]
    println!("{}, {}", line, n_lists);


    // loop {
    //     match chars.next() {
    //         Some(c) => match c {
    //             '[' => {
    //                 let new_list:Vec<T> = Vec::new();
    //                 current_list.push(new_list);
    //             },
    //             _ => {},
    //         },
    //         None => break,
    //     };
    // }
    // println!("{:?}", list);
    list
}