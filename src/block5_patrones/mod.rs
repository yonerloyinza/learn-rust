// Bloque 5 — Patrones de Diseño y Arquitectura
// Compilar: rustc block5_patrones/mod.rs -o block5

mod traits_interface;
mod builder;
mod factory;
mod strategy;
mod observer;
mod adapter;
mod middleware;
mod clean_arch;

pub use traits_interface::*;
pub use builder::*;
pub use factory::*;
pub use strategy::*;
pub use observer::*;
pub use adapter::*;
pub use middleware::*;
pub use clean_arch::*;

// ---- traits_interface.rs ----
pub mod traits_interface {
    /// Polimorfismo con traits y Box<dyn Trait>.
    pub trait Storage {
        fn save(&mut self, key: &str, val: &str);
        fn load(&self, key: &str) -> Option<String>;
    }

    pub struct FsStorage;
    impl Storage for FsStorage {
        fn save(&mut self, key: &str, val: &str) {
            println!("fs save {}={}", key, val);
        }
        fn load(&self, key: &str) -> Option<String> {
            println!("fs load {}", key);
            None
        }
    }

    pub struct S3Storage;
    impl Storage for S3Storage {
        fn save(&mut self, key: &str, val: &str) {
            println!("s3 save {}={}", key, val);
        }
        fn load(&self, key: &str) -> Option<String> {
            println!("s3 load {}", key);
            None
        }
    }

    pub fn run(backend: Box<dyn Storage>) {
        let _ = backend;
    }
}

// ---- builder.rs ----
pub mod builder {
    /// Builder para configuración compleja de cliente HTTP.
    pub struct Client {
        pub timeout: u64,
        pub retries: u32,
        pub headers: Vec<(String, String)>,
    }

    pub struct ClientBuilder {
        timeout: Option<u64>,
        retries: Option<u32>,
        headers: Vec<(String, String)>,
    }

    impl ClientBuilder {
        pub fn new() -> Self {
            Self {
                timeout: None,
                retries: None,
                headers: vec![],
            }
        }

        pub fn timeout(mut self, ms: u64) -> Self {
            self.timeout = Some(ms);
            self
        }

        pub fn retries(mut self, n: u32) -> Self {
            self.retries = Some(n);
            self
        }

        pub fn header(mut self, k: &str, v: &str) -> Self {
            self.headers.push((k.to_string(), v.to_string()));
            self
        }

        pub fn build(self) -> Client {
            Client {
                timeout: self.timeout.unwrap_or(30_000),
                retries: self.retries.unwrap_or(3),
                headers: self.headers,
            }
        }
    }

    impl Default for ClientBuilder {
        fn default() -> Self {
            Self::new()
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn builds_with_defaults() {
            let c = ClientBuilder::new().build();
            assert_eq!(c.timeout, 30_000);
            assert_eq!(c.retries, 3);
        }

        #[test]
        fn overrides() {
            let c = ClientBuilder::new().timeout(5_000).header("x", "y").build();
            assert_eq!(c.timeout, 5_000);
            assert_eq!(c.headers.len(), 1);
        }
    }
}

// ---- factory.rs ----
pub mod factory {
    /// Factory que elige implementación de compresión en runtime.
    pub trait Compression {
        fn compress(&self, data: &[u8]) -> Vec<u8>;
    }

    pub struct Gzip;
    impl Compression for Gzip {
        fn compress(&self, _data: &[u8]) -> Vec<u8> {
            vec![1, 2, 3] // placeholder
        }
    }

    pub struct NoneAlg;
    impl Compression for NoneAlg {
        fn compress(&self, data: &[u8]) -> Vec<u8> {
            data.to_vec()
        }
    }

    pub enum Algorithm {
        Gzip,
        None,
    }

    pub fn create(alg: Algorithm) -> Box<dyn Compression> {
        match alg {
            Algorithm::Gzip => Box::new(Gzip),
            Algorithm::None => Box::new(NoneAlg),
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn factory_returns_impl() {
            let g: Box<dyn Compression> = create(Algorithm::Gzip);
            assert_eq!(g.compress(b"data"), vec![1, 2, 3]);
            let n: Box<dyn Compression> = create(Algorithm::None);
            assert_eq!(n.compress(b"data"), b"data");
        }
    }
}

// ---- strategy.rs ----
pub mod strategy {
    /// Estrategia de serialización intercambiable.
    pub trait Serializer {
        fn serialize(&self, val: &str) -> String;
    }

    pub struct Json;
    impl Serializer for Json {
        fn serialize(&self, val: &str) -> String {
            format!("{{\"v\":{}}}", val)
        }
    }

