// Bloque 1 — Fundamentos Avanzados
// Compilar: rustc block1_fundamentos/mod.rs -o block1

mod ownership;
mod threads;
mod async_runtime;
mod error_handling;
mod functional;

pub use ownership::*;
pub use threads::*;
pub use async_runtime::*;
pub use error_handling::*;
pub use functional::*;

// ---- ownership.rs ----
pub mod ownership {
    /// Parser de logs que devuelve slices con lifetime explícito (sin clonar).
    pub struct LogParser<'a> {
        lines: Vec<&'a str>,
    }

    impl<'a> LogParser<'a> {
        pub fn new(input: &'a str) -> Self {
            let lines: Vec<&'a str> = input.lines().collect();
            Self { lines }
        }

        /// Devuelve una referencia prestada a la línea i (no clona).
        pub fn get(&self, i: usize) -> Option<&'a str> {
            self.lines.get(i).copied()
        }

        pub fn count(&self) -> usize {
            self.lines.len()
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn parser_returns_borrowed_slice() {
            let data = "error: disk full\nwarn: high mem\ninfo: ok";
            let parser = LogParser::new(data);
            assert_eq!(parser.get(0), Some("error: disk full"));
            assert_eq!(parser.count(), 3);
        }
    }
}

// ---- threads.rs ----
pub mod threads {
    use std::thread;

    /// Descarga N URLs en hilos paralelos y junta resultados.
    pub fn parallel_fetch(urls: &[&str]) -> Vec<String> {
        let mut handles = vec![];
        for url in urls {
            let url = url.to_string();
            handles.push(thread::spawn(move || {
                format!("fetched {}", url)
            }));
        }
        let mut results = vec![];
        for h in handles {
            results.push(h.join().unwrap());
        }
        results
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn joins_all_results() {
            let urls = ["a.com", "b.com", "c.com"];
            let res = parallel_fetch(&urls);
            assert_eq!(res.len(), 3);
            assert!(res.iter().all(|r| r.starts_with("fetched")));
        }
    }
}

// ---- error_handling.rs ----
pub mod error_handling {
    use thiserror::Error; // placeholder conceptual; en std puro se define sin thiserror

    #[derive(Debug, Error)]
    #[error("fetch failed: {0}")]
    pub struct FetchError(pub String);

    /// Propaga errores con `?` y tipo propio.
    pub fn fetch_with_retry(url: &str, retries: u32) -> Result<String, FetchError> {
        let mut last_err = None;
        for _ in 0..=retries {
            match try_fetch(url) {
                Ok(body) => return Ok(body),
                Err(e) => last_err = Some(e),
            }
        }
        Err(last_err.map_or_else(|| FetchError("unknown".into()), |e| e))
    }

    fn try_fetch(_url: &str) -> Result<String, FetchError> {
        Err(FetchError("network down".into()))
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn retry_propagates_error() {
            let res = fetch_with_retry("http://x", 2);
            assert!(res.is_err());
        }
    }
}

// ---- functional.rs ----
pub mod functional {
    /// Transformación perezosa de un stream de eventos.
    pub fn process_events(events: &[i32]) -> Vec<i32> {
        events
            .iter()
            .filter(|&&x| x > 0)
            .map(|&x| x * 2)
            .collect()
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn filters_and_maps() {
            assert_eq!(process_events(&[-1, 2, -3, 4]), vec![4, 8]);
        }
    }
}

// ---- async_runtime.rs ----
pub mod async_runtime {
    /// Servidor TCP simple que atiende conexiones concurrentes (Tokio).
    /// Nota: requiere [dependencies] tokio = { version = "1", features = ["full"] }.
    pub async fn run_listener(addr: &str) {
        // Usar tokio::net::TcpListener en proyecto real.
        let _ = addr;
    }

    #[cfg(test)]
    mod tests {
        #[test]
        fn placeholder() {}
    }
}
