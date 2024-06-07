// This code is adapted from https://github.com/sebschmi/node-to-arc-centric-dbg


use genome_graph::bigraph::interface::static_bigraph::StaticEdgeCentricBigraph;
use genome_graph::bigraph::traitgraph::index::GraphIndex;
use genome_graph::bigraph::traitgraph::interface::{
    ImmutableGraphContainer, NavigableGraph, Neighbor,
};
use genome_graph::bigraph::traitgraph::traitsequence::interface::Sequence;
use genome_graph::compact_genome::implementation::DefaultSequenceStore;
use genome_graph::compact_genome::interface::alphabet::dna_alphabet::DnaAlphabet;
use genome_graph::compact_genome::interface::sequence::GenomeSequence;
use genome_graph::compact_genome::interface::sequence_store::SequenceStore;
use genome_graph::io::bcalm2::read_bigraph_from_bcalm2_as_edge_centric;
use genome_graph::types::PetBCalm2EdgeGraph;
use log::info;
use std::io::Write;
use std::io::BufRead;



pub fn node_to_arc_centric_dbg(k: usize, input: &mut impl BufRead, output: &mut impl Write) {
    info!("Reading graph");
    let mut sequence_store = DefaultSequenceStore::<DnaAlphabet>::new();
    let graph: PetBCalm2EdgeGraph<_> =
        read_bigraph_from_bcalm2_as_edge_centric(input, &mut sequence_store, k).unwrap();

    info!("Writing graph...");
    output_arc_centric_dbg(&graph, &sequence_store, k, output);
}

fn output_arc_centric_dbg(
    graph: &PetBCalm2EdgeGraph<
        <DefaultSequenceStore<DnaAlphabet> as SequenceStore<DnaAlphabet>>::Handle,
    >,
    sequence_store: &DefaultSequenceStore<DnaAlphabet>,
    k: usize,
    output: &mut impl Write,
) {
    writeln!(output, "{}", graph.node_count()).unwrap();
    for n1 in graph.node_indices() {
        let mut neighbors: Vec<_> = graph.out_neighbors(n1).collect();
        neighbors.sort_unstable_by_key(|neighbor| neighbor.node_id);

        let mut n2_iterator = neighbors.iter().peekable();
        while let Some(Neighbor {
            node_id: n2,
            edge_id,
        }) = n2_iterator.next().cloned()
        {
            let edge_data = graph.edge_data(edge_id);

            // if there is a pair of reverse complemental edges with a self-complemental label,
            // then we merge them, as they represent the same sequence.
            let weight_multiplier = if let Some(Neighbor {
                node_id: next_n2,
                edge_id: next_edge_id,
            }) = n2_iterator.peek()
            {
                let next_edge_data = graph.edge_data(*next_edge_id);
                if n2 == *next_n2
                    && graph.mirror_edge_edge_centric(edge_id).unwrap() == *next_edge_id
                {
                    if edge_data.forwards == next_edge_data.forwards {
                        if sequence_store.get(&edge_data.sequence_handle)
                            == sequence_store.get(&next_edge_data.sequence_handle)
                        {
                            n2_iterator.next().unwrap();
                            2
                        } else {
                            1
                        }
                    } else if sequence_store
                        .get(&edge_data.sequence_handle)
                        .iter()
                        .copied()
                        .zip(
                            sequence_store
                                .get(&next_edge_data.sequence_handle)
                                .reverse_complement_iter(),
                        )
                        .all(|(c1, c2)| c1 == c2)
                    {
                        n2_iterator.next().unwrap();
                        2
                    } else {
                        1
                    }
                } else {
                    1
                }
            } else {
                1
            };

            let kmer_count = edge_data.length - (k - 1);

            let n1 = n1.as_usize();
            let n2 = n2.as_usize();
            let weight = edge_data.total_abundance / kmer_count * weight_multiplier;
            write!(output, "{n1} {n2} {weight} ").unwrap();

            let sequence = sequence_store.get(&edge_data.sequence_handle);
            if edge_data.forwards {
                for character in sequence.iter() {
                    write!(output, "{}", character).unwrap();
                }
            } else {
                for character in sequence.reverse_complement_iter() {
                    write!(output, "{}", character).unwrap();
                }
            }
            writeln!(output).unwrap();
        }
    }
}