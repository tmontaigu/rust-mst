extern crate mst;

use std::collections::HashMap;
use mst::kruskal_mst_ordered;
use mst::kruskal_mst;

//https://en.wikipedia.org/wiki/File:MST_kruskal_en.gif

#[test]
fn test_example_1() {
    let vertices : Vec<char> = vec!['a', 'b', 'c', 'd', 'e'];

    let edges_ordered : Vec<(char, char)> = vec![('a', 'e'), ('c', 'd'), ('a', 'b'), ('b', 'e'), ('b', 'c'),
                                                 ('c', 'e'), ('e', 'd')];

    let tree : Vec<(char, char)> = kruskal_mst_ordered(&vertices, &edges_ordered);
    let expected_tree : Vec<(char, char)> = vec![('a', 'e'), ('c', 'd'), ('a', 'b'), ('b', 'c'), ];

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
    weighted_edges.insert(('c', 'd'), 2);
    weighted_edges.insert(('b', 'c'), 5);
    weighted_edges.insert(('a', 'e'), 1);
    weighted_edges.insert(('e', 'd'), 7);
    weighted_edges.insert(('a', 'b'), 3);
    weighted_edges.insert(('c', 'e'), 6);
    weighted_edges.insert(('b', 'e'), 4);

    let tree : Vec<(char, char)> = kruskal_mst(&vertices, &weighted_edges);
    let expected_tree : Vec<(char, char)> = vec![('a', 'e'), ('a', 'b'), ('b', 'c'), ('c', 'd')];

    assert_eq!(&expected_tree.len(), &tree.len());
    for edge in expected_tree {
        let index = tree.iter().position(|&e| e == edge);
        println!("edge: ({}, {})", edge.0, edge.1);
        assert!(index != None);
        println!("found");
    }
}