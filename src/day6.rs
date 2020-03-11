use std::collections::HashMap;

// const EXAMPLE: &str = "COM)B
// B)C
// C)D
// D)E
// E)F
// B)G
// G)H
// D)I
// E)J
// J)K
// K)L";

#[derive(Debug)]
struct Node {
    // name: String,
    parent: Option<String>,
    depth: usize,
    children: Vec<String>,
}

const INTPUT_PATH: &str = "src/day6_input.txt";

pub fn solve_day_6_pt_1() -> usize {
    let input = std::fs::read_to_string(INTPUT_PATH).unwrap();
    let tree = make_tree(&input);
    // println!("{:?}", tree);
    let mut count = 0;
    for node_name in tree.keys() {
        let node = &tree.get(node_name).unwrap();
        count += count_orbits(&tree, node);
    }
    count
}

pub fn solve_day_6_pt_2() -> usize {
    let input = std::fs::read_to_string(INTPUT_PATH).unwrap();
    let tree = make_tree(&input);
    // println!("{:?}", tree);
    orbit_transfer_calc(&tree, tree.get("YOU").unwrap(), tree.get("SAN").unwrap())
}
fn make_tree(input: &str) -> HashMap<String, Node> {
    let mut tree = HashMap::new();
    for line in input.lines() {
        let objects = line.split(")").collect::<Vec<&str>>();
        let obj1 = objects[0].to_string();
        let obj2 = objects[1].to_string();
        tree.entry(obj1.clone())
            .or_insert(Node {
                // name: obj1.clone(),
                parent: None,
                depth: 0,
                children: vec![],
            })
            .children
            .push(obj2.clone());

        let child_depth = tree.get(&obj1).unwrap().depth + 1;

        let child = tree.entry(obj2.clone()).or_insert(Node {
            // name: obj2.clone(),
            parent: Some(obj1.clone()),
            depth: child_depth,
            children: vec![],
        });
        child.depth = child_depth;
        child.parent = Some(obj1.clone());
    }
    tree
}

fn count_orbits(tree: &HashMap<String, Node>, node: &Node) -> usize {
    let mut count = node.children.len();
    for child in &node.children {
        let child_node = tree.get(child).unwrap();
        count += count_orbits(tree, &child_node);
    }
    count
}

fn orbit_transfer_calc(tree: &HashMap<String, Node>, origin: &Node, target: &Node) -> usize {
    let count = 0;
    // let parent1 = origin.parent.clone();
    // let parent2 = target.parent.clone();
    let mut curr1 = &origin;
    let mut curr2 = &target;
    while curr1.parent != curr2.parent {
        if curr1.depth < curr2.depth {
            let parent = &curr2.parent.clone().unwrap();
            curr2 = &tree.get(parent).unwrap();
        } else {
            let parent = &curr1.parent.clone().unwrap();
            curr1 = &tree.get(parent).unwrap();
        }
        println!(
            "curr1 parent: {}, curr2 parent: {}",
            curr1.parent.clone().unwrap(),
            curr2.parent.clone().unwrap()
        );
    }
    0
}
