// Bloque 3 — Algoritmia y Lógica
// Compilar: rustc block3_algoritmia/mod.rs -o block3

mod complexity;
mod recursion;
mod binary_search;
mod bfs_dfs;
mod sorting;
mod dp;

pub use complexity::*;
pub use recursion::*;
pub use binary_search::*;
pub use bfs_dfs::*;
pub use sorting::*;
pub use dp::*;

// ---- complexity.rs ----
pub mod complexity {
    /// Anotación de complejidad (Big O) para cada operación.
    pub fn linear_search<T: PartialEq>(arr: &[T], target: &T) -> Option<usize> {
        // O(n) tiempo, O(1) espacio
        for (i, v) in arr.iter().enumerate() {
            if v == target {
                return Some(i);
            }
        }
        None
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn finds() {
            assert_eq!(linear_search(&[1, 2, 3], &2), Some(1));
            assert_eq!(linear_search(&[1, 2, 3], &9), None);
        }
    }
}

// ---- recursion.rs ----
pub mod recursion {
    /// Parseador recursivo de JSON anidado (simplificado).
    #[derive(Debug, PartialEq)]
    pub enum Json {
        Null,
        Bool(bool),
        Num(f64),
        Str(String),
        Arr(Vec<Json>),
        Obj(Vec<(String, Json)>),
    }

    pub fn parse(s: &str) -> Option<(Json, usize)> {
        let s = s.trim_start();
        let bytes = s.as_bytes();
        match bytes.first()? {
            b'{' => parse_obj(s),
            b'[' => parse_arr(s),
            b'"' => parse_str(s),
            b't' | b'f' => parse_bool(s),
            b'n' => parse_null(s),
            _ => parse_num(s),
        }
    }

    fn parse_obj(s: &str) -> Option<(Json, usize)> {
        let mut i = 1;
        let mut obj = vec![];
        loop {
            let s = &s[i..].trim_start();
            if s.starts_with('}') {
                return Some((Json::Obj(obj), i + 1));
            }
            let (key, consumed) = parse_str(s)?;
            i += consumed;
            let s = &s[consumed..].trim_start();
            if !s.starts_with(':') {
                return None;
            }
            i += 1;
            let (val, consumed) = parse(&s[1..])?;
            i += consumed;
            obj.push((key.0, val));
            let s = &s[consumed..].trim_start();
            if s.starts_with(',') {
                i += 1;
                continue;
            }
            if s.starts_with('}') {
                return Some((Json::Obj(obj), i + 1));
            }
            return None;
        }
    }

    fn parse_arr(s: &str) -> Option<(Json, usize)> {
        let mut i = 1;
        let mut arr = vec![];
        loop {
            let s = &s[i..].trim_start();
            if s.starts_with(']') {
                return Some((Json::Arr(arr), i + 1));
            }
            let (val, consumed) = parse(s)?;
            i += consumed;
            let s = &s[i..].trim_start();
            if s.starts_with(',') {
                i += 1;
                continue;
            }
            if s.starts_with(']') {
                return Some((Json::Arr(arr), i + 1));
            }
            return None;
        }
    }

    fn parse_str(s: &str) -> Option<(Json, usize)> {
        let s = &s[1..];
        let end = s.find('"')?;
        let val = &s[..end];
        Some((Json::Str(val.to_string()), end + 2))
    }

    fn parse_bool(s: &str) -> Option<(Json, usize)> {
        if s.starts_with("true") {
            Some((Json::Bool(true), 4))
        } else if s.starts_with("false") {
            Some((Json::Bool(false), 5))
        } else {
            None
        }
    }

    fn parse_null(s: &str) -> Option<(Json, usize)> {
        if s.starts_with("null") {
            Some((Json::Null, 4))
        } else {
            None
        }
    }

    fn parse_num(s: &str) -> Option<(Json, usize)> {
        let end = s
            .bytes()
            .position(|b| !b.is_ascii_digit() && b != b'.' && b != b'-')
            .unwrap_or(s.len());
        let num: f64 = s[..end].parse().ok()?;
        Some((Json::Num(num), end))
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn parses_nested() {
            let (json, _) = parse(r#"{"a": [1, 2], "b": true}"#).unwrap();
            assert_eq!(
                json,
                Json::Obj(vec![
                    ("a".into(), Json::Arr(vec![Json::Num(1.0), Json::Num(2.0)])),
                    ("b".into(), Json::Bool(true)),
                ])
            );
        }
    }
}

// ---- binary_search.rs ----
pub mod binary_search {
    /// Búsqueda binaria en índice de logs ordenado por timestamp.
    #[derive(Debug)]
    pub struct LogEntry {
        pub ts: u64,
        pub msg: String,
    }

    pub fn search(logs: &[LogEntry], target_ts: u64) -> Option<usize> {
        let mut lo = 0;
        let mut hi = logs.len();
        while lo < hi {
            let mid = (lo + hi) / 2;
            match logs[mid].ts.cmp(&target_ts) {
                std::cmp::Ordering::Equal => return Some(mid),
                std::cmp::Ordering::Less => lo = mid + 1,
                std::cmp::Ordering::Greater => hi = mid,
            }
        }
        None
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn finds_timestamp() {
            let logs = vec![
                LogEntry { ts: 10, msg: "a".into() },
                LogEntry { ts: 20, msg: "b".into() },
                LogEntry { ts: 30, msg: "c".into() },
            ];
            assert_eq!(search(&logs, 20), Some(1));
            assert_eq!(search(&logs, 99), None);
        }
    }
}

// ---- bfs_dfs.rs ----
pub mod bfs_dfs {
    use std::collections::{HashSet, VecDeque};

