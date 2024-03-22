use std::collections::HashSet;
use std::collections::VecDeque;
use crate::edge::EdgeId;
use crate::graph::build_graph;
use crate::flow::initialize_weight_of_neighbors_from;
use crate::uniqueness::unique_sequences;
use crate::recursion::recursion;
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


    let mut counter = 0;
    for edge in &edges {
        if counter == total_edges / 2 || counter == total_edges / 4 || counter == total_edges / 10 {
            info!("Coumputed {} / {} edges", counter, total_edges);
            if let Some(ref mut meter) = meter {
                meter.report();
            }
        }
        let first_edge_id = edge.id;
        let current_edge = edge.clone();
        let first_weight = edge.weight;
        let excess_flow = edge.weight;
        let mut safe_path: VecDeque<EdgeId> = VecDeque::new();
        safe_path.push_back(edge.id);
        recursion(safe_path, first_edge_id, current_edge, first_weight, excess_flow, &weight_of_neighbors_of_each_node, &edgelist, &mut safe_edge_paths, &mut extra_weight_of_paths);
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