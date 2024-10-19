/*
	graph
	This problem requires you to implement a basic graph functio
*/
// I AM NOT DONE

/*use std::collections::{HashMap, HashSet};
use std::fmt;
#[derive(Debug, Clone)]
pub struct NodeNotInGraph;
impl fmt::Display for NodeNotInGraph {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "accessing a node that is not in the graph")
    }
}
pub struct UndirectedGraph {
    adjacency_table: HashMap<String, Vec<(String, i32)>>,
}
impl Graph for UndirectedGraph {
    fn new() -> UndirectedGraph {
        UndirectedGraph {
            adjacency_table: HashMap::new(),
        }
    }
    fn adjacency_table_mutable(&mut self) -> &mut HashMap<String, Vec<(String, i32)>> {
        &mut self.adjacency_table
    }
    fn adjacency_table(&self) -> &HashMap<String, Vec<(String, i32)>> {
        &self.adjacency_table
    }
    fn add_edge(&mut self, edge: (&str, &str, i32)) {
        //TODO
    }
}
pub trait Graph {
    fn new() -> Self;
    fn adjacency_table_mutable(&mut self) -> &mut HashMap<String, Vec<(String, i32)>>;
    fn adjacency_table(&self) -> &HashMap<String, Vec<(String, i32)>>;
    fn add_node(&mut self, node: &str) -> bool {
        //TODO
		true
    }
    fn add_edge(&mut self, edge: (&str, &str, i32)) {
        //TODO
    }
    fn contains(&self, node: &str) -> bool {
        self.adjacency_table().get(node).is_some()
    }
    fn nodes(&self) -> HashSet<&String> {
        self.adjacency_table().keys().collect()
    }
    fn edges(&self) -> Vec<(&String, &String, i32)> {
        let mut edges = Vec::new();
        for (from_node, from_node_neighbours) in self.adjacency_table() {
            for (to_node, weight) in from_node_neighbours {
                edges.push((from_node, to_node, *weight));
            }
        }
        edges
    }
}
#[cfg(test)]
mod test_undirected_graph {
    use super::Graph;
    use super::UndirectedGraph;
    #[test]
    fn test_add_edge() {
        let mut graph = UndirectedGraph::new();
        graph.add_edge(("a", "b", 5));
        graph.add_edge(("b", "c", 10));
        graph.add_edge(("c", "a", 7));
        let expected_edges = [
            (&String::from("a"), &String::from("b"), 5),
            (&String::from("b"), &String::from("a"), 5),
            (&String::from("c"), &String::from("a"), 7),
            (&String::from("a"), &String::from("c"), 7),
            (&String::from("b"), &String::from("c"), 10),
            (&String::from("c"), &String::from("b"), 10),
        ];
        for edge in expected_edges.iter() {
            assert_eq!(graph.edges().contains(edge), true);
        }
    }
}*/
use std::collections::{HashMap, HashSet};  
use std::fmt;  
  
#[derive(Debug, Clone)]  
pub struct NodeNotInGraph;  
impl fmt::Display for NodeNotInGraph {  
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {  
        write!(f, "accessing a node that is not in the graph")  
    }  
}  
  
pub struct UndirectedGraph {  
    adjacency_table: HashMap<String, Vec<(String, i32)>>,  
}  
  
impl Graph for UndirectedGraph {  
    fn new() -> UndirectedGraph {  
        UndirectedGraph {  
            adjacency_table: HashMap::new(),  
        }  
    }  
  
    fn adjacency_table_mutable(&mut self) -> &mut HashMap<String, Vec<(String, i32)>> {  
        &mut self.adjacency_table  
    }  
  
    fn adjacency_table(&self) -> &HashMap<String, Vec<(String, i32)>> {  
        &self.adjacency_table  
    }  
  
    fn add_node(&mut self, node: &str) -> bool {  
        self.adjacency_table.entry(node.to_string()).or_insert(Vec::new()).is_empty()  
    }  
  
    fn add_edge(&mut self, edge: (&str, &str, i32)) {  
        let (from, to, weight) = edge;  
        self.adjacency_table  
            .entry(from.to_string())  
            .or_insert(Vec::new())  
            .push((to.to_string(), weight));  
        self.adjacency_table  
            .entry(to.to_string())  
            .or_insert(Vec::new())  
            .push((from.to_string(), weight));  
    }  
}  
  
pub trait Graph {  
    fn new() -> Self;  
    fn adjacency_table_mutable(&mut self) -> &mut HashMap<String, Vec<(String, i32)>>;  
    fn adjacency_table(&self) -> &HashMap<String, Vec<(String, i32)>>;  
    fn add_node(&mut self, node: &str) -> bool;  
    fn add_edge(&mut self, edge: (&str, &str, i32));  
    fn contains(&self, node: &str) -> bool {  
        self.adjacency_table().contains_key(node)  
    }  
    fn nodes(&self) -> HashSet<&String> {  
        self.adjacency_table().keys().collect()  
    }  
    fn edges(&self) -> Vec<(&String, &String, i32)> {  
        let mut edges = Vec::new();  
        for (from_node, from_node_neighbours) in self.adjacency_table() {  
            for (to_node, weight) in from_node_neighbours {  
                edges.push((from_node, to_node, *weight));  
            }  
        }  
        edges  
    }  
}  
  
#[cfg(test)]  
mod test_undirected_graph {  
    use super::Graph;  
    use super::UndirectedGraph;  
  
    #[test]  
    fn test_add_edge() {  
        let mut graph = UndirectedGraph::new();  
        graph.add_edge(("a", "b", 5));  
        graph.add_edge(("b", "c", 10));  
        graph.add_edge(("c", "a", 7));  
  
        let edges_set: HashSet<(&String, &String, i32)> = graph.edges().into_iter().collect();  
        let expected_edges_set: HashSet<(&'static str, &'static str, i32)> = [  
            ("a", "b", 5),  
            ("b", "a", 5),  
            ("c", "a", 7),  
            ("a", "c", 7),  
            ("b", "c", 10),  
            ("c", "b", 10),  
        ].iter().cloned().collect();  
  
        for expected_edge in expected_edges_set.iter() {  
            let (from, to, weight) = expected_edge;  
            let from_str = String::from(*from);  
            let to_str = String::from(*to);  
            assert!(edges_set.contains(&(&from_str, &to_str, *weight)));  
        }  
    }  
}