#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct InventoryItem {
    count: u64,
    cost: f64,
    description: String,
}

impl InventoryItem {
    pub fn new(count: u64, cost: f64, description: String) -> InventoryItem {
        InventoryItem {
            count,
            cost,
            description,
        }
    }
}

impl Eq for InventoryItem {
    // f64 isn't technically fully orderable, but we'll live with it.
}
impl Ord for InventoryItem {
    // implement Ord so they can go in a BTreeSet
    fn cmp(&self, other: &InventoryItem) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}
impl std::hash::Hash for InventoryItem {
    // implement Hash so they can go in a HashSet
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.count.hash(state);
        self.description.hash(state);
    }
}

pub fn total_value<'a>(items: impl IntoIterator<Item = &'a InventoryItem>) -> f64 {
    // TODO: the sum of the count times the cost of each item.
    items.into_iter().fold(0.0, |acc, x| acc + x.count as f64 * x.cost)
}

pub fn out_of_stock<'a>(items: impl IntoIterator<Item = &'a InventoryItem>) -> Vec<InventoryItem> {
    // TODO: create and return a vector of the items that have count 0.
    items.into_iter().filter(|x| x.count == 0).cloned().collect()
}

pub fn explode<'a>(items: impl IntoIterator<Item = &'a InventoryItem>) -> Vec<InventoryItem> {
    // TODO: a vector of the inventory items representing the same "inventory" but with all counts 1. That is, an item with .count==3 should produce three identical items with the count changed to 1. 
    // Hint: .flat_map()
    items
        .into_iter()
        .flat_map(|item| std::iter::repeat(item.clone()).take(item.count as usize))
        .map(|mut item| {
            item.count = 1;
            item
        })
        .collect()
}