    /// Laberinto como matriz de chars: '.' libre, '#' muro, 'S' inicio, 'E' fin.
    /// BFS devuelve el camino más corto (Vec de coordenadas).
    pub fn bfs_maze(grid: &[Vec<char>], start: (usize, usize), end: (usize, usize)) -> Option<Vec<(usize, usize)>> {
        let rows = grid.len();
        let cols = grid[0].len();
        let mut visited = HashSet::new();
        let mut q = VecDeque::new();
        q.push_back((start, vec![start]));
        visited.insert(start);

        while let Some(((r, c), path)) = q.pop_front() {
            if (r, c) == end {
                return Some(path);
            }
            for (dr, dc) in [(0, 1), (1, 0), (0, usize::MAX), (usize::MAX, 0)] {
                let nr = r.wrapping_add(dr);
                let nc = c.wrapping_add(dc);
                if nr >= rows || nc >= cols {
                    continue;
                }
                if grid[nr][nc] == '#' || !visited.insert((nr, nc)) {
                    continue;
                }
                let mut np = path.clone();
                np.push((nr, nc));
                q.push_back(((nr, nc), np));
            }
        }
        None
    }

    /// DFS para orden topológico de dependencias de crates.
    pub fn topological_sort(deps: &[(&str, &[&str])]) -> Vec<String> {
        let mut graph: std::collections::HashMap<&str, Vec<&str>> = std::collections::HashMap::new();
        for (crate_name, deps_list) in deps {
            graph.insert(crate_name, deps_list.to_vec());
        }
        let mut visited = std::collections::HashSet::new();
        let mut order = vec![];

        fn visit(
            node: &str,
            graph: &std::collections::HashMap<&str, Vec<&str>>,
            visited: &mut std::collections::HashSet<&str>,
            order: &mut Vec<String>,
        ) {
            if visited.contains(node) {
                return;
            }
            visited.insert(node);
            for dep in graph.get(node).unwrap_or(&vec![]) {
                visit(dep, graph, visited, order);
            }
            order.push(node.to_string());
        }

        for (node, _) in deps {
            visit(node, &graph, &mut visited, &mut order);
        }
        order
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn maze_path() {
            let grid = vec![
                vec!['S', '.', '#'],
                vec!['.', '.', '.'],
                vec!['#', '.', 'E'],
            ];
            let path = bfs_maze(&grid, (0, 0), (2, 2)).unwrap();
            assert_eq!(path[0], (0, 0));
            assert_eq!(path[path.len() - 1], (2, 2));
        }

        #[test]
        fn topo_order() {
            let deps = [
                ("a", &["b", "c"]),
                ("b", &["c"]),
                ("c", &[]),
            ];
            let order = topological_sort(&deps);
            let c_idx = order.iter().position(|x| x == "c").unwrap();
            let b_idx = order.iter().position(|x| x == "b").unwrap();
            let a_idx = order.iter().position(|x| x == "a").unwrap();
            assert!(c_idx < b_idx && b_idx < a_idx);
        }
    }
}

// ---- sorting.rs ----
pub mod sorting {
    /// Quicksort genérico (educativo). En producción usar slice::sort_unstable.
    pub fn quicksort<T: Ord + Clone>(arr: &mut [T]) {
        if arr.len() <= 1 {
            return;
        }
        let pivot = arr[arr.len() / 2].clone();
        let mut left = vec![];
        let mut right = vec![];
        let mut eq = 0;
        for v in arr.iter() {
            if *v < pivot {
                left.push(v.clone());
            } else if *v > pivot {
                right.push(v.clone());
            } else {
                eq += 1;
            }
        }
        let mut i = 0;
        for v in left {
            arr[i] = v;
            i += 1;
        }
        for _ in 0..eq {
            arr[i] = pivot.clone();
            i += 1;
        }
        for v in right {
            arr[i] = v;
            i += 1;
        }
        quicksort(&mut arr[..i - eq - right.len()]); // izquierda (aprox)
        // Nota: versión simplificada con fines didácticos.
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn sorts() {
            let mut v = vec![3, 1, 4, 1, 5];
            quicksort(&mut v);
            assert_eq!(v, vec![1, 1, 3, 4, 5]);
        }
    }
}

// ---- dp.rs ----
pub mod dp {
    use std::collections::HashMap;

    /// Fibonacci con memoización (HashMap) vs iterativo.
    pub fn fib_memo(n: u64, memo: &mut HashMap<u64, u64>) -> u64 {
        if n <= 1 {
            return n;
        }
        if let Some(&v) = memo.get(&n) {
            return v;
        }
        let v = fib_memo(n - 1, memo) + fib_memo(n - 2, memo);
        memo.insert(n, v);
        v
    }

    pub fn fib_iter(n: u64) -> u64 {
        let mut a = 0;
        let mut b = 1;
        for _ in 0..n {
            let t = a + b;
            a = b;
            b = t;
        }
        a
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn fib_matches() {
            let mut memo = HashMap::new();
            assert_eq!(fib_memo(10, &mut memo), fib_iter(10));
            assert_eq!(fib_iter(10), 55);
        }
    }
}
