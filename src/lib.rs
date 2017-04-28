extern crate disjoint_set;

use std::hash::Hash;
use disjoint_set::DisjointSet;
use std::collections::HashMap;
use std::iter::FromIterator;


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

    // Put (edge, weight) of HashMap in vector to sort by weight in
    // ascending order
    let mut order: Vec<(&(T, T), &K)> = Vec::from_iter(edges);
    order.sort_by(|&(_, a), &(_, b)| a.cmp(&b));

    // Extract the edges and put them in a new vector, keeping the order
    let mut edges_ordered : Vec<(T,T)> = Vec::with_capacity(order.len());
    for (edge, _) in order{
        edges_ordered.push(edge.clone());
    }

    kruskal_mst_ordered(&vertices, &edges_ordered)
}


pub fn prim_mst<T, K>(vertices: &Vec<T>, edges: &HashMap<(T,T), K>)
where T: Clone + Hash + Eq + std::fmt::Display
{
    let mut adjacency_list = HashMap::<T,Vec<T>>::new();
    for (edge, _) in edges {
        let (v1, v2) = edge.clone();

        {
            let mut neighbours_v1 = adjacency_list.entry(v1.clone()).or_insert(Vec::<T>::new());
            neighbours_v1.push(v2.clone());
        }

        {
            let mut neighbours_v2 = adjacency_list.entry(v2).or_insert(Vec::<T>::new());
            neighbours_v2.push(v1.clone());
        }
    }

    // let mut t_vertices : Vec<T> = Vec::with_capacity(vertices.len());
    // let mut t_edges : Vec<(T, T)> = Vec::new();
    // t_vertices.push(vertices[0].clone());


    // let mut considered_edges : Vec<((T, T), K)> = Vec::new();
    // for vertice in &t_vertices {
    //     match adjacency_list.get(vertice) {
    //         Some(vec) => {
    //             for e in vec {
    //                 let (v1, v2) = (vertice.clone(), e.clone());
    //                 match edges.get(&(v1, v2)) {
    //                     Some(value) => considered_edges.push( ((v1, v2), value) ),
    //                     None => {},
    //                 }
    //             }
    //         },
    //         None => {}
    //     }
    // }


}

fn create_adjacency_hash<T, K>(edges: &HashMap<(T,T), K>) -> HashMap<T,Vec<T>>
where T: Eq + Hash
{
    let mut adjacency_hash = HashMap::<T, Vec<T>>::new();


    adjacency_hash
}