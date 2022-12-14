use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fmt::{Debug, Display},
    sync::atomic::{AtomicUsize, Ordering}, fs::File,
};
use node::Node;
// use edge::Edge;

pub mod node;
pub mod edge;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Graph<N> {
    pub nodes: HashMap<u64, N>,
    pub edges: HashMap<u64, (HashMap<u64, N>, HashMap<u64, N>)>,
    pub root: Option<u64>
}

static COUNTER: AtomicUsize = AtomicUsize::new(0);
fn set_id() -> usize {
    COUNTER.fetch_add(1, Ordering::SeqCst)
}


impl<N> Graph<N> where N: Debug + Copy {
    pub fn new(nodes: HashMap<u64, N>, edges: HashMap<u64, (HashMap<u64, N>, HashMap<u64, N>)>) -> Self {
        Graph { nodes, edges, root: None }
    }

    // Check if the node exists
    pub fn check_node(&self, add_node: Node<N>) -> bool
    where N: Copy + Eq {
        let mut b = 0;
        if add_node.0.len() == 0 {
            b = 0
        } else {
            for (_k, v) in add_node.0 {
                let map = self.nodes.clone();
                let a = map.iter().find_map(|(_key, &val)| if val == v { Some(v) } else { None }).unwrap();
                let x = Some(a);

                if let Some(_value) = x { b = 1 }
                else { b = 0 }
            }            
        }
        if b == 1 { true } else { false }
    }

    pub fn add_node(&mut self, add_node: Node<N>) -> &Graph<N> 
    where N: Copy + Eq
    {
        for (k, v) in add_node.0 {
            if if_gr_contains(self, v) {
                ();
            } else {
                self.nodes.insert(k, v);
            }
        };
        self
    }

    pub fn create_node(list_of_nodes: &[N]) -> Node<N> {
        let mut hash_node: HashMap<u64, N> = HashMap::new();
        let mut index = 0;
        for _i in list_of_nodes.iter() {
            let id = set_id() as u64;
            hash_node.insert(id, list_of_nodes[index]);
            index += 1;
        }
        let new_node: Node<N> = Node(hash_node);
        new_node
    }

    pub fn set_root(&mut self, root: Option<u64>) -> Option<u64> {
        let ids = &self.nodes;
        let a = root.unwrap();
        if ids.contains_key(&a) {
            self.root = root;
            root
        } else {
            self.root = None;
            root
        }
    }

    pub fn get_node(&self, index: &u64) -> Node<N> {
        let mut hash_node: HashMap<u64, N> = HashMap::new();
        for node in self.nodes.iter() {
            if node.0 == index {
                hash_node.insert(*node.0, *node.1);
            }
        }
        Node(hash_node)
    }

    pub fn get_id(&self, node: N) -> Option<u64> where N: Copy + PartialEq {
        let mut result_id = Some(0u64);
        for id in self.nodes.iter() {
            let existing_id = id.0;
            let existing_node = id.1;
            
            if *existing_node == node {
                result_id = Some(*existing_id);
                break;
            } else {
                result_id = None
            }
        }
        result_id
    }
    
    pub fn serial_triv(graph: &Graph<N>, path: &str) where
    N: Serialize + Copy + Display + ToString + std::fmt::Debug
    {
        let path = format!("{}/serial_graph.yml", path);
        let _file = match File::create(&path) {
            Err(why) => panic!("couldn't create {}: {}", path, why),
            Ok(file) => file,
        };
        let file = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .open(path)
            .expect("Couldn't open file");
        
        let serialized = serde_yaml::to_string(graph)
            .unwrap()
            .into_bytes();

        let value_serialized = String::from_utf8(serialized).expect("Invalid utf8 message");

        serde_yaml::to_writer(file, &value_serialized).unwrap();
    }
}

pub fn if_gr_contains<N>(graph: &Graph<N>, node: N) -> bool where N: Copy + Eq{
    let mut indicator = true;
    for i in graph.nodes.iter() {
        let node_names = graph.nodes[i.0];
        if node_names == node {
            indicator= true;
            break;
        } else { indicator = false }
    }
    indicator
}

impl<N> std::fmt::Display for Graph<N> where N: Debug {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "\nGRAPH:\n nodes: {:?}\n edges: {:?}\n root: {:?}\n-----",
            self.nodes, self.edges, self.root
        )
    }
}
