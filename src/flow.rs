use crate::edge::Weight;
use crate::graph::Edgelist;


pub fn initialize_weight_of_neighbors_from(edgelist: &Edgelist) -> Vec<Weight> {
    let mut weights_of_neighbors = Vec::new();
    for i in 0..edgelist.len() {
        weights_of_neighbors.push(0);
        for edge in edgelist[i].values() {
            weights_of_neighbors[i] += edge.weight;
        }
    }


    weights_of_neighbors
}
