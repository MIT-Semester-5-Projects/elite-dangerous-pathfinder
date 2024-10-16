use std::cmp::Ordering;

#[derive(Clone)]
pub struct State {
    pub cost: f64,
    pub system_id: u64,
}

// Implementing PartialEq and Ord manually so we don't need Eq for f64
impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost && self.system_id == other.system_id
    }
}

impl Eq for State {}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Reverse order so BinaryHeap behaves like a min-heap
        other.cost.partial_cmp(&self.cost).unwrap()
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
