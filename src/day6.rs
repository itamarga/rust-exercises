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
    orbit_transfer_calc(&tree, "YOU".to_string(), "SAN".to_string())
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

        let child = tree.entry(obj2.clone()).or_insert(Node {
            // name: obj2.clone(),
            parent: Some(obj1.clone()),
            depth: 0,
            children: vec![],
        });
        child.parent = Some(obj1.clone());
    }

    set_depth(&mut tree, &"COM".to_string(), 0);
    println!("{:?}", tree);
    tree
}

fn set_depth(tree: &mut HashMap<String, Node>, key: &String, depth: usize) {
    let node = tree.get_mut(key).unwrap();
    node.depth = depth;
    let children = node.children.clone();
    for child in children {
        set_depth(tree, &child, depth + 1);
    }
}

fn count_orbits(tree: &HashMap<String, Node>, node: &Node) -> usize {
    let mut count = node.children.len();
    for child in &node.children {
        let child_node = tree.get(child).unwrap();
        count += count_orbits(tree, &child_node);
    }
    count
}

fn orbit_transfer_calc(tree: &HashMap<String, Node>, origin: String, target: String) -> usize {
    let mut steps = 0;
    let mut node1 = origin;
    let mut node2 = target;
    while node1 != node2 {
        let curr1 = tree.get(&node1).unwrap();
        let curr2 = tree.get(&node2).unwrap();
        println!(
            "Curr1: {} Depth1: {}, Curr2: {} Depth2: {}",
            node1, curr1.depth, node2, curr2.depth
        );

        if curr1.depth > curr2.depth {
            node1 = curr1.parent.clone().unwrap();
        } else {
            node2 = curr2.parent.clone().unwrap();
        }
        steps += 1;
    }
    println!("Curr1: {}, Curr2: {}", node1, node2);
    steps - 2
}
