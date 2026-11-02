// Proyecto Capstone — CLI de monitoreo de servicios.
// Compilar: rustc capstone/main.rs -o capstone --edition 2024

use std::collections::HashMap;
use std::thread;

mod domain;
mod infrastructure;
mod usecases;
mod cli;

pub use domain::*;
pub use infrastructure::*;
pub use usecases::*;
pub use cli::*;

mod domain {
    pub struct Service {
        pub name: String,
        pub url: String,
        pub deps: Vec<String>,
    }

    pub struct CheckResult {
        pub service: String,
        pub up: bool,
        pub latency_ms: u64,
    }
}

mod infrastructure {
    use super::domain::*;
    pub struct HttpProbe;
    impl HttpProbe {
        pub fn probe(&self, s: &Service) -> CheckResult {
            // Simulado: sin red real.
            CheckResult {
                service: s.name.clone(),
                up: true,
                latency_ms: 42,
            }
        }
    }
}

mod usecases {
    use super::domain::*;
    use super::infrastructure::HttpProbe;
    use std::collections::HashSet;

    pub struct Monitor {
        probe: HttpProbe,
        services: HashMap<String, Service>,
    }

    impl Monitor {
        pub fn new() -> Self {
            Self {
                probe: HttpProbe,
                services: HashMap::new(),
            }
        }

        pub fn add(&mut self, s: Service) {
            self.services.insert(s.name.clone(), s);
        }

        /// DFS: detecta ciclos entre dependencias de servicios.
        pub fn has_dependency_cycle(&self) -> bool {
            let mut white: HashSet<String> = self.services.keys().cloned().collect();
            let mut gray: HashSet<String> = HashSet::new();
            let mut black: HashSet<String> = HashSet::new();

            fn visit(
                name: &str,
                services: &HashMap<String, Service>,
                white: &mut HashSet<String>,
                gray: &mut HashSet<String>,
                black: &mut HashSet<String>,
            ) -> bool {
                white.remove(name);
                gray.insert(name.to_string());
                if let Some(s) = services.get(name) {
                    for dep in &s.deps {
                        if gray.contains(dep) {
                            return true;
                        }
                        if black.contains(dep) {
                            continue;
                        }
                        if visit(dep, services, white, gray, black) {
                            return true;
                        }
                    }
                }
                gray.remove(name);
                black.insert(name.to_string());
                false
            }

            while let Some(name) = white.iter().next() {
                if visit(name, &self.services, &mut white, &mut gray, &mut black) {
                    return true;
                }
            }
            false
        }

        pub fn check_all(&self) -> Vec<CheckResult> {
            let mut results = vec![];
            let handles: Vec<_> = self
                .services
                .values()
                .map(|s| {
                    let probe = self.probe;
                    let s = s.clone();
                    thread::spawn(move || probe.probe(&s))
                })
                .collect();
            for h in handles {
                results.push(h.join().unwrap());
            }
            results
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn detects_cycle() {
            let mut m = Monitor::new();
            m.add(Service {
                name: "a".into(),
                url: "http://a".into(),
                deps: vec!["b".into()],
            });
            m.add(Service {
                name: "b".into(),
                url: "http://b".into(),
                deps: vec!["a".into()],
            });
            assert!(m.has_dependency_cycle());
        }

        #[test]
        fn no_cycle() {
            let mut m = Monitor::new();
            m.add(Service {
                name: "a".into(),
                url: "http://a".into(),
                deps: vec![],
            });
            assert!(!m.has_dependency_cycle());
        }
    }
}

mod cli {
    use super::usecases::Monitor;
    use super::domain::Service;

    pub fn run() {
        let mut mon = Monitor::new();
        mon.add(Service {
            name: "api".into(),
            url: "http://api".into(),
            deps: vec!["db".into()],
        });
        mon.add(Service {
            name: "db".into(),
            url: "http://db".into(),
            deps: vec![],
        });

        if mon.has_dependency_cycle() {
            println!("ciclo detectado");
        } else {
            let res = mon.check_all();
            for r in res {
                println!("{} up={} latency={}ms", r.service, r.up, r.latency_ms);
            }
        }
    }
}

fn main() {
    cli::run();
}
