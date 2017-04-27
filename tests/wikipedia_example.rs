extern crate mst;

use std::collections::HashMap;
use mst::kruskal_mst_gen;


#[test]
fn test_example_1() {
    let vertices : Vec<char> = vec!['a', 'b', 'c', 'd', 'e'];
    let mut weighted_edges = HashMap::new();
    weighted_edges.insert(('a', 'e'), 1);
    weighted_edges.insert(('a', 'b'), 3);
    weighted_edges.insert(('b', 'e'), 4);
    weighted_edges.insert(('b', 'c'), 5);
    weighted_edges.insert(('c', 'e'), 6);
    weighted_edges.insert(('c', 'd'), 7);
    weighted_edges.insert(('e', 'd'), 7);

    let edges_ordered : Vec<(char, char)> = vec![('a', 'e'), ('a', 'b'), ('b', 'e'), ('b', 'c'),
                                                 ('c', 'e'), ('c', 'd'), ('e', 'd')];


    let tree : Vec<(char, char)> = kruskal_mst_gen(&vertices, &edges_ordered);
    let expected_tree : Vec<(char, char)> = vec![('a', 'e'), ('a', 'b'), ('b', 'c'), ('c', 'd')];

    let iter = tree.iter().zip(expected_tree.iter());
    for (edge, expected_edge) in iter {
        assert_eq!(edge, expected_edge);
    }

}