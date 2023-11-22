use core::fmt::Debug;
use core::hash::Hash;
use std::{
    collections::{HashMap, HashSet, VecDeque},
    fmt::Display,
    rc::Rc,
};

pub struct Graph<V, E> {
    edges: HashMap<Rc<V>, HashMap<Rc<V>, Rc<E>>>,
}

impl<V: Hash + Eq, E> Graph<V, E> {
    pub fn new() -> Graph<V, E> {
        Graph {
            edges: HashMap::new(),
        }
    }

    /// Attempts to add a vertex to the graph.
    ///
    /// If the graph already contains the vertex. Nothing is
    /// done. The Rc of the vertex is returned as a way to communicate the key for the vertex as
    /// well as the data associated with it.
    ///
    /// This function can be used to grab the Rc of the vertex if unknown i.e.
    /// example
    /// ```
    /// let mut g = Graph::new();
    ///
    /// g.vertex(23);
    ///
    ///
    /// let start = g.vertex(23);
    /// ```
    pub fn add_vertex(&mut self, v: V) -> Rc<V> {
        let key = Rc::new(v);
        self.edges.entry(key.clone()).or_insert(HashMap::new());
        key
    }

    pub fn add_edge(&mut self, from: &Rc<V>, to: &Rc<V>, edge: E) -> Option<Rc<E>> {
        if let Some(edges) = self.edges.get_mut(from) {
            let edge = Rc::new(edge);
            edges.entry(to.clone()).or_insert(edge.clone());
            return Some(edge);
        }
        None
    }

    pub fn vertex(&self, v: V) -> Option<Rc<V>> {
        let key = Rc::new(v);
        if self.edges.contains_key(&key) {
            return Some(key);
        }
        None
    }

    pub fn edges(&self, v: &Rc<V>) -> Option<&HashMap<Rc<V>, Rc<E>>> {
        self.edges.get(v)
    }

    pub fn get_edge(&self, from: &Rc<V>, to: &Rc<V>) -> Option<&Rc<E>> {
        self.edges.get(from).and_then(|e| e.get(to))
    }
}

pub struct SearchIter<'a, V, E> {
    graph: &'a Graph<V, E>,
    source: Rc<V>,
    sink: Rc<V>,
    visited: HashSet<Rc<V>>,
    queue: VecDeque<Rc<V>>,
    is_bfs: bool,
}

impl<'a, V: Hash + Eq, E> Iterator for SearchIter<'a, V, E> {
    type Item = Rc<V>;

    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}

impl<V: Display, E: Display> Display for Graph<V, E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (k, v) in self.edges.iter() {
            writeln!(f, "{}", k)?;
            for (e, to) in v.iter() {
                writeln!(f, "\t--{}--> {}", e, to)?;
            }
        }
        Ok(())
    }
}

impl<V: Debug, E: Debug> Debug for Graph<V, E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (k, v) in self.edges.iter() {
            writeln!(f, "{:?}", k)?;
            for (e, to) in v.iter() {
                writeln!(f, "\t--{:?}--> {:?}", e, to)?;
            }
        }
        Ok(())
    }
}
