/* Community detection within social networks using Twitter/X data.
Utilising Kosaraju's algorithm to detect Strongly Connected Components (SCCs).
Kosaraju’s algorithm is a two-pass depth-first search (DFS) approach to find SCCs
in a directed graph. An SCC is a maximal subset of a graph where every node is
reachable from every other node in that subset.
*/

use community_detection_kosaraju_algo::TWITTER_USERNAMES;
use petgraph::algo::kosaraju_scc;
use petgraph::prelude::*;
use std::collections::HashMap;

/// Analyzes the graph by detecting strongly connected components (SCCs),
/// and prints the number of communities, community sizes, and the largest community.
///
/// # Arguments
///
/// * `graph` - A directed graph with Twitter usernames as nodes.
/// * `description` - A label to describe the analysis (e.g., "Initial dataset", "Expanded dataset").
///
/// # Example
///
/// ```
/// let graph = build_graph(&["user1", "user2", "user3", "user1"]);
/// analyze_graph(&graph, "Test Dataset");
/// ```
///
fn analyze_graph(graph: &DiGraph<&str, &str>, description: &str) {
    // Use Kosaraju's algorithm to detect SCCs
    let scc = kosaraju_scc(graph);

    // Print number of communities
    println!("{}: Detected {} communities", description, scc.len());

    // Print size of each community
    let mut community_sizes: Vec<usize> = scc.iter().map(|component| component.len()).collect();
    community_sizes.sort_unstable();
    println!("{}: Community sizes: {:?}", description, community_sizes);

    // Find and print the largest community
    if let Some(largest_scc) = scc.iter().max_by_key(|component| component.len()) {
        println!(
            "{}: Largest community has {} nodes",
            description,
            largest_scc.len()
        );
        let usernames: Vec<&str> = largest_scc
            .iter()
            .map(|&node_index| graph[node_index])
            .collect();
        println!(
            "{}: Users in the largest community: {:?}",
            description, usernames
        );
    }
}

/// Builds a directed graph from a list of Twitter usernames.
/// The graph represents retweet or mention interactions between users.
///
/// # Arguments
///
/// * `usernames` - A slice of Twitter usernames, where consecutive pairs represent a mention/retweet interaction.
///
/// # Returns
///
/// A directed graph (`DiGraph`) where each node is a Twitter username, and each edge represents an interaction.
///
/// # Example
///
/// ```
/// let graph = build_graph(&["user1", "user2", "user3", "user1"]);
/// assert_eq!(graph.node_count(), 3);
/// assert_eq!(graph.edge_count(), 2);
/// ```

/*'a: The lifetime parameter 'a ensures that the lifetime of references in the usernames slice,
and the references to nodes and edges in the graph, all have the same lifetime. This guarantees
that the data referenced by the graph is valid for as long as the usernames slice is valid,
preventing dangling references.
*/
fn build_graph<'a>(usernames: &'a [&'a str]) -> DiGraph<&'a str, &'a str> {
    // Create a new directed Graph
    let mut graph = DiGraph::<&'a str, &'a str>::new();
    // Create a HashMap to store node indices by user name
    let mut nodes = HashMap::new();

    // Iterate over the data to populate the graph
    for window in usernames.windows(2) {
        let user = window[0];
        let mention = window[1];

        // Add the nodes to the graph and to the HashMap
        let user_node = *nodes.entry(user).or_insert_with(|| graph.add_node(user));
        let mention_node = *nodes
            .entry(mention)
            .or_insert_with(|| graph.add_node(mention));

        graph.add_edge(user_node, mention_node, "retweets");
    }

    graph // return the graph
}

/// Main function to run the community detection analysis on Twitter user data.
///
/// This function analyzes both an initial dataset of 140 usernames and an expanded dataset of 170 usernames.
/// It builds graphs from these datasets and compares the community structures between them.
///
/// # Example
///
/// ```
/// main();
/// ```
///
fn main() {
    // Initial dataset analysis
    let initial_graph = build_graph(&TWITTER_USERNAMES[..140]); // Original 140 usernames
    analyze_graph(&initial_graph, "Initial dataset");

    // Expanded dataset analysis
    let expanded_graph = build_graph(&TWITTER_USERNAMES); // Expanded dataset with 200 usernames
    analyze_graph(&expanded_graph, "Expanded dataset");
}
