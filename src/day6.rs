use std::collections::HashSet;
use std::hash::{Hash, Hasher};

pub fn day(input: std::string::String) {
    let input_vec = input.lines().map(|x| OrbitSet::new(x)).collect::<Vec<_>>();

    build_map(&input_vec);

    // find tips and work backwards?

    println!("Day 6 Result1: {:?}", input_vec);
    println!("Day 6 Result2: {:?}", 0);
}

fn build_map(input: &Vec<OrbitSet>) {
    let mut nodes: HashSet<MapNode> = Default::default();

    for set in input {
        let com_node = MapNode::new(set.com.to_owned(), Default::default());
        let obj_node = MapNode::new(set.obj.to_owned(), set.com.to_owned());

        let has_start = nodes.contains(&com_node);
        let has_end = nodes.contains(&obj_node);

        let mut com_node = if has_start {
            nodes.take(&com_node).unwrap()
        } else {
            com_node
        };

        com_node.next.insert(set.obj.to_owned());
        nodes.insert(com_node);

        let mut obj_node = if has_end {
            nodes.take(&obj_node).unwrap()
        } else {
            obj_node
        };

        obj_node.com = set.com.to_owned();
        nodes.insert(obj_node);
    }

    for node in nodes {
        println!("{:?}", node);
    }
}

#[derive(Debug, Clone, Eq)]
struct MapNode {
    id: String,
    com: String,
    next: HashSet<String>,
}

impl MapNode {
    fn new(id: String, com: String) -> MapNode {
        MapNode {
            id: id,
            com: com,
            next: Default::default(),
        }
    }
}

impl Hash for MapNode {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl PartialEq for MapNode {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

#[derive(Debug, Clone)]
struct OrbitSet {
    com: String,
    obj: String,
}

impl OrbitSet {
    fn new(input: &str) -> OrbitSet {
        let input_vec = input.split(')').collect::<Vec<_>>();
        OrbitSet {
            com: input_vec[0].to_owned(),
            obj: input_vec[1].to_owned(),
        }
    }
}
