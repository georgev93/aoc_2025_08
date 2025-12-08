type Coordinate = [i64; 3];
type JunctionIndex = usize;
type JunctionPair = (JunctionIndex, JunctionIndex);

pub struct JunctionMess {
    /// Vector of coordinates (puzzle input)
    coordinates: Vec<Coordinate>,

    /// `mapping_vec[a] = b` where a is an index in the coordinates vector and b is a
    /// circuit_size_vector index
    mapping_vec: Vec<Option<usize>>,

    /// The index corresponds to a circuit number, the value is the size of the circuit
    circuit_size_vec: Vec<u64>,

    /// The distance (squared) between two Junction coordinates
    distance_vec: Vec<(i64, JunctionPair)>,
}

impl JunctionMess {
    pub fn new(input_str: &str) -> Self {
        let input_arr: Vec<Coordinate> = input_str
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

        Self {
            mapping_vec: vec![None; input_arr.len()],
            circuit_size_vec: Vec::with_capacity(input_arr.len()),
            distance_vec: Vec::with_capacity(input_arr.len().pow(2)),
            coordinates: input_arr,
        }
    }

    pub fn pt1(&mut self, num_of_connections: usize) -> u64 {
        self.calculate_distances();
        self.sort_distances();
        for connection in 0..num_of_connections {
            self.make_shortest_connection();
        }

        // Multiply the three largest circuits
        // Sort by largest circuits
        self.circuit_size_vec.sort();

        let mut result = 1u64;
        for circuit_size in &self.circuit_size_vec[(self.circuit_size_vec.len() - 3)..] {
            result *= circuit_size;
        }

        result
    }

    fn calculate_distances(&mut self) {
        for (j1_idx, j1) in self.coordinates.iter().enumerate() {
            for (j2_idx, j2) in self.coordinates.iter().enumerate().skip(j1_idx + 1) {
                self.distance_vec.push((
                    j1.iter()
                        .zip(j2.iter())
                        .fold(0i64, |acc, (x, y)| acc + (x - y).pow(2)),
                    (j1_idx, j2_idx),
                ));
            }
        }
    }

    /// Sort the distances vector in reverse order (longest distances closest to index 0)
    fn sort_distances(&mut self) {
        self.distance_vec.sort_unstable_by_key(|x| -x.0);
    }

    /// Makes the shortest possible connection, updates all vectors, and pops the distance vector
    /// element off of the vector
    fn make_shortest_connection(&mut self) {
        let (_, (junc1, junc2)) = self
            .distance_vec
            .pop()
            .expect("Error: Vector was empty and you tried to pop!");

        let potential_j1_ciruit = self.mapping_vec[junc1];
        let potential_j2_ciruit = self.mapping_vec[junc2];

        match (potential_j1_ciruit, potential_j2_ciruit) {
            (Some(j1_ciruit), Some(j2_ciruit)) => {
                if j1_ciruit == j2_ciruit {
                    return;
                }
                self.merge_circuits(j1_ciruit, j2_ciruit);
            }
            (None, None) => {
                // Create a new circuit and start the value at 2
                self.circuit_size_vec.push(2);

                let new_circuit = Some(self.circuit_size_vec.len() - 1);

                self.mapping_vec[junc1] = new_circuit;
                self.mapping_vec[junc2] = new_circuit;
            }
            (Some(circuit), None) | (None, Some(circuit)) => {
                self.circuit_size_vec[circuit] += 1;

                // One of these is redundant, but this seems faster than checking
                self.mapping_vec[junc1] = Some(circuit);
                self.mapping_vec[junc2] = Some(circuit);
            }
        }
    }

    /// Combine two circuits, both updating the counts and re-mapping the absorbed circuit (the
    /// smaller circuit)
    ///
    /// * `circuit1`: One of the circuits
    /// * `circuit2`: The other circuit
    fn merge_circuits(&mut self, circuit1: usize, circuit2: usize) {
        // The circuit we're absorbing into
        let circuit: usize;

        // The circuit merging into the other one
        let absorbed_circuit: usize;

        // Determine circuit is absorbing into which based on size
        if self.circuit_size_vec[circuit1] > self.circuit_size_vec[circuit2] {
            circuit = circuit1;
            absorbed_circuit = circuit2;
        } else {
            circuit = circuit2;
            absorbed_circuit = circuit1;
        }

        // Update the circuit size counts
        self.circuit_size_vec[circuit] += self.circuit_size_vec[absorbed_circuit];
        self.circuit_size_vec[absorbed_circuit] = 0;

        // Updating the mapping for all of the absorbed junctions
        for junction in &mut self.mapping_vec {
            if *junction == Some(absorbed_circuit) {
                *junction = Some(circuit);
            }
        }
    }
}
