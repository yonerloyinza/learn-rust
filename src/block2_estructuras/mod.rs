// Bloque 2 — Estructuras de Datos
// Compilar: rustc block2_estructuras/mod.rs -o block2

mod vec_ring;
mod linked_list;
mod deque;
mod hashmap;
mod heap;
mod bst;
mod graph;
mod trie;

pub use vec_ring::*;
pub use linked_list::*;
pub use deque::*;
pub use hashmap::*;
pub use heap::*;
pub use bst::*;
pub use graph::*;
pub use trie::*;

// ---- vec_ring.rs ----
pub mod vec_ring {
    /// Ring buffer de muestras de audio con índices modulares.
    pub struct RingBuffer<T> {
        buf: Vec<Option<T>>,
        cap: usize,
        head: usize,
        len: usize,
    }

    impl<T: Clone> RingBuffer<T> {
        pub fn new(cap: usize) -> Self {
            Self {
                buf: vec![None; cap],
                cap,
                head: 0,
                len: 0,
            }
        }

        pub fn push(&mut self, v: T) {
            self.buf[self.head] = Some(v);
            self.head = (self.head + 1) % self.cap;
            if self.len < self.cap {
                self.len += 1;
            }
        }

        pub fn len(&self) -> usize {
            self.len
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn overwrites_oldest() {
            let mut r = RingBuffer::new(3);
            r.push(1);
            r.push(2);
            r.push(3);
            r.push(4); // sobrescribe 1
            assert_eq!(r.len(), 3);
        }
    }
}

// ---- linked_list.rs ----
pub mod linked_list {
    /// Lista enlazada simple con Box (propiedad única).
    pub enum List<T> {
        Empty,
        Cons(T, Box<List<T>>),
    }

    impl<T> List<T> {
        pub fn new() -> Self {
            List::Empty
        }

        pub fn push(mut self, elem: T) -> Self {
            List::Cons(elem, Box::new(self))
        }

        pub fn len(&self) -> usize {
            match self {
                List::Empty => 0,
                List::Cons(_, rest) => 1 + rest.len(),
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn counts_elements() {
            let l: List<i32> = List::new().push(1).push(2).push(3);
            assert_eq!(l.len(), 3);
        }
    }
}

// ---- deque.rs ----
pub mod deque {
    use std::collections::VecDeque;

    /// Cola de historial de comandos con capacidad fija (deshacer/rehacer).
    pub struct CommandHistory {
        back: VecDeque<String>,
        fwd: VecDeque<String>,
        cap: usize,
    }

    impl CommandHistory {
        pub fn new(cap: usize) -> Self {
            Self {
                back: VecDeque::with_capacity(cap),
                fwd: VecDeque::new(),
                cap,
            }
        }

        pub fn do_cmd(&mut self, cmd: String) {
            self.back.push_back(cmd);
            if self.back.len() > self.cap {
                self.back.pop_front();
            }
            self.fwd.clear();
        }

        pub fn back_len(&self) -> usize {
            self.back.len()
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn respects_capacity() {
            let mut h = CommandHistory::new(2);
            h.do_cmd("a".into());
            h.do_cmd("b".into());
            h.do_cmd("c".into());
            assert_eq!(h.back_len(), 2);
        }
    }
}

// ---- hashmap.rs ----
pub mod hashmap {
    use std::collections::HashMap;

    /// Contador de palabras en tiempo real con API `entry`.
    pub fn count_words(words: &[&str]) -> HashMap<String, u64> {
        let mut map = HashMap::new();
        for w in words {
            *map.entry(w.to_string()).or_insert(0) += 1;
        }
        map
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn counts() {
            let m = count_words(&["a", "b", "a"]);
            assert_eq!(m.get("a"), Some(&2));
            assert_eq!(m.get("b"), Some(&1));
        }
    }
}

// ---- heap.rs ----
pub mod heap {
    use std::collections::BinaryHeap;
    use std::cmp::Reverse;

    /// Cola de prioridad de trabajos (menor número = mayor prioridad).
    pub struct JobScheduler {
        heap: BinaryHeap<Reverse<(u32, String)>>,
    }

    impl JobScheduler {
        pub fn new() -> Self {
            Self {
                heap: BinaryHeap::new(),
            }
        }

        pub fn push(&mut self, priority: u32, job: String) {
            self.heap.push(Reverse((priority, job)));
        }

        pub fn pop(&mut self) -> Option<String> {
            self.heap.pop().map(|Reverse((_, job))| job)
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn highest_priority_first() {
            let mut s = JobScheduler::new();
            s.push(5, "low".into());
            s.push(1, "high".into());
            s.push(3, "mid".into());
            assert_eq!(s.pop(), Some("high".into()));
            assert_eq!(s.pop(), Some("mid".into()));
        }
    }
}

// ---- bst.rs ----
pub mod bst {
    /// Árbol binario de búsqueda de rutas de archivo.
    pub struct Node {
        path: String,
        left: Option<Box<Node>>,
        right: Option<Box<Node>>,
    }

    impl Node {
        pub fn new(path: &str) -> Self {
            Self {
                path: path.to_string(),
                left: None,
                right: None,
            }
        }

