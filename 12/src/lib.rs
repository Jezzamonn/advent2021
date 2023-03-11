use std::fs;

mod graph;

use graph::Graph;

pub fn solve_pt1(filename: &str) -> u64 {
    let contents = fs::read_to_string(filename).expect("Something totes went wrong with that file");

    let graph = Graph::from_str(&contents);

    // // Debug: Write to dot file to make sure the graph is correct.
    // let dot_path = Path::new(filename).with_extension("dot");
    // fs::write(dot_path, graph.to_dot()).expect("Failed to write dot file");

    graph.number_of_paths().expect("Cannae get the paths for yeh")
}

pub fn solve_pt2(filename: &str) -> u64 {
    let contents = fs::read_to_string(filename).expect("Something totes went wrong with that file");

    let graph = Graph::from_str(&contents);

    graph.number_of_paths_with_revisiting().expect("Cannae get the paths for yeh")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pt1_demo1() {
        assert_eq!(solve_pt1("demo1.txt"), 10);
    }

    #[test]
    fn test_pt1_demo2() {
        assert_eq!(solve_pt1("demo2.txt"), 19);
    }

    #[test]
    fn test_pt1_demo3() {
        assert_eq!(solve_pt1("demo3.txt"), 226);
    }

    #[test]
    fn test_pt2_demo1() {
        assert_eq!(solve_pt2("demo1.txt"), 36);
    }

    #[test]
    fn test_pt2_demo2() {
        assert_eq!(solve_pt2("demo2.txt"), 103);
    }

    #[test]
    fn test_pt2_demo3() {
        assert_eq!(solve_pt2("demo3.txt"), 3509);
    }
}
