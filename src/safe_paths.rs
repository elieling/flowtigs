use std::collections::HashSet;
use std::collections::VecDeque;
use crate::edge::EdgeId;
use crate::edge::Weight;
use crate::graph::build_graph;
use crate::flow::initialize_weight_of_neighbors_from;
use crate::uniqueness::unique_sequences;
// use crate::recursion::recursion;
use crate::memory_meter::MemoryMeter;
use log::info;


pub fn safe_paths(path: &str, k: usize, mut meter: Option<&mut MemoryMeter>) -> HashSet<String> {

    
    

    // Read the data and build the graph
    let (edgelist, n_nodes, string_sequences, edges) = build_graph(path);

    
    let total_edges = edges.len();

    info!("Data structure built successfully. Graph contains {} nodes and {} edges.", n_nodes, total_edges);
    if let Some(ref mut meter) = meter {
        meter.report();
    }
 


    //---------------------------------------------------------------------------
    // Flow decomposition is done and the cycles are gathered.
    // Next, two-pointer algorithm.
    //---------------------------------------------------------------------------

    // The paths as edges
    let mut safe_edge_paths = Vec::new();
    // The extra weight left corresponding to each path
    let mut extra_weight_of_paths = Vec::new();
    // The weight of neighbors of each node for edges leaving from that node
    let weight_of_neighbors_of_each_node: Vec<i64> = initialize_weight_of_neighbors_from(&edgelist);


    // counter only for logging purposes
    let mut counter = 0;
    let mut greatest_flow = 0;
    let mut longest_while = 0;
    let mut print_when_ends = false;
    // Find all safe paths that start on a specific edge. Repeat for all edges
    for edge in &edges {

        // Logging
        if counter == (total_edges / 2) + (total_edges / 3) ||  counter == total_edges / 2 || counter == total_edges / 4 || counter == total_edges / 10 {
            info!("----- Coumputed {} / {} edges", counter, total_edges);
            if let Some(ref mut meter) = meter {
                meter.report();
            }
        }

        // Initializing variables
        let excess_flow = edge.weight;
        if excess_flow > greatest_flow {
            greatest_flow = excess_flow;
            info!("New greatest flow: {}. Coumputed {} / {} edges", greatest_flow, counter, total_edges);
            if let Some(ref mut meter) = meter {
                meter.report();
            }
            print_when_ends = true;
        }
        let mut safe_path: VecDeque<EdgeId> = VecDeque::new();
        let mut waiting_list: VecDeque<(VecDeque<EdgeId>, Weight)> = VecDeque::new(); // Path, excess flow      

        // Initializing safe paths starting from edge, as well as thewaiting list for upcoming while loop
        safe_path.push_back(edge.id);
        let weigth_of_all_next_edges = weight_of_neighbors_of_each_node[edge.end_node];
        for next_edge in edgelist[edge.end_node].values() {
            let weight_from_this_path = weigth_of_all_next_edges - next_edge.weight;
            if (excess_flow - weight_from_this_path) > 0 {
                let mut updated_safe_path = safe_path.clone();
                updated_safe_path.push_back(next_edge.id);
                waiting_list.push_back((updated_safe_path, excess_flow - weight_from_this_path));
            }
        }


        let mut length_of_while = 0;
        // Iterating through each safe path starting with edge
        while !waiting_list.is_empty() {

            let (safe_path, excess_flow) = waiting_list.pop_back().expect("Waiting list is empty");
            let current_edge_id = safe_path.back().expect("Empty safe path");

            // If a cycle has no leakage, stop running
            if edge.id == *current_edge_id && edge.weight == excess_flow {
                safe_edge_paths.push(safe_path);
                extra_weight_of_paths.push(excess_flow);
                continue;
            }

            let current_edge = edges[*current_edge_id].clone();

            // Verify whether the walk can be continued
            let mut can_continue = false;
            let weigth_of_all_next_edges = weight_of_neighbors_of_each_node[current_edge.end_node];
            for next_edge in edgelist[current_edge.end_node].values() {
                let weight_from_this_path = weigth_of_all_next_edges - next_edge.weight;
                if (excess_flow - weight_from_this_path) > 0 {
                    can_continue = true;
                }
            }

            // If it can not continue, we store it in the list of safe paths
            if !can_continue {
                safe_edge_paths.push(safe_path);
                extra_weight_of_paths.push(excess_flow);
            } else {
                // Extend safe path while keeping positive excess flow
                for next_edge in edgelist[current_edge.end_node].values() {
                    let weight_from_this_path = weigth_of_all_next_edges - next_edge.weight;
                    if (excess_flow - weight_from_this_path) > 0 {
                        let mut updated_safe_path = safe_path.clone();
                        updated_safe_path.push_back(next_edge.id);
                        waiting_list.push_back((updated_safe_path, excess_flow - weight_from_this_path));
                    }
                }
            }
            length_of_while += 1;
            if length_of_while == 2 * longest_while || length_of_while == 5 * longest_while || length_of_while == 10 * longest_while || length_of_while == 25 * longest_while || length_of_while == 50 * longest_while || length_of_while == 100 * longest_while {
                info!("##### LONG WHILE: {}. Flow value of first edge: {}. Computed {} / {} edges", length_of_while, edge.weight, counter, total_edges);
                info!("##### First edge id: {}, latest edge id: {}, excess flow: {}", edge.id, current_edge_id, excess_flow);
                if let Some(ref mut meter) = meter {
                    meter.report();
                }
            }
        }
        if length_of_while > longest_while {
            longest_while = length_of_while;
            info!("New longest while: {}. Coumputed {} / {} edges", longest_while, counter, total_edges);
            if let Some(ref mut meter) = meter {
                meter.report();
            }
        }
        if print_when_ends {
            print_when_ends = false;
            info!("ENDED. Coumputed {} / {} edges", counter, total_edges);
            if let Some(ref mut meter) = meter {
                meter.report();
            }
        }
        counter += 1;
    }

    info!("Safe paths calculated successfully.");
    if let Some(ref mut meter) = meter {
        meter.report();
    }


    let safe_paths = unique_sequences(safe_edge_paths, k, &extra_weight_of_paths, &edgelist, weight_of_neighbors_of_each_node, string_sequences, &edges);


    info!("Safe paths made to strings successfully.");
    if let Some(ref mut meter) = meter {
        meter.report();
    }

   safe_paths
}