extern crate mst;

use std::collections::HashMap;
use mst::kruskal_mst_ordered;
use mst::kruskal_mst;


#[test]
fn test_example_1() {
    let vertices : Vec<char> = vec!['a', 'b', 'c', 'd', 'e'];

    let edges_ordered : Vec<(char, char)> = vec![('a', 'e'), ('a', 'b'), ('b', 'e'), ('b', 'c'),
                                                 ('c', 'e'), ('c', 'd'), ('e', 'd')];

    let tree : Vec<(char, char)> = kruskal_mst_ordered(&vertices, &edges_ordered);
    let expected_tree : Vec<(char, char)> = vec![('a', 'e'), ('a', 'b'), ('b', 'c'), ('c', 'd')];

    let iter = tree.iter().zip(expected_tree.iter());

    assert_eq!(&expected_tree.len(), &tree.len());
    for (edge, expected_edge) in iter {
        assert_eq!(edge, expected_edge);
    }
}

#[test]
fn test_kruskal_hash_map_1() {
    let vertices : Vec<char> = vec!['a', 'b', 'c', 'd', 'e'];
    let mut weighted_edges = HashMap::new();
    weighted_edges.insert(('c', 'd'), 7);
    weighted_edges.insert(('b', 'c'), 5);
    weighted_edges.insert(('a', 'e'), 1);
    weighted_edges.insert(('e', 'd'), 7);
    weighted_edges.insert(('a', 'b'), 3);
    weighted_edges.insert(('c', 'e'), 6);
    weighted_edges.insert(('b', 'e'), 4);


    let tree : Vec<(char, char)> = kruskal_mst(&vertices, &weighted_edges);
    let expected_tree : Vec<(char, char)> = vec![('a', 'e'), ('a', 'b'), ('b', 'c'), ('c', 'd')];

    println!("{:?}", weighted_edges);
    assert_eq!(&expected_tree.len(), &tree.len());
    let iter = tree.iter().zip(expected_tree.iter());
    for (edge, expected_edge) in iter {
        assert_eq!(edge, expected_edge);
    }
}