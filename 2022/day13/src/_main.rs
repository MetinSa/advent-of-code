use std::{fs, collections::VecDeque};

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let mut lines = input.lines();
    let mut correct_pairs: Vec<usize> = vec![];
    let mut current_pair = 0;
    loop {
        let left_str = match lines.next() {
            Some(x) => x,
            None => break,
        };
        let right_str = lines.next().unwrap();
        lines.next(); // Skip the empty lines

        let left_vec = parse_line(left_str);
        let right_vec = parse_line(right_str);
        let mut left_vec_iter = left_vec.iter();
        let mut right_vec_iter = right_vec.iter();
        current_pair += 1;

        // 'outer: loop {
        //     let left_vec = left_vec_iter.next();
        //     let right_vec = right_vec_iter.next();

            // for (left, right) in left_vec.zip(right_vec) {
            //     println!("{:?}, {:?}", left, right);
            //     panic!();
            // }
            // let left_items = match left_vec {
            //     Some(x) => x,
            //     None => {
            //         correct_pairs.push(current_pair);
            //         break 'outer;
            //     }
            // };
            // let right_items = match right_vec {
            //     Some(x) => x,
            //     None => {
            //         break 'outer;
            //     }
            // };
            // let mut left_items_iter = left_items.iter();
            // let mut right_items_iter = right_items.iter();
            // loop {
            //     let left_item = match left_items_iter.next() {
            //         Some(x) => x,
            //         None => {
            //             correct_pairs.push(current_pair);
            //             break 'outer;
            //         }
            //     };
            //     let right_item = match right_items_iter.next() {
            //         Some(x) => x,
            //         None => {
            //             break 'outer;
            //         }
            //     };
            //     if left_item < right_item {
            //         correct_pairs.push(current_pair);
            //         break 'outer;
            //     } else if left_item > right_item {
            //         break 'outer;
            //     }
            // }
        // }
    }
    println!("{:?}", correct_pairs);
    println!("{:?}", correct_pairs.iter().sum::<usize>());
}

fn parse_line(line: &str) -> VecDeque<Vec<i32>> {
    let mut list: VecDeque<Vec<i32>> = VecDeque::new();
    let mut line = line.to_string();
    loop {
        let stop = match line.find("]") {
            Some(x) => x,
            None => break,
        };
        let start = line[..stop].rfind("[").unwrap();

        if stop == start + 1 {
            list.push_back(vec![]);
        } else {
            list.push_back(
                line[start + 1..stop]
                    .to_string()
                    .split(",")
                    .map(|x| x.parse().unwrap())
                    .collect::<Vec<i32>>(),
            );
        }
        line.replace_range(start..=stop, "");
        line = line.replace("[,", "[").replace(",]", "]");
        line = line.replace(",,", ",");
    }
    println!("{:?}", list);
    // panic!();
    list
}
