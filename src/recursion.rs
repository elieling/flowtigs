use std::collections::VecDeque;
use crate::Edge;
use crate::edge::EdgeId;
use crate::edge::Weight;
use crate::graph::Edgelist;



pub fn recursion(safe_path: VecDeque<EdgeId>, first_edge_id: EdgeId, current_edge: Edge, first_weight: Weight, excess_flow: Weight, weight_of_neighbors_of_each_node: &Vec<Weight>, 
    edgelist: &Edgelist, safe_edge_paths: &mut Vec<VecDeque<EdgeId>>, extra_weight_of_paths: &mut Vec<Weight>) {

        // If a cycle has no excess flow, stop running
        if first_edge_id == current_edge.id && first_weight == excess_flow && safe_path.len() > 1 {
            safe_edge_paths.push(safe_path.clone());
            extra_weight_of_paths.push(excess_flow);
            return;
        }

        // Verify whether the walk can be continued
        let mut can_continue = false;
        let weigth_of_all_next_edges = weight_of_neighbors_of_each_node[current_edge.end_node];
        for next_edge in edgelist[current_edge.end_node].values() {
            let weight_from_this_path = weigth_of_all_next_edges - next_edge.weight;
            if (excess_flow - weight_from_this_path) > 0 {
                can_continue = true;
            }
        }

        if !can_continue {
            safe_edge_paths.push(safe_path);
            extra_weight_of_paths.push(excess_flow);
        } else {
            // Iterate until excess flow is negative
            for next_edge in edgelist[current_edge.end_node].values() {
                let weight_from_this_path = weigth_of_all_next_edges - next_edge.weight;
                if (excess_flow - weight_from_this_path) > 0 {
                    let mut updated_safe_path = safe_path.clone();
                    updated_safe_path.push_back(next_edge.id);
                    recursion(updated_safe_path, first_edge_id, next_edge.clone(), first_weight, excess_flow - weight_from_this_path, 
                        weight_of_neighbors_of_each_node, edgelist, safe_edge_paths, extra_weight_of_paths);
                }
            }
        }
    }


    






