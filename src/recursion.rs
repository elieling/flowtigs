use std::collections::VecDeque;
use crate::Edge;
use crate::edge::EdgeId;
use crate::edge::Weight;
use crate::graph::Edgelist;



pub fn recursion(safe_path: VecDeque<EdgeId>, first_edge: Edge, current_edge: Edge, first_weight: Weight, excess_flow: Weight, weight_of_neighbors_of_each_node: &Vec<Weight>, 
    edgelist: &Edgelist, safe_edge_paths: &mut Vec<VecDeque<EdgeId>>, extra_weight_of_paths: &mut Vec<Weight>) {

        // If a cycle has no excess flow, stop running
        if first_edge.id == current_edge.id && first_weight == excess_flow && safe_path.len() > 1 {
            safe_edge_paths.push(safe_path.clone());
            extra_weight_of_paths.push(excess_flow);
            return;
        }

        // Iterate until excess flow is negative
        for next_edge in edgelist[current_edge.end_node].values() {
            let weight_from_this_path = weight_of_neighbors_of_each_node[current_edge.end_node] - next_edge.weight;
            if (excess_flow - weight_from_this_path) <= 0 {
                safe_edge_paths.push(safe_path.clone());
                extra_weight_of_paths.push(excess_flow);
            } else {
                let mut updated_safe_path = safe_path.clone();
                updated_safe_path.push_back(next_edge.id);
                recursion(updated_safe_path, first_edge.clone(), next_edge.clone(), first_weight, excess_flow - weight_from_this_path, 
                    weight_of_neighbors_of_each_node, edgelist, safe_edge_paths, extra_weight_of_paths);
            }
        }
}


    






