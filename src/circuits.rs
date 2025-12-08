type Coordinate = [i64; 3];
type JunctionIndex = usize;
type CircuitIndex = usize;
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

    /// The index of the most recent circuit that had a junction connected
    most_recent_circuit_index: CircuitIndex,

    /// The last pair to be evaluated
    last_pair: JunctionPair,
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
            most_recent_circuit_index: 0,
            last_pair: (0, 0),
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
        self.circuit_size_vec
            .select_nth_unstable_by(3, |a, b| b.cmp(a));

        let mut result = 1u64;
        for circuit_size in &self.circuit_size_vec[..3] {
            result *= circuit_size;
        }

        result
    }

    pub fn pt2(&mut self) -> u64 {
        self.calculate_distances();
        self.sort_distances();

        let num_of_junctions = self.coordinates.len() as u64;
        println!("{}", num_of_junctions);
        self.make_shortest_connection();
        while self.circuit_size_vec[self.most_recent_circuit_index] < num_of_junctions {
            self.make_shortest_connection();
        }
        (self.coordinates[self.last_pair.0][0] * self.coordinates[self.last_pair.1][0]) as u64
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
    fn make_shortest_connection(&mut self) -> bool {
        let (_, (junc1, junc2)) = self
            .distance_vec
            .pop()
            .expect("Error: Vector was empty and you tried to pop!");

        self.last_pair = (junc1, junc2);

        let potential_j1_ciruit = self.mapping_vec[junc1];
        let potential_j2_ciruit = self.mapping_vec[junc2];

        match (potential_j1_ciruit, potential_j2_ciruit) {
            (Some(j1_ciruit), Some(j2_ciruit)) => {
                if j1_ciruit == j2_ciruit {
                    return false;
                }
                self.merge_circuits(j1_ciruit, j2_ciruit);
            }
            (None, None) => {
                // Create a new circuit and start the value at 2
                self.circuit_size_vec.push(2);

                self.most_recent_circuit_index = self.circuit_size_vec.len() - 1;

                self.mapping_vec[junc1] = Some(self.most_recent_circuit_index);
                self.mapping_vec[junc2] = Some(self.most_recent_circuit_index);
                return false;
            }
            (Some(circuit), None) | (None, Some(circuit)) => {
                self.circuit_size_vec[circuit] += 1;
                self.most_recent_circuit_index = circuit;

                // One of these is redundant, but this seems faster than checking
                self.mapping_vec[junc1] = Some(circuit);
                self.mapping_vec[junc2] = Some(circuit);
            }
        }
        true
    }

    /// Combine two circuits, both updating the counts and re-mapping the absorbed circuit (the
    /// smaller circuit)
    ///
    /// * `circuit1`: One of the circuits
    /// * `circuit2`: The other circuit
    fn merge_circuits(&mut self, circuit1: usize, circuit2: usize) {
        // The circuit merging into the other one
        let absorbed_circuit: usize;

        // Determine circuit is absorbing into which based on size
        if self.circuit_size_vec[circuit1] > self.circuit_size_vec[circuit2] {
            self.most_recent_circuit_index = circuit1;
            absorbed_circuit = circuit2;
        } else {
            self.most_recent_circuit_index = circuit2;
            absorbed_circuit = circuit1;
        }

        // Update the circuit size counts
        self.circuit_size_vec[self.most_recent_circuit_index] +=
            self.circuit_size_vec[absorbed_circuit];
        self.circuit_size_vec[absorbed_circuit] = 0;

        // Updating the mapping for all of the absorbed junctions
        for junction in &mut self.mapping_vec {
            if *junction == Some(absorbed_circuit) {
                *junction = Some(self.most_recent_circuit_index);
            }
        }
    }
}
