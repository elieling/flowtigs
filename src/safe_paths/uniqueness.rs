
use std::collections::HashSet;
use crate::safe_paths::graph::Edgelist;
use crate::safe_paths::edge::Edge;
use crate::safe_paths::edge::EdgeId;
use crate::safe_paths::edge::Weight;
use std::collections::VecDeque;
use std::cmp::max;
use log::error;




pub fn create_parent_structure(edgelist: &Edgelist) -> Vec<Vec<Edge>> {
    let mut parents = Vec::new();
    let empty_vector = Vec::new();
    for _ in 0..edgelist.len() {
        parents.push(empty_vector.clone());
    }
    for node in edgelist {
        for edge in node.values() {
            parents[edge.end_node].push(edge.clone());
        }
    }
    parents
}



// Check if the safe path is maximal
pub fn is_maximal(path: &VecDeque<EdgeId>, edgelist: &Edgelist, weight_left: Weight, parents: &[Vec<Edge>], 
    weights_of_neighbors: &[Weight], edges: &Vec<Edge>) -> bool {

    let last_edge_id = path.back().unwrap();
    let first_edge_id = path.get(0).unwrap();

    // Right side
    let last_node = edges[*last_edge_id].end_node;
    let mut maximum_weight_of_a_neighbor = 0;
    let mut total_weight_of_neighbors = 0;
    for child in edgelist[last_node].values() {
        total_weight_of_neighbors += child.weight;
        if child.id == *first_edge_id {continue;}
        maximum_weight_of_a_neighbor = max(maximum_weight_of_a_neighbor, child.weight);
    }
    
    // Check if the flow is sufficient to get a longer path. If yes, return false.
    if weight_left > total_weight_of_neighbors - maximum_weight_of_a_neighbor {
        return false;
    } 

    // Left side
    let first_node = edges[*first_edge_id].start_node;
    let mut maximum_weight_of_parent_edge = 0;
    for parent in &parents[first_node] {
        if parent.id != *last_edge_id {
            maximum_weight_of_parent_edge = max(maximum_weight_of_parent_edge, parent.weight);
        }
    }

    // Check if the flow is sufficient to get a longer path. If yes, return false.
    if weight_left + maximum_weight_of_parent_edge - weights_of_neighbors[first_node] > 0 {
        return false;
    }

    true
}


fn reverse_byte(byte: u8) -> u8 {
    if byte == 65 {
        return 84;
    }
    if byte == 67 {
        return 71;
    }
    if byte == 71 {
        return 67;
    }
    65
}


fn get_smaller_between_iself_and_reverse_complement(sequence: String) -> String {
    let mut reverse_complement = String::from("");
    let mut counter = sequence.len();
    let byte_sequence = sequence.as_bytes();
    for _ in 0..sequence.len() {
        counter -= 1;
        reverse_complement.push(reverse_byte(byte_sequence[counter]) as char);
    }

    if sequence < reverse_complement {
        return sequence;
    }
    reverse_complement
}



pub fn unique_sequences(safe_edge_paths: Vec<VecDeque<EdgeId>>, k: usize, weights: &[Weight], 
    edgelist: &Edgelist, weights_of_neighbors: Vec<Weight>, string_sequences: Vec<String>, 
    edges: &Vec<Edge>) -> HashSet<String> {

    let parents = create_parent_structure(edgelist);
    let mut safe_paths = HashSet::new();
    let  mut counter = 0;
    for mut sequence in safe_edge_paths {
        if is_maximal(&sequence, edgelist, weights[counter], &parents, &weights_of_neighbors, edges) {
            // let first_edge_id = sequence.pop_front();
            let first_edge_id = match sequence.pop_front() {
                Some(id) => id,
                None => {
                    error!("Emprty walk");
                    0
                }
            };
            // let first_edge = edges[first_edge_id];
            // let mut string_path = (string_sequences[first_edge.unwrap().id]).to_string(); 
            let mut string_path = (string_sequences.get(first_edge_id)).expect("REASON").to_string(); 
            for edge_id in sequence {
                string_path += &string_sequences[edge_id][k-1..]; 
            }
            safe_paths.insert(get_smaller_between_iself_and_reverse_complement((string_path).to_string()));
        }
        counter += 1;
    }
    
    safe_paths
}
