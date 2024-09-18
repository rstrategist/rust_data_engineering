/* an implementation of the Dijkstra's algorithm for finding the shortest path in a graph.
The algorithm is a classic graph traversal technique for finding the shortest path from a
single source node to all other nodes in the graph.
The example provided uses a graph representation of some landmarks in Lisbon, Portugal,
and calculates the shortest route from Belem Tower to Lisbon Cathedral.
The Dijkstra algorithm function returns a HashMap where the keys are the nodes and the values
are the shortest distances from the source node to those nodes.
Adapted to ask for user input of the source and target nodes.
*/

use petgraph::algo::dijkstra;
use petgraph::prelude::*;
use std::collections::HashMap;
use std::io;

fn main() {
    // Create a new undirected graph
    let mut graph = Graph::<&str, u32, Undirected>::new_undirected();

    // Add edges representing distances between landmarks
    let belem_tower = graph.add_node("Belem Tower");
    let monastery = graph.add_node("Jerónimos Monastery");
    let lx_factory = graph.add_node("LX Factory");
    let commerce_square = graph.add_node("Commerce Square");
    let lisbon_cathedral = graph.add_node("Lisbon Cathedral");

    graph.extend_with_edges([
        (belem_tower, monastery, 1), // The distance from Belem Tower to Jerónimos Monastery is 1 km
        (belem_tower, lx_factory, 3), // The distance from Belem Tower to LX Factory is 3 km
        (belem_tower, commerce_square, 7), // The distance from Belem Tower to Commerce Square is 7 km
        (monastery, lx_factory, 3), // The distance from Jerónimos Monastery to LX Factory is 3 km
        (monastery, commerce_square, 6), // The distance from Jerónimos Monastery to Commerce Square is 6 km
        (lx_factory, commerce_square, 5), // The distance from LX Factory to Commerce Square is 5 km
        (commerce_square, lisbon_cathedral, 1), // The distance from Commerce Square to Lisbon Cathedral is 1 km
    ]);

    // Create a map to associate node names with their corresponding graph nodes
    let mut node_map: HashMap<&str, NodeIndex> = HashMap::new();
    node_map.insert("Belem Tower", belem_tower);
    node_map.insert("Jerónimos Monastery", monastery);
    node_map.insert("LX Factory", lx_factory);
    node_map.insert("Commerce Square", commerce_square);
    node_map.insert("Lisbon Cathedral", lisbon_cathedral);

    // Print available nodes
    println!("Available locations to select from:");
    for node in node_map.keys() {
        println!("- {}", node);
    }

    // Get user input for start and end nodes
    let start_node = get_user_input("Enter the start location: ");
    let end_node = get_user_input("Enter the end location: ");

    // Check if the entered nodes exist in the graph
    if let (Some(&start), Some(&end)) = (
        node_map.get(start_node.as_str()),
        node_map.get(end_node.as_str()),
    ) {
        // Calculate the shortest path from the start node to the end node using Dijkstra's algorithm
        let distances = dijkstra(&graph, start, Some(end), |e| *e.weight());

        // Check if a path was found and print the shortest distance
        if let Some(distance) = distances.get(&end) {
            println!(
                "The shortest distance from {} to {} is {} km",
                start_node, end_node, distance
            );
        } else {
            println!("No route found from {} to {}.", start_node, end_node);
        }
    } else {
        println!("One or both of the specified nodes do not exist in the graph.");
    }
}

/// Function to get user input from the console
fn get_user_input(prompt: &str) -> String {
    let mut input = String::new();
    println!("{}", prompt);
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().to_string()
}

// // Calculate the shortest path from Belem Tower to Lisbon Cathedral using Dijkstra's algorithm
// // The parameter "e" is the edge_cost: This is a closure that takes an edge reference and returns the cost
// // (or weight) of that edge. Here, we've used |e| *e.weight(), which means the cost is the weight of the edge.
// //             dijkstra(&graph, source node, target node, closure returning weight of each edge);
// let node_map = dijkstra(&graph, belem_tower, Some(lisbon_cathedral), |e| *e.weight());

// // Check if a path was found and print the shortest distance
// if let Some(distance) = node_map.get(&lisbon_cathedral) {
//     println!(
//         "The shortest distance from Belem Tower to Lisbon Cathedral is {} km",
//         distance
//     );
// } else {
//     println!("No route found from Belem Tower to Lisbon Cathedral.");
// }
