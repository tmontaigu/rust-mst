extern crate disjoint_set;

use std::hash::Hash;
use disjoint_set::DisjointSet;


pub fn kruskal_mst_gen<T>(vertices: &Vec<T>, edges_ordered: &Vec<(T, T)>) -> Vec<(T, T)>
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
