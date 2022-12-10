use std::{
    cell::RefCell,
    rc::{Rc, Weak}, fs, borrow::BorrowMut, thread::current,
};

#[derive(Debug)]
struct Node {
    name: String,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
    values: Vec<i32>,
}

fn main() {

    let input = fs::read_to_string("input.txt").unwrap();

    let root = Rc::new(Node {
        name: "/".to_string(),
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
        values: Vec::new(),
    });

    println!("{:?}", root);
    let mut current_node = Rc::clone(&root);

    for line in input.lines() {
        if line.contains("cd") {
            let name = line.split(" ").last().unwrap();
            let new_node = Rc::new(Node {
                name: name.to_string(),
                parent: RefCell::new(Weak::new()),
                children: RefCell::new(vec![]),
                values: Vec::new(),
            });
            *new_node.parent.borrow_mut() = Rc::downgrade(&current_node);
            current_node.children.borrow_mut().push(Rc::clone(&new_node));
            current_node = new_node;
        }
    
    }

}
