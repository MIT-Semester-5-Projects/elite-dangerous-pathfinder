use std::cmp::Ordering;

#[derive(Clone)]
pub struct DijkState {
    pub cost: f64,
    pub system_id: u64,
}

// Implementing PartialEq and Ord manually so we don't need Eq for f64
impl PartialEq for DijkState {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost && self.system_id == other.system_id
    }
}

impl Eq for DijkState {}

impl Ord for DijkState {
    fn cmp(&self, other: &Self) -> Ordering {
        // Reverse order so BinaryHeap behaves like a min-heap
        other.cost.partial_cmp(&self.cost).unwrap()
    }
}

impl PartialOrd for DijkState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Clone)]
pub struct StarState {
    pub cost: f64, // g(n) - actual cost
    pub system_id: u64,
    pub heuristic: f64, // h(n) - heuristic estimate (for priority queue ordering)
    pub ship_mass: f64, // Ship mass affects fuel cost
}

// Implementing PartialEq and Ord manually so we don't need Eq for f64
impl PartialEq for StarState {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost && self.system_id == other.system_id
    }
}

impl Eq for StarState {}

impl Ord for StarState {
    fn cmp(&self, other: &Self) -> Ordering {
        // Use total cost (g(n) + h(n)) for comparison
        other.heuristic.partial_cmp(&self.heuristic).unwrap()
    }
}

impl PartialOrd for StarState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
