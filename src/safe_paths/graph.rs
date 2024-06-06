use std::fs;
use std::collections::HashMap;
use crate::safe_paths::edge::Edge;
use crate::safe_paths::edge::build_edge;
use crate::safe_paths::edge::NodeId;
use crate::safe_paths::edge::EdgeId;
use crate::safe_paths::edge::Weight;
use log::info;


pub type Edgelist = Vec<HashMap<EdgeId, Edge>>;


// Reading the file
fn read_file(path: &str) -> String {
    // println!("Using file {}", path);
    fs::read_to_string(path)
        .expect("Should have been able to read the file")
}


// Creating data structure representing the graph and calculating indegree and outdegree of each node
fn create_graph(values: Vec<&str>, n_nodes : NodeId) -> (Vec<HashMap<EdgeId, Edge>>, Vec<String>, Vec<Edge>) {
    
    // Setup empty data structure
    let mut edgelist: Vec<HashMap<EdgeId, Edge>> = Vec::new();
    let mut edges: Vec<Edge> = Vec::new();
    let empty : HashMap<EdgeId, Edge> = HashMap::new();
    let mut string_sequences: Vec<String> = Vec::new(); // Data structure to keep the string sequence related to an edge id
    for _ in 0..n_nodes {
        edgelist.push(empty.clone());
    }


    // Create the graph
    let rounds = (values).len() / 4;
    let mut id : EdgeId = 0;
    for i in 0..rounds {
        string_sequences.push(String::new());
        let node1: NodeId = values[i*4+1].parse().unwrap();
        let node2: NodeId = values[i*4+2].parse().unwrap();
        let nodeweight: Weight = values[i*4+3].parse().unwrap();
        let edge = build_edge(id, node1, node2, nodeweight); 
        string_sequences[id] = (&values[i*4+4]).to_string();
        edgelist[node1].insert(edge.id, edge.clone());
        edges.push(edge.clone());
        id += 1;

    }
    (edgelist, string_sequences, edges)
}









// Read the data and build the graph
pub fn build_graph(path: &str) -> (Vec<HashMap<EdgeId, Edge>>, NodeId, Vec<String>, Vec<Edge>) {

    // Reading the file
    let contents = read_file(path);

    info!("Contents: {}, Path: {}",contents, path);
    
    // Setup
    let values: Vec<&str> = contents.split_whitespace().collect();
    let n_nodes = &values[0];
    let n_nodes : NodeId = n_nodes.parse().unwrap();    

    // Creating data structure representing the graph 
    let (edgelist, string_sequences, edges) = create_graph(values, n_nodes);

 
   
    (edgelist, n_nodes, string_sequences, edges)
}


