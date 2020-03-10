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
    // parent: Option<String>,
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

fn make_tree(input: &str) -> HashMap<String, Node> {
    let mut tree = HashMap::new();
    for line in input.lines() {
        let objects = line.split(")").collect::<Vec<&str>>();
        let obj1 = objects[0].to_string();
        let obj2 = objects[1].to_string();
        if obj1 == "5JZ" {
            let a = "this";
        }
        tree.entry(obj1)
            .or_insert(Node {
                // name: obj1.clone(),
                // parent: None,
                children: vec![],
            })
            .children
            .push(obj2.clone());

        tree.entry(obj2).or_insert(Node {
            // name: obj2.clone(),
            // parent: Some(obj1.clone()),
            children: vec![],
        });
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
