extern crate disjoint_set;
extern crate revord;

use std::hash::Hash;
use disjoint_set::DisjointSet;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::BinaryHeap;
use revord::RevOrd;

// FIXME: Maybe there too many .clone() that should not be there
// need to understand better borrowing and ownership :|


pub fn kruskal_mst_ordered<T>(vertices: &Vec<T>, edges_ordered: &Vec<(T, T)>) -> Vec<(T, T)>
where T: Clone + Hash + Eq
{
    let mut dset = DisjointSet::<T>::new();
    let mut tree : Vec<(T, T)> = Vec::new();

    for vertice in vertices {
        dset.make_set(vertice.clone());
    }

    for edge in edges_ordered.iter() {
        let (v1, v2) : (T, T) = edge.clone();

        if dset.find(v1.clone()) != dset.find(v2.clone()) {
            tree.push(edge.clone());
            let res = dset.union(v1, v2);

            // We don't care about what's adjacency_hashurned
            match res {
                Ok(_) => {},
                Err(_) => {},
             }
        }
    }
    tree
}

pub fn kruskal_mst<T, K>(vertices: &Vec<T>, edges: &HashMap<(T, T), K>) -> Vec<(T, T)>
where T: Clone + Hash + Eq , K: Eq + Ord
{

    // Put (edge, weight) of HashMap in vector to sort by weight in
    // ascending order
    let mut order: Vec<(&(T, T), &K)> = edges.iter().collect();
    order.sort_by(|&(_, a), &(_, b)| a.cmp(&b));

    // Extract the edges and put them in a new vector, keeping the order
    let mut edges_ordered : Vec<(T,T)> = Vec::with_capacity(order.len());
    for (edge, _) in order{
        edges_ordered.push(edge.clone());
    }

    kruskal_mst_ordered(&vertices, &edges_ordered)
}


// pub fn prim_mst<T, K>(vertices: &Vec<T>, edges: &HashMap<(T,T), K>)
// where T: Clone + Hash + Eq + std::fmt::Display + std::cmp::Ord,
//       K: std::cmp::Ord + Clone
// {

    // let adjacency_hash = adjacency_hash_from_edges(edges);

    // let mut t_vertices = HashSet::<&T>::with_capacity(vertices.len());
    // let mut t_edges : Vec<(T, T)> = Vec::new();

    // let mut pq = BinaryHeap::<(RevOrd<K>, (T, T))>::new();

    // start from 1st vertex:
    // Add it to the tree set
    // add its edges to the binary heap
    // t_vertices.insert(&vertices[0]);
    // let neighbours = adjacency_hash.get(&vertices[0]).unwrap();
    // for neighbour in neighbours {
    //     if t_vertices.contains(neighbour) {
    //         continue;
    //     }
    //     let edge_weight = edges.get(&(vertices[0].clone(), neighbour.clone())).unwrap();
    //     pq.push((RevOrd(edge_weight.clone()), (vertices[0].clone(), neighbour.clone()) ));
    // }


    // while t_vertices.len() != vertices.len() {
        // let (mut weight, (mut v1, mut v2));

        // Loop until we have an edges that where v1 and v2 are not in t_vertices
        // sounds bad
        // loop {
            // (weight, (v1, v2)) = pq.pop().unwrap();
        // }
    // }
// }

fn get_min_vertice(vertices_values: &Vec<i32>, tree_set: &HashSet<usize>) -> usize {
    let mut min = std::i32::MAX;
    let mut index = 0;
    for (i, value) in vertices_values.iter().enumerate() {
        if value < &min && !tree_set.contains(&i) {
            min = value.to_owned();
            index = i;
        }
    }
    index
}


pub fn prim_matrix(adjacency_matrix: Vec<Vec<i32>>) -> Vec<(i32, i32)> {
    let num_vertices = adjacency_matrix.len();
    let mut tree_edges = Vec::<(i32, i32)>::new();
    let mut t_vertices = HashSet::<usize>::with_capacity(num_vertices);

    let mut values = vec![std::i32::MAX; num_vertices];

    values[0] = 0;

    while t_vertices.len() != num_vertices {
        let min_vertice = get_min_vertice(&values, &t_vertices);
        t_vertices.insert(min_vertice);
        println!("t_vertices: {:?}", t_vertices);
        for (i, weight) in adjacency_matrix[min_vertice].iter().enumerate(){
            if t_vertices.contains(&i) || *weight < 0{
                continue;
            }
            println!("edge: ({}, {}) -> {}, i: {}", min_vertice, i, weight, *weight <= 0);
            if *weight < values[i] {
                values[i] = *weight;
                tree_edges.push((min_vertice as i32, i as i32));
                t_vertices.insert(i);
            }
        }

    }
    tree_edges
}


fn adjacency_hash_from_edges<T, K>(edges: &HashMap<(T, T), K>) -> HashMap<T, Vec<T>>
where T: Eq + Hash + Clone
{
    let mut adjacency_hash = HashMap::new();

    for (edge, _) in edges {
        let (v1, v2) = edge.clone();
        {
         let neighbours = adjacency_hash.entry(v1.clone()).or_insert(Vec::<T>::new());
         neighbours.push(v2.clone());
        }
        {
         let neighbours = adjacency_hash.entry(v2.clone()).or_insert(Vec::<T>::new());
         neighbours.push(v1.clone());
        }
    }


    adjacency_hash
}

#[test]
fn hey() {
    let mut pq = BinaryHeap::new();


    pq.push((RevOrd(1), (1, 2)));
    pq.push((RevOrd(0), (1, 0)));
    pq.push((RevOrd(2), (5, 2)));

    let (r, v) = pq.pop().unwrap();

    println!("Popped: {}, {}, {}",  r.0, v.0, v.1);

    // assert!(false);
    assert!(RevOrd(1) > RevOrd(2));
    assert!(RevOrd(1) < RevOrd(0));
}

