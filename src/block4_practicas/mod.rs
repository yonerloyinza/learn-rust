// Bloque 4 — Buenas Prácticas, Calidad y Testing
// Compilar: rustc block4_practicas/mod.rs -o block4

mod unit_tests;
mod integration;
mod doctest_example;
mod mock;
mod tdd;
mod clippy_notes;
mod benchmark;

pub use unit_tests::*;
pub use integration::*;
pub use doctest_example::*;
pub use mock::*;
pub use tdd::*;
pub use clippy_notes::*;
pub use benchmark::*;

// ---- unit_tests.rs ----
pub mod unit_tests {
    /// Función pura probada con tests unitarios.
    pub fn is_even(n: i32) -> bool {
        n % 2 == 0
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn even_returns_true() {
            assert!(is_even(4));
        }

        #[test]
        fn odd_returns_false() {
            assert!(!is_even(5));
        }
    }
}

// ---- integration.rs ----
// Tests de integración viven en tests/integration_test.rs (ver archivo aparte).
pub mod integration {
    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }
}

// ---- doctest_example.rs ----
pub mod doctest_example {
    /// Calcula el doble de un número.
    ///
    /// # Ejemplo
    ///
    /// ```
    /// use block4_practicas::doctest_example::double;
    /// assert_eq!(double(21), 42);
    /// ```
    pub fn double(n: i32) -> i32 {
        n * 2
    }
}

// ---- mock.rs ----
pub mod mock {
    /// Trait de almacenamiento con impl real y mock para tests.
    pub trait Storage {
        fn save(&mut self, key: &str, val: &str);
        fn load(&self, key: &str) -> Option<String>;
    }

    pub struct RealStorage {
        map: std::collections::HashMap<String, String>,
    }

    impl RealStorage {
        pub fn new() -> Self {
            Self {
                map: std::collections::HashMap::new(),
            }
        }
    }

    impl Storage for RealStorage {
        fn save(&mut self, key: &str, val: &str) {
            self.map.insert(key.to_string(), val.to_string());
        }

        fn load(&self, key: &str) -> Option<String> {
            self.map.get(key).cloned()
        }
    }

    pub struct MockStorage {
        pub calls: Vec<String>,
    }

    impl MockStorage {
        pub fn new() -> Self {
            Self { calls: vec![] }
        }
    }

    impl Storage for MockStorage {
        fn save(&mut self, key: &str, _val: &str) {
            self.calls.push(format!("save({})", key));
        }

        fn load(&self, _key: &str) -> Option<String> {
            None
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn mock_records_calls() {
            let mut m = MockStorage::new();
            m.save("k", "v");
            assert_eq!(m.calls, vec!["save(k)".to_string()]);
        }
    }
}

// ---- tdd.rs ----
pub mod tdd {
    /// RingBuffer desarrollado con TDD: test primero, luego implementa.
    pub struct RingBuffer {
        buf: Vec<i32>,
        cap: usize,
        len: usize,
    }

    impl RingBuffer {
        pub fn new(cap: usize) -> Self {
            Self {
                buf: vec![0; cap],
                cap,
                len: 0,
            }
        }

        pub fn push(&mut self, v: i32) {
            if self.len < self.cap {
                self.buf[self.len] = v;
                self.len += 1;
            }
        }

        pub fn get(&self, i: usize) -> Option<i32> {
            if i < self.len {
                Some(self.buf[i])
            } else {
                None
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn respects_capacity() {
            let mut r = RingBuffer::new(2);
            r.push(1);
            r.push(2);
            r.push(3); // ignorado por capacidad
            assert_eq!(r.get(0), Some(1));
            assert_eq!(r.get(1), Some(2));
            assert_eq!(r.get(2), None);
        }
    }
}

// ---- clippy_notes.rs ----
pub mod clippy_notes {
    /// Ejemplo que clippy sugeriría mejorar: uso innecesario de &String.
    pub fn old_style(s: &String) -> usize {
        s.len()
    }

    /// Versión idiomática recomendada por clippy: &str.
    pub fn idiomatic(s: &str) -> usize {
        s.len()
    }
}

// ---- benchmark.rs ----
pub mod benchmark {
    /// Compara BFS vs DFS en grafo grande (usar criterion en proyecto real).
    pub fn dummy_bfs(n: usize) -> usize {
        n * 2
    }

    pub fn dummy_dfs(n: usize) -> usize {
        n * 2
    }
}
