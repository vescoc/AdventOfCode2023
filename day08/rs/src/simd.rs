use std::collections::HashMap;
use std::simd::prelude::*;

use num::integer::lcm;

use super::Network;

pub(crate) fn solve(path: &str, network: Network) -> u64 {
    const ZERO: usizex8 = usizex8::from_slice(&[0; 8]);
    const ONE: usizex8 = usizex8::from_slice(&[1; 8]);
    const TWO: usizex8 = usizex8::from_slice(&[2; 8]);

    #[allow(non_snake_case)]
    let FALSE: masksizex8 = masksizex8::from_array([false; 8]);

    let path = path
        .chars()
        .map(|c| if c == 'L' { ZERO } else { ONE })
        .cycle();

    let lookup = network
        .keys()
        .enumerate()
        .map(|(i, &node)| (node, i))
        .collect::<HashMap<_, _>>();

    let (mut current, graph, end, n, mut running) = {
        let mut start = [0; 8];
        let mut graph = Vec::with_capacity(network.len() * 2);
        let mut end = [usizex8::default(); 8];
        let mut running = [false; 8];

        let mut start_index = 0;
        let mut end_index = 0;

        for (node, (left_node, right_node)) in network {
            let node_idx = lookup[node];
            if node.ends_with('A') {
                start[start_index] = node_idx;
                running[start_index] = true;
                start_index += 1;
            } else if node.ends_with('Z') {
                end[end_index] = usizex8::splat(node_idx);
                end_index += 1;
            }

            let left_node_idx = lookup[left_node];
            graph.push(left_node_idx);

            let right_node_idx = lookup[right_node];
            graph.push(right_node_idx);
        }

        (
            usizex8::from_slice(&start),
            graph,
            end,
            start_index,
            masksizex8::from_array(running),
        )
    };

    let mut count = ZERO;
    for d in path {
        count += running.select(ONE, ZERO);

        current = usizex8::gather_or_default(&graph, TWO * current + d);

        for i in 0..n {
            running = current.simd_eq(end[i]).select_mask(FALSE, running);
        }

        if !running.any() {
            break;
        }
    }

    count[0..n]
        .iter()
        .map(|value| *value as u64)
        .reduce(lcm)
        .unwrap()
}
