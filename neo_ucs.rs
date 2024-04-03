use std::collections::{BinaryHeap, HashMap};
use std::cmp::Ordering;

// Define a struct for the node in the graph
#[derive(Clone, Eq, PartialEq)]
struct Node {
    name: char,
    cost: u32,
}

// Implement Ord for the Node struct
impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost) // Reversed ordering for BinaryHeap
    }
}

// Implement PartialOrd for the Node struct
impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// Define a struct for the graph
struct Graph {
    nodes: HashMap<char, Vec<(char, u32)>>,
}

impl Graph {
    fn new() -> Self {
        Graph {
            nodes: HashMap::new(),
        }
    }

    // Function to add an edge to the graph
    fn add_edge(&mut self, from: char, to: char, cost: u32) {
        self.nodes.entry(from).or_insert_with(Vec::new).push((to, cost));
    }

    // Function to perform Uniform Cost Search
    fn uniform_cost_search(&self, start: char, goal: char) -> Option<u32> {
        let mut frontier = BinaryHeap::new();
        let mut explored = HashMap::new();

        frontier.push(Node { name: start, cost: 0 });

        while let Some(Node { name, cost }) = frontier.pop() {
            if name == goal {
                return Some(cost);
            }

            if let Some(neighbors) = self.nodes.get(&name) {
                for &(next_node, next_cost) in neighbors {
                    let new_cost = cost + next_cost;
                    if !explored.contains_key(&next_node) || new_cost < explored[&next_node] {
                        explored.insert(next_node, new_cost);
                        frontier.push(Node {
                            name: next_node,
                            cost: new_cost,
                        });
                    }
                }
            }
        }

        None
    }
}

fn main() {
    let mut graph = Graph::new();
    graph.add_edge('A', 'B', 1);
    graph.add_edge('A', 'C', 5);
    graph.add_edge('B', 'D', 3);
    graph.add_edge('C', 'D', 1);
    graph.add_edge('C', 'E', 2);
    graph.add_edge('D', 'E', 2);

    if let Some(cost) = graph.uniform_cost_search('A', 'E') {
        println!("Cost from A to E: {}", cost);
    } else {
        println!("No path found from A to E");
    }
}
