/*
This program models a network of fighters using an undirected graph. Each fighter is represented as a node, and each fight between two fighters is represented as an edge.

Key components:
- The `Fighter` struct represents a fighter with a name.
- The `add_edge` function adds an edge between two nodes in the graph, indicating a fight between two fighters.
- The `main` function initializes the graph, adds fighters as nodes, and establishes connections (fights) between them.

The program calculates the closeness centrality for each fighter, which is defined here as the inverse of the number of fights (degree) a fighter has. It then prints the centrality values and provides a brief analysis of each fighter's centrality in the network.

Closeness centrality in this context:
- A lower centrality value indicates a higher number of fights.
- A higher centrality value indicates fewer fights.

The program uses the `petgraph` crate for graph operations and the `fmt` module for formatting output.
The plotlib library is used to create an svg dot file of the graph for visualisation.
*/

use petgraph::dot::{Config, Dot};
use petgraph::graph::{NodeIndex, UnGraph};
use petgraph::Direction;
use plotlib::page::Page;
use plotlib::repr::Plot;
use plotlib::style::PointMarker;
use plotlib::view::ContinuousView;
use std::fmt;

#[derive(Debug)]
struct Fighter {
    name: String,
}

// Constructor for Fighter struct
impl Fighter {
    /// Creates a new Fighter with the given name.
    ///
    /// # Arguments
    ///
    /// * `name` - A string slice that holds the name of the fighter.
    ///
    /// # Example
    ///
    /// ```
    /// let fighter = Fighter::new("Khabib Nurmagomedov");
    /// ```
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }
}

// Impl Display
impl fmt::Display for Fighter {
    /// Formats the fighter's name for display.
    ///
    /// # Arguments
    ///
    /// * `f` - A mutable reference to a formatter.
    ///
    /// # Returns
    ///
    /// * `fmt::Result` - The result of the formatting operation.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

/// Adds an edge between two nodes in the graph.
///
/// # Arguments
///
/// * `graph` - A mutable reference to the graph.
/// * `nodes` - A slice of NodeIndex representing the nodes.
/// * `a` - The index of the first node.
/// * `b` - The index of the second node.
///
/// # Example
///
/// ```
/// add_edge(&mut graph, &fighter_nodes, 0, 1);
/// ```
fn add_edge(graph: &mut UnGraph<&Fighter, f32>, nodes: &[NodeIndex], a: usize, b: usize) {
    graph.add_edge(nodes[a], nodes[b], 1.0);
}

fn main() {
    // Create an undirected graph
    let mut graph = UnGraph::new_undirected();

    // Initialize fighters
    let fighters = [
        Fighter::new("Dustin Poirier"),
        Fighter::new("Khabib Nurmagomedov"),
        Fighter::new("Jose Aldo"),
        Fighter::new("Conor McGregor"),
        Fighter::new("Nate Diaz"),
    ];

    // Add fighters to the graph as nodes
    let fighter_nodes: Vec<NodeIndex> = fighters
        .iter()
        .map(|fighter| graph.add_node(fighter))
        .collect();

    // Add the edges/connections (i.e. who has fought with whom)
    add_edge(&mut graph, &fighter_nodes, 0, 1); // Dustin Poirier vs. Khabib Nurmagomedov
    add_edge(&mut graph, &fighter_nodes, 1, 3); // Khabib Nurmagomedov vs. Conor McGregor
    add_edge(&mut graph, &fighter_nodes, 3, 0); // Conor McGregor vs. Dustin Poirier
    add_edge(&mut graph, &fighter_nodes, 3, 2); // Conor McGregor vs. Jose Aldo
    add_edge(&mut graph, &fighter_nodes, 3, 4); // Conor McGregor vs. Nate Diaz
    add_edge(&mut graph, &fighter_nodes, 0, 4); // Dustin Poirier vs. Nate Diaz
    add_edge(&mut graph, &fighter_nodes, 2, 4); // Jose Aldo vs. Nate Diaz

    // Calculate the centrality
    for (i, &node) in fighter_nodes.iter().enumerate() {
        let name = &fighters[i].name;
        let degree = graph.edges_directed(node, Direction::Outgoing).count() as f32;
        let closeness = 1.0 / degree;
        println!("The closeness centrality of {} is {:.2}", name, closeness);

        match name.as_str() {
            "Conor McGregor" => println!(
                "{} has the lowest centrality because he has fought with all other fighters in the network. In this context, a lower centrality value means a higher number of fights.",
                name
            ),
            "Dustin Poirier" | "Nate Diaz" => println!(
                "{} has a centrality of {:.2}, implying they had less fights compared to Conor McGregor but more than Khabib Nurmagomedov and Jose Aldo.",
                name, closeness
            ),
            "Khabib Nurmagomedov" | "Jose Aldo" => println!(
                "{} has the highest centrality of {:.2} as they have fought with the least number of fighters.",
                name, closeness
            ),
            _ => {}
        }
        println!("-----------------");
    }

    // Prepare data for visualization
    let mut data = Vec::new();
    for (i, &node) in fighter_nodes.iter().enumerate() {
        let degree = graph.edges_directed(node, Direction::Outgoing).count() as f64;
        data.push((i as f64, degree));
    }

    // Create a scatter plot
    let s = Plot::new(data).point_style(
        plotlib::style::PointStyle::new()
            .marker(PointMarker::Square)
            .colour("#DD3355"),
    );

    // Create a view and add the scatter plot to it
    let v = ContinuousView::new()
        .add(s)
        .x_label("Fighter Index")
        .y_label("Number of Fights");

    // Save the plot to an SVG file
    Page::single(&v).save("fighters_graph.svg").unwrap();
}
