use std::collections::HashSet;
use std::hash::{Hash, Hasher};

pub fn day(input: std::string::String) {
    let input_vec = input.lines().map(|x| OrbitSet::new(x)).collect::<Vec<_>>();

    let map = build_map(&input_vec);
    let mut result_one = 0;

    for node in map {
        result_one += node.distance;
    }

    println!("Day 6 Result1: {:?}", result_one);
    println!("Day 6 Result2: {:?}", 0);
}

fn build_map(input: &Vec<OrbitSet>) -> HashSet<MapNode> {
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

    fill_distance(nodes)
}

fn fill_distance(mut nodes: HashSet<MapNode>) -> HashSet<MapNode> {
    let nodes_clone = nodes.clone();

    for node in nodes_clone {
        let first_target = MapNode::new(node.id.to_owned(), Default::default());
        let mut com_node = nodes.get(&first_target).unwrap();
        let mut to_base_com = 0;

        while !com_node.com.is_empty() {
            to_base_com += 1;
            let target = MapNode::new(com_node.com.to_owned(), Default::default());
            com_node = nodes.get(&target).unwrap();
        }

        let mut real_node = nodes.take(&node).unwrap();
        real_node.distance = to_base_com;
        nodes.insert(real_node);
    }

    nodes
}

#[derive(Debug, Clone, Eq)]
struct MapNode {
    id: String,
    com: String,
    distance: i32,
    next: HashSet<String>,
}

impl MapNode {
    fn new(id: String, com: String) -> MapNode {
        MapNode {
            id: id,
            com: com,
            distance: 0,
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
