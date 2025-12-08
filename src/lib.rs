use std::sync::{
    Arc,
    atomic::{AtomicU64, Ordering},
};

use std::thread;

pub mod file_parser;
use crate::file_parser::FileParser;

pub fn solve_pt1(input_file: &str, shortest_circuits: usize) -> u64 {
    let input_arr: Vec<[i64; 3]> = input_file
        .lines()
        .map(|line| {
            let nums = line
                .split(',')
                .map(|num| num.trim().parse::<i64>().expect("Failed to parse i32"))
                .collect::<Vec<i64>>();

            nums.try_into()
                .expect("Number of numbers on this line not equal to 3")
        })
        .collect();

    // Maps junction coordinates to already started circuit (or None)
    let mut mapping_vec: Vec<Option<usize>> = vec![None; input_arr.len()];

    // Keeps track of circuit sizes
    let mut circuit_size_vec: Vec<u64> = Vec::with_capacity(input_arr.len());

    // Distances between junction pairs
    let mut distance_vec: Vec<(i64, (usize, usize))> = Vec::with_capacity(input_arr.len().pow(2));

    // Calculate distances
    for (j1_idx, j1) in input_arr.iter().enumerate() {
        for (j2_idx, j2) in input_arr.iter().enumerate().skip(j1_idx + 1) {
            distance_vec.push((
                j1.iter()
                    .zip(j2.iter())
                    .fold(0i64, |acc, (x, y)| acc + (x - y).pow(2)),
                (j1_idx, j2_idx),
            ));
        }
    }

    // Sort based on distance
    distance_vec.sort_unstable_by_key(|x| x.0);

    // Make circuits
    for connection in &distance_vec[..shortest_circuits] {
        let existing_connection_for_j1 = mapping_vec[connection.1.0];
        let existing_connection_for_j2 = mapping_vec[connection.1.1];

        if let (Some(circuit1), Some(circuit2)) =
            (existing_connection_for_j1, existing_connection_for_j2)
        {
            if circuit1 == circuit2 {
                // println!(
                //     "Redundant Connection: nodes {} and {} in circuit {}",
                //     connection.1.0, connection.1.1, circuit1
                // );
                continue;
            }
            let circuit: usize;
            let absorbed_circuit: usize;
            if circuit_size_vec[circuit1] > circuit_size_vec[circuit2] {
                circuit = circuit1;
                absorbed_circuit = circuit2;
            } else {
                circuit = circuit2;
                absorbed_circuit = circuit1;
            }

            circuit_size_vec[circuit] += circuit_size_vec[absorbed_circuit];
            circuit_size_vec[absorbed_circuit] = 0;

            for junction in &mut mapping_vec {
                if *junction == Some(absorbed_circuit) {
                    // dbg!(&junction);
                    *junction = Some(circuit);
                }
            }
            // println!(
            //     "Merging circuit {} into circuit {} after connecting nodes {} and {}",
            //     absorbed_circuit, circuit, connection.1.0, connection.1.1
            // );
            // println!(
            //     "    Circuit {} size: {}",
            //     circuit, circuit_size_vec[circuit]
            // );
        } else {
            // println!(
            //     "!!!! Evaluating {} on circuit {} and {} on circuit {}",
            //     connection.1.0,
            //     existing_connection_for_j1.unwrap_or_else(|| 1000),
            //     connection.1.1,
            //     existing_connection_for_j2.unwrap_or_else(|| 1000)
            // );
            let mut circuit = existing_connection_for_j1.or(existing_connection_for_j2);

            if circuit.is_none() {
                circuit_size_vec.push(2);
                circuit = Some(circuit_size_vec.len() - 1);
                // println!(
                //     "New circuit {}: nodes {} and {}",
                //     circuit.unwrap(),
                //     connection.1.0,
                //     connection.1.1
                // );
            } else {
                circuit_size_vec[circuit.unwrap()] += 1;
                // println!(
                //     "Connecting to existing circuit {}: {}, {}",
                //     circuit.unwrap(),
                //     connection.1.0,
                //     connection.1.1
                // );
            }
            // println!(
            //     "    Circuit {} size: {}",
            //     circuit.unwrap(),
            //     circuit_size_vec[circuit.unwrap()]
            // );

            mapping_vec[connection.1.0] = circuit;
            mapping_vec[connection.1.1] = circuit;
        }
    }

    // Sort by largest circuits
    circuit_size_vec.sort();

    let mut result = 1u64;
    for circuit_size in &circuit_size_vec[(circuit_size_vec.len() - 3)..] {
        result *= circuit_size;
    }

    result
}

pub fn solve_pt2(input_file: &str) -> u64 {
    0
}

pub fn solve(input_file: &str) -> (u64, u64) {
    (0, 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_PT1: u64 = 40;
    const EXAMPLE_PT2: u64 = 0;
    const ACTUAL_PT1: u64 = 57564;
    const ACTUAL_PT2: u64 = 0;

    // #[test]
    // fn example() {
    //     let my_file = FileParser::new("data/example.txt");
    //     let (part_1, part_2) = solve(my_file.get_str());
    //     assert_eq!(part_1, EXAMPLE_PT1);
    //     assert_eq!(part_2, EXAMPLE_PT2);
    // }

    #[test]
    fn example_pts() {
        let my_file = FileParser::new("data/example.txt");
        assert_eq!(solve_pt1(my_file.get_str(), 10), EXAMPLE_PT1);
        // assert_eq!(solve_pt2(my_file.get_str()), EXAMPLE_PT2);
    }
    //
    // #[test]
    // fn actual() {
    //     let my_file = FileParser::new("data/input.txt");
    //     let (part_1, part_2) = solve(my_file.get_str());
    //     assert_eq!(part_1, ACTUAL_PT1);
    //     assert_eq!(part_2, ACTUAL_PT2);
    // }
    //
    #[test]
    fn actual_pts() {
        let my_file = FileParser::new("data/input.txt");
        assert_eq!(solve_pt1(my_file.get_str(), 1000), ACTUAL_PT1);
        // assert_eq!(solve_pt2(my_file.get_str()), ACTUAL_PT2);
    }
}
