extern crate disjoint_set;

use std::hash::Hash;
use disjoint_set::DisjointSet;
use std::collections::HashMap;
use std::iter::FromIterator;


// FIXME: Maybe this .clone() should not be there
// need to understand better borrowing and ownership :|


pub fn kruskal_mst_ordered<T>(vertices: &Vec<T>, edges_ordered: &Vec<(T, T)>) -> Vec<(T, T)>
where T: Clone + Hash + Eq
{
    let mut dset = DisjointSet::<T>::new();
    let mut tree : Vec<(T, T)> = Vec::new();

    for vertice in vertices.iter() {
        dset.make_set(vertice.clone());
    }

    for edge in edges_ordered.iter() {
        let (v1, v2) : (T, T) = edge.clone();

        if dset.find(v1.clone()) != dset.find(v2.clone()) {
            tree.push(edge.clone());
            let res = dset.union(v1, v2);

            // We don't care about what's returned
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

    // Put (edge, weight) of HashMap in vector to set by weight in
    // ascending order
    let mut order: Vec<(&(T, T), &K)> = Vec::from_iter(edges);
    order.sort_by(|&(_, a), &(_, b)| a.cmp(&b));

    // Extract the edges in put the in a new vector, keeping the order
    let mut edges_ordered : Vec<(T,T)> = Vec::with_capacity(order.len());
    for (edge, _) in order{
        edges_ordered.push(edge.clone());
    }

    kruskal_mst_ordered(&vertices, &edges_ordered)
}
