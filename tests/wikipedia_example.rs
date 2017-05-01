extern crate mst;

use std::collections::HashMap;
use mst::{kruskal_mst_ordered, kruskal_mst, prim_matrix};

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
        assert!(index != None);
    }
}



#[test]
fn geek_for_geek_kruskal() {
   /* Let us create the following graph
          2    3
      (0)--(1)--(2)
       |   / \   |
      6| 8/   \5 |7
       | /     \ |
      (3)-------(4)
            9          */

    let vertices = vec![1, 2, 3, 4];
    let mut edges = HashMap::new();
    edges.insert((0, 1), 2);
    edges.insert((0, 3), 6);
    edges.insert((1, 3), 8);
    edges.insert((1, 2), 3);
    edges.insert((1, 4), 5);
    edges.insert((2, 4), 7);
    edges.insert((3, 4), 9);


    let tree = kruskal_mst(&vertices, &edges);

    let expected_tree = vec![(0, 1), (1, 2), (0, 3), (1, 4)];
    for edge in expected_tree {
        let index = tree.iter().position(|&e| e == edge);
        assert!(index != None);
    }
}





#[test]
fn geek_for_geek_prim() {
   /* Let us create the following graph
          2    3
      (0)--(1)--(2)
       |   / \   |
      6| 8/   \5 |7
       | /     \ |
      (3)-------(4)
            9          */

    let a = vec![0, 2, 0, 6, 0];
    let b = vec![2, 0, 3, 8, 5];
    let c = vec![0, 3, 0, 0, 7];
    let d = vec![6, 8, 0, 0, 9];
    let e = vec![0, 5, 7, 9, 0];

    let adj_mat = vec![a, b, c, d, e];

    let tree = prim_matrix(adj_mat);

    let expected_tree = vec![(0, 1), (1, 2), (0, 3), (1, 4)];
    for edge in expected_tree {
        let index = tree.iter().position(|&e| e == edge);
        assert!(index != None);
    }
}