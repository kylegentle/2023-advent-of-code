use num_integer::Integer;
use std::cell::RefCell;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::rc::{Rc, Weak};
use std::result::Result;

#[derive(Clone, Debug)]
struct Node {
    id: String,
    left: Option<Weak<RefCell<Node>>>,
    right: Option<Weak<RefCell<Node>>>,
}

impl Node {
    fn new(id: String) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node {
            id,
            left: None,
            right: None,
        }))
    }

    fn new_with_children(
        id: String,
        left: Weak<RefCell<Node>>,
        right: Weak<RefCell<Node>>,
    ) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node {
            id,
            left: Some(left),
            right: Some(right),
        }))
    }

    fn parse_into_map(
        line: &String,
        nodes: &mut HashMap<String, Rc<RefCell<Node>>>,
    ) -> Result<(), Box<dyn Error>> {
        let parts: Vec<&str> = line
            .strip_suffix(")")
            .map(|l| l.split(" = ("))
            .ok_or("line should match node format")?
            .collect::<Vec<_>>();

        let id = parts[0].to_string();
        let children: Vec<&str> = parts[1].split(", ").collect();

        let left_id = children[0].to_string();
        let right_id = children[1].to_string();

        let left = Rc::downgrade(nodes.entry(left_id.clone()).or_insert(Node::new(left_id)));
        let right = Rc::downgrade(nodes.entry(right_id.clone()).or_insert(Node::new(right_id)));

        nodes
            .entry(id.clone())
            .and_modify(|n| {
                n.borrow_mut().left = Some(left.clone());
                n.borrow_mut().right = Some(right.clone());
            })
            .or_insert(Node::new_with_children(id, left, right));

        Ok(())
    }
}

fn count_steps(
    node: &Rc<RefCell<Node>>,
    dest_id: &str,
    steps: &Vec<char>,
) -> Result<usize, Box<dyn Error>> {
    let mut cur = node.clone();
    let mut step: usize = 0;

    while cur.borrow().id != dest_id {
        cur = do_step(&cur, steps[step % steps.len()])?;
        step += 1
    }

    Ok(step)
}

fn count_ghost_steps(
    nodes: Vec<Rc<RefCell<Node>>>,
    steps: &Vec<char>,
) -> Result<usize, Box<dyn Error>> {
    nodes
        .iter()
        .map(|n| steps_to_ghost_end(n, steps))
        .try_fold(1, |acc, steps| Ok(acc.lcm(&steps?)))
}

fn steps_to_ghost_end(
    node: &Rc<RefCell<Node>>,
    steps: &Vec<char>,
) -> Result<usize, Box<dyn Error>> {
    let mut cur = node.clone();
    let mut step: usize = 0;

    while !cur.borrow().id.ends_with('Z') {
        cur = do_step(&cur, steps[step % steps.len()])?;
        step += 1
    }

    Ok(step)
}

fn do_step(node: &Rc<RefCell<Node>>, direction: char) -> Result<Rc<RefCell<Node>>, Box<dyn Error>> {
    let node = node.borrow();
    let next = match direction {
        'L' => node
            .left
            .as_ref()
            .ok_or(format!("missing left child for node {}", node.id))?
            .upgrade()
            .ok_or(format!("left child node deallocated for {}", node.id)),
        'R' => node
            .right
            .as_ref()
            .ok_or(format!("missing right child for node {}", node.id))?
            .upgrade()
            .ok_or(format!("right child node deallocated for {}", node.id)),
        x => Err(format!("Invalid step instruction: {}", x)),
    }?;

    Ok(next)
}

pub fn wasteland(f: File) -> Result<(), Box<dyn Error>> {
    let lines: Vec<String> = BufReader::new(f).lines().collect::<Result<_, _>>()?;
    let mut map = HashMap::new();

    let steps: Vec<char> = lines[0].chars().collect();
    lines[2..]
        .iter()
        .try_for_each(|l| Node::parse_into_map(l, &mut map))?;

    let start = map.get("AAA");
    let p1 = match start {
        Some(node) => count_steps(node, "ZZZ", &steps),
        None => Err("missing start node".into()),
    }?;

    println!("Part 1: {}", p1);

    let p2_nodes: Vec<Rc<RefCell<Node>>> = map
        .values()
        .filter(|n| n.borrow().id.ends_with('A'))
        .map(|n| n.clone())
        .collect();
    let p2 = count_ghost_steps(p2_nodes, &steps)?;

    println!("Part 2: {}", p2);

    Ok(())
}