    pub struct Toml;
    impl Serializer for Toml {
        fn serialize(&self, val: &str) -> String {
            format!("v = '{}'", val)
        }
    }

    pub struct Context {
        strategy: Box<dyn Serializer>,
    }

    impl Context {
        pub fn new(s: Box<dyn Serializer>) -> Self {
            Self { strategy: s }
        }

        pub fn run(&self, val: &str) -> String {
            self.strategy.serialize(val)
        }

        pub fn set_strategy(&mut self, s: Box<dyn Serializer>) {
            self.strategy = s;
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn swaps_strategy() {
            let mut ctx = Context::new(Box::new(Json));
            assert_eq!(ctx.run("42"), "{\"v\":42}");
            ctx.set_strategy(Box::new(Toml));
            assert_eq!(ctx.run("42"), "v = '42'");
        }
    }
}

// ---- observer.rs ----
pub mod observer {
    use std::sync::mpsc;

    /// Bus de eventos con canal mpsc y suscriptores.
    pub struct EventBus {
        tx: mpsc::Sender<String>,
        rx: mpsc::Receiver<String>,
    }

    impl EventBus {
        pub fn new() -> Self {
            let (tx, rx) = mpsc::channel();
            Self { tx, rx }
        }

        pub fn publisher(&self) -> mpsc::Sender<String> {
            self.tx.clone()
        }

        pub fn try_next(&self) -> Option<String> {
            self.rx.try_recv().ok()
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn publishes_and_receives() {
            let bus = EventBus::new();
            let pub_ = bus.publisher();
            pub_.send("event1".into()).unwrap();
            assert_eq!(bus.try_next(), Some("event1".into()));
        }
    }
}

// ---- adapter.rs ----
pub mod adapter {
    /// Adaptor seguro sobre binding C externo (ej. libcurl).
    extern "C" {
        fn curl_perform(url: *const u8) -> i32;
    }

    pub struct CurlAdapter;

    impl CurlAdapter {
        pub fn get(&self, url: &str) -> Result<String, String> {
            let c_url = std::ffi::CString::new(url).unwrap();
            unsafe {
                let code = curl_perform(c_url.as_ptr());
                if code == 0 {
                    Ok(format!("body of {}", url))
                } else {
                    Err(format!("curl error {}", code))
                }
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn adapter_wraps_unsafe() {
            let a = CurlAdapter;
            // No se puede llamar a curl_perform real en test sin lib; solo verifica compilación.
            let _ = a;
        }
    }
}

// ---- middleware.rs ----
pub mod middleware {
    /// Capas que envuelven request/response (manual, estilo tower).
    pub type Middleware = Box<dyn Fn(String) -> String>;

    pub fn logging(inner: Middleware) -> Middleware {
        Box::new(move |req| {
            println!("LOG: {}", req);
            inner(req)
        })
    }

    pub fn auth(inner: Middleware) -> Middleware {
        Box::new(move |req| {
            if req.contains("token=ok") {
                inner(req)
            } else {
                "401".into()
            }
        })
    }

    pub fn endpoint(req: String) -> String {
        format!("200 {}", req)
    }

    pub fn handle(req: String) -> String {
        let chain: Middleware = auth(logging(Box::new(endpoint)));
        chain(req)
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn middleware_chain() {
            assert!(handle("x".into()).contains("401"));
            assert!(handle("token=ok".into()).contains("200"));
        }
    }
}

// ---- clean_arch.rs ----
pub mod clean_arch {
    /// Separación: domain (sin deps), usecases, infrastructure, entry.
    pub mod domain {
        pub struct Task {
            pub id: u64,
            pub title: String,
            pub done: bool,
        }
    }

    pub mod infrastructure {
        use super::domain::Task;
        pub struct SqlRepo;
        impl SqlRepo {
            pub fn save(&self, _t: &Task) {
                println!("saved to sqlite");
            }
        }
    }

    pub mod usecases {
        use super::domain::Task;
        use super::infrastructure::SqlRepo;
        pub struct CreateTask {
            repo: SqlRepo,
        }
        impl CreateTask {
            pub fn new() -> Self {
                Self { repo: SqlRepo }
            }
            pub fn execute(&self, title: String) {
                let t = Task { id: 0, title, done: false };
                self.repo.save(&t);
            }
        }
    }

    pub mod cli {
        use super::usecases::CreateTask;
        pub fn run(title: String) {
            let uc = CreateTask::new();
            uc.execute(title);
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn cli_executes_usecase() {
            // Solo verifica que compila y no panica.
            cli::run("learn rust".into());
        }
    }
}