        pub fn insert(root: &mut Option<Box<Node>>, path: &str) {
            match root {
                None => *root = Some(Box::new(Node::new(path))),
                Some(node) => {
                    if path < &node.path {
                        Node::insert(&mut node.left, path);
                    } else {
                        Node::insert(&mut node.right, path);
                    }
                }
            }
        }

        pub fn search(root: &Option<Box<Node>>, path: &str) -> bool {
            match root {
                None => false,
                Some(node) => {
                    if path == &node.path {
                        true
                    } else if path < &node.path {
                        Node::search(&node.left, path)
                    } else {
                        Node::search(&node.right, path)
                    }
                }
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn insert_and_search() {
            let mut root: Option<Box<Node>> = None;
            Node::insert(&mut root, "/a/b");
            Node::insert(&mut root, "/a/c");
            assert!(Node::search(&root, "/a/b"));
            assert!(!Node::search(&root, "/x"));
        }
    }
}

// ---- graph.rs ----
pub mod graph {
    use std::collections::{HashMap, HashSet, VecDeque};

    pub type Graph = HashMap<usize, Vec<usize>>;

    /// BFS: camino más corto entre dos nodos (número de saltos).
    pub fn bfs(graph: &Graph, start: usize, goal: usize) -> Option<usize> {
        if start == goal {
            return Some(0);
        }
        let mut visited = HashSet::new();
        let mut q = VecDeque::new();
        visited.insert(start);
        q.push_back((start, 0));
        while let Some((node, dist)) = q.pop_front() {
            for &nb in graph.get(&node).unwrap_or(&vec![]) {
                if nb == goal {
                    return Some(dist + 1);
                }
                if visited.insert(nb) {
                    q.push_back((nb, dist + 1));
                }
            }
        }
        None
    }

    /// DFS: detecta ciclo en grafo dirigido.
    pub fn has_cycle(graph: &Graph) -> bool {
        let mut white: HashSet<usize> = graph.keys().copied().collect();
        let mut gray: HashSet<usize> = HashSet::new();
        let mut black: HashSet<usize> = HashSet::new();

        fn dfs(
            node: usize,
            graph: &Graph,
            white: &mut HashSet<usize>,
            gray: &mut HashSet<usize>,
            black: &mut HashSet<usize>,
        ) -> bool {
            white.remove(&node);
            gray.insert(node);
            for &nb in graph.get(&node).unwrap_or(&vec![]) {
                if gray.contains(&nb) {
                    return true; // ciclo
                }
                if black.contains(&nb) {
                    continue;
                }
                if dfs(nb, graph, white, gray, black) {
                    return true;
                }
            }
            gray.remove(&node);
            black.insert(node);
            false
        }

        while let Some(&node) = white.iter().next() {
            if dfs(node, graph, &mut white, &mut gray, &mut black) {
                return true;
            }
        }
        false
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn bfs_shortest_path() {
            let mut g = Graph::new();
            g.insert(0, vec![1, 2]);
            g.insert(1, vec![3]);
            g.insert(2, vec![3]);
            g.insert(3, vec![]);
            assert_eq!(bfs(&g, 0, 3), Some(2));
        }

        #[test]
        fn detects_cycle() {
            let mut g = Graph::new();
            g.insert(0, vec![1]);
            g.insert(1, vec![2]);
            g.insert(2, vec![0]); // ciclo
            assert!(has_cycle(&g));

            let mut g2 = Graph::new();
            g2.insert(0, vec![1]);
            g2.insert(1, vec![2]);
            g2.insert(2, vec![]);
            assert!(!has_cycle(&g2));
        }
    }
}

// ---- trie.rs ----
pub mod trie {
    use std::collections::HashMap;

    pub struct Node {
        children: HashMap<char, Node>,
        is_end: bool,
    }

    impl Node {
        pub fn new() -> Self {
            Self {
                children: HashMap::new(),
                is_end: false,
            }
        }
    }

    pub struct Trie {
        root: Node,
    }

    impl Trie {
        pub fn new() -> Self {
            Self { root: Node::new() }
        }

        pub fn insert(&mut self, word: &str) {
            let mut node = &mut self.root;
            for ch in word.chars() {
                node = node.children.entry(ch).or_insert(Node::new());
            }
            node.is_end = true;
        }

        pub fn starts_with(&self, prefix: &str) -> Vec<String> {
            let mut node = &self.root;
            for ch in prefix.chars() {
                match node.children.get(ch) {
                    Some(n) => node = n,
                    None => return vec![],
                }
            }
            let mut out = vec![];
            collect(node, prefix, &mut out);
            out
        }

        fn collect(node: &Node, prefix: &str, out: &mut Vec<String>) {
            if node.is_end {
                out.push(prefix.to_string());
            }
            for (ch, child) in &node.children {
                let mut next = prefix.to_string();
                next.push(*ch);
                collect(child, &next, out);
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn autocomplete() {
            let mut t = Trie::new();
            t.insert("hola");
            t.insert("holo");
            t.insert("adios");
            let sug = t.starts_with("ho");
            assert!(sug.contains(&"hola".to_string()));
            assert!(sug.contains(&"holo".to_string()));
            assert!(!sug.iter().any(|s| s == "adios"));
        }
    }
}
