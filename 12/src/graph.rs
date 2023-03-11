use std::collections::{HashMap, HashSet};

pub struct Node {
    pub name: String,
    // Names of the neighbors
    pub neighbors: Vec<String>,
    pub is_big: bool,
}

impl Node {
    pub fn new(name: &str) -> Self {
        Node {
            name: name.to_string(),
            neighbors: Vec::new(),
            is_big: name.chars().all(|c| c.is_uppercase()),
        }
    }
}

pub struct Graph {
    pub nodes: HashMap<String, Node>,
}

impl Graph {
    pub fn new() -> Self {
        Graph {
            nodes: HashMap::new(),
        }
    }

    pub fn from_str(input: &str) -> Self {
        let mut graph = Graph::new();

        for line in input.lines() {
            graph.add_edge(line);
        }

        graph
    }

    /// Parses a line from the graph input file and updates the graph to contain this edge, creating nodes if necessary.
    ///
    /// Edges are bi-directional, and each line looks like this:
    /// "a-b"
    pub fn add_edge(&mut self, line: &str) {
        let mut parts = line.split('-');
        let a = parts.next().unwrap();
        let b = parts.next().unwrap();

        let a_node = self.nodes.entry(a.to_string()).or_insert(Node::new(a));
        a_node.neighbors.push(b.to_string());

        let b_node = self.nodes.entry(b.to_string()).or_insert(Node::new(b));
        b_node.neighbors.push(a.to_string());
    }

    /// Outputs the graph as a dot file.
    pub fn to_dot(&self) -> String {
        let mut dot = String::new();

        dot.push_str("strict graph G {\n");

        for node in self.nodes.values() {
            for neighbor in &node.neighbors {
                dot.push_str(&format!("  \"{}\" -- \"{}\";\n", node.name, neighbor));
            }
        }

        dot.push_str("}\n");

        dot
    }

    pub fn number_of_paths(&self) -> Result<u64, String> {
        self.number_of_paths_to_end("start", &HashSet::new())
    }

    pub fn number_of_paths_to_end(
        &self,
        name: &str,
        visited: &HashSet<String>,
    ) -> Result<u64, String> {
        // Special case: We reached the end.
        // There's 1 path, just containing the end
        if name == "end" {
            return Ok(1);
        }

        let node = self
            .nodes
            .get(name)
            .ok_or_else(|| format!("Can't find-a da node-a {}", name))?;

        // If this is a small node, we can't revisit it.
        if !node.is_big && visited.contains(name) {
            // So this path is invalid. Return 0, meaning no path through here.
            return Ok(0);
        }

        // Mark this node as visited. Note that we need to copy the visited set to keep each search state separate.
        let mut new_visited = visited.clone();
        new_visited.insert(name.to_string());

        node.neighbors
            .iter()
            .map(|n| self.number_of_paths_to_end(n, &new_visited))
            .try_fold(0, |acc, r| r.map(move |v| acc + v))
    }
}
