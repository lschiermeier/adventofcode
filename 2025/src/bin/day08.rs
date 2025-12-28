use std::{
    cmp::max,
    cmp::min,
    collections::{HashSet},
};

use aoc_2025::*;
use itertools::Itertools;
#[macro_use]
extern crate log;
extern crate pretty_env_logger;
#[allow(unused)]

fn main() {
    pretty_env_logger::init();
    info!("Starting {}", file!());
    let args = std::env::args().collect::<Vec<String>>();
    let test_mode = args.len() > 1 && args[1] == "test";
    let src_name = file!();

    let input_path = gen_input_path(src_name, test_mode);
    let starting_cords =
        parse_table::<i64>(read_table(input_path, ',').expect("File could not be opened."));
    let junction_cords: Vec<Point3d> = starting_cords
        .iter()
        .map(|x| x.try_into().unwrap())
        .collect();
    let sorted_edges: Vec<Edge<Point3d>> = junction_cords
        .iter()
        .copied()
        .map(|y| {
            junction_cords
                .iter()
                .copied()
                // calc square distance
                .map(move |x| Edge::new(x, y))
        })
        .flatten()
        .filter(|x| x.square_dist != 0)
        .sorted_by_key(|x| x.square_dist)
        .dedup_by(|x, y| x.square_dist == y.square_dist)
        .collect();
    let connection_target_count = if test_mode { 10 } else { 1000 };
    let mut connections_made = 0;
    let mut networks: Vec<Network> = vec![];
    let mut edge_iter = sorted_edges.iter();
    for edge in sorted_edges {
        let maybe_before_network = networks
            .iter()
            .enumerate()
            .find(|x| x.1.points.contains(&edge.before));
        let maybe_after_network = networks
            .iter()
            .enumerate()
            .find(|x| x.1.points.contains(&edge.after));
        match (maybe_before_network, maybe_after_network) {
            (None, None) => {
                trace!("Adding network for: {edge:?}");
                networks.push(Network::new(&edge));
            }
            (None, Some((idx, _))) => {
                trace!("Connecting edge to after network: {edge:?}");
                let mut mut_net = networks.get_mut(idx).expect("Just found this...");
                mut_net.edges.insert(edge);
                mut_net.points.insert(edge.before);
            }
            (Some((idx, _)), None) => {
                trace!("Connecting edge to before network: {edge:?}");
                let mut mut_net = networks.get_mut(idx).expect("Just found this...");
                mut_net.edges.insert(edge);
                mut_net.points.insert(edge.after);
            }
            (Some((idx_before, before_network)), Some((idx_after, after_network))) => {
                if before_network == after_network {
                    trace!("Nothing to do for: {edge:?}");
                } else {
                    trace!("Joining networks for: {edge:?}");
                    let idx_fill = min(idx_before, idx_after);
                    let idx_drain = max(idx_before, idx_after);
                    trace!(
                        "Points to drain from {idx_drain}: {:?}",
                        networks.get(idx_drain).unwrap().points
                    );
                    trace!(
                        "Current points in {idx_fill} fill net: {:?}",
                        networks.get(idx_fill).unwrap().points
                    );
                    let drain_net = networks.swap_remove(idx_drain);
                    let mut mut_net = networks.get_mut(idx_fill).expect("Just found this...");
                    drain_net.edges.iter().for_each(|x| {
                        mut_net.edges.insert(*x);
                    });
                    let insert_results = drain_net.points.iter().for_each(|x| {
                        mut_net.points.insert(*x);
                    });
                    trace!("Points in filled net: {:?}", mut_net.points);
                }
            }
        }
        connections_made += 1;
        trace!("Connections made: {connections_made}");
        let network_sizes: Vec<usize> = networks.iter().map(|x| x.points.len()).collect();
        trace!("{network_sizes:?}");
        if connections_made == connection_target_count {
            networks.sort_by_key(|x| x.points.len());
            let network_sizes: Vec<usize> = networks.iter().map(|x| x.points.len()).collect();
            debug!("{network_sizes:?}");
        
            let top_nets = networks.iter().rev().take(3).collect_vec();
            let result_p1 = top_nets
                .iter()
                .map(|x| x.points.len())
                .fold(1, |x, y| x * y);
            info!("result p1: {result_p1:?}");
        }
        if *network_sizes.get(0).unwrap() == starting_cords.len() {
            debug!("fully connected in iteration {connections_made:?} using:\n{edge:?}");
            let result_p2 = edge.after.x * edge.before.x;
            info!("result p2: {result_p2}");
            break;
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Network {
    pub points: HashSet<Point3d>,
    pub edges: HashSet<Edge<Point3d>>,
}

impl Network {
    pub fn new(starting_edge: &Edge<Point3d>) -> Self {
        let mut points: HashSet<Point3d> = HashSet::new();
        points.insert(starting_edge.before);
        points.insert(starting_edge.after);
        let mut edges: HashSet<Edge<Point3d>> = HashSet::new();
        edges.insert(*starting_edge);
        Network {
            points: points,
            edges: edges,
        }
    }
}
