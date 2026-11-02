
# Plan de Aprendizaje Rust 2026 — Directo al Grano

Formato: bloque → concepto clave → caso de uso → ejercicio práctico. Edición 2026 (Rust edition 2024, async-native, no-std opcional).

## Bloque 1 — Fundamentos Avanzados (Memoria, Ejecución, Concurrencia)

| Tema | Concepto clave | Caso de uso | Ejercicio |
|------|---------------|-------------|-----------|
| Ownership, Borrow, Lifetime | Propiedad única, préstamo inmutable/mutable, `'a` lifetimes | Parser de logs que no clona líneas | Escribe `LogParser` que devuelve `&str` slices con lifetime explícito |
| Stack vs Heap | `Box`, `Vec`, `String`, `Rc`, `Arc` | Árbol de directorios compartido entre hilos | `Arc<Mutex<Node>>` para un grafo de carpetas leído concurrentemente |
| Tipos y Pattern Match | `enum`, `struct`, `match`, `if let`, `while let` | Motor de reglas de validación | Enum `ValidationError` + match exhaustivo sobre entrada JSON simulada |
| Error Handling | `Result<T, E>`, `Option<T>`, `?`, `thiserror`/`anyhow` | Cliente HTTP con reintentos | Función `fetch_with_retry` que propaga errores con `?` y tipo propio |
| Threads | `std::thread`, `spawn`, `join`, `move`, `Send`/`Sync` | Descarga paralela de URLs | Descarga 10 URLs en hilos y junta resultados con `join` |
| Async/Await | `tokio`, `async fn`, `.await`, `Select`, `JoinSet` | Servidor de chat con websockets | Tokio `TcpListener` que atiende conexiones concurrentes con `JoinSet` |
| Channels | `mpsc`, `broadcast`, `Mutex`/`RwLock` | Pipeline de procesamiento de video | Canal `mpsc` entre productor (descarga) y consumidor (codificación) |
| Funciones de orden superior | Closures, `map`, `filter`, `fold`, iteradores perezosos | Transformación de stream de eventos | Iterador perezoso que filtra y mapea eventos sin allocar |

## Bloque 2 — Estructuras de Datos (Lineales y No Lineales)

| Tema | Concepto clave | Caso de uso | Ejercicio |
|------|---------------|-------------|-----------|
| Vec / Array / Slice | `Vec<T>`, `[T; N]`, `&[T]` | Buffer de muestras de audio | Ring buffer con `Vec` y índices modulares |
| Linked List | `Option<Box<Node>>`, `Rc<RefCell<Node>>` (no-std opcional) | Historial de navegación | Lista doblemente enlazada con `unsafe` o `arena` crate |
| Stack / VecDeque | LIFO, FIFO | Deshacer/rehacer en editor | `VecDeque` con capacidad fija para historial de comandos |
| HashMap / HashSet | `K: Eq + Hash`, `V` | Contador de palabras en tiempo real | Stream de palabras → `HashMap<String, u64>` con `entry` API |
| Binary Heap | `BinaryHeap<T>` (`Ord`) | Cola de prioridad de tareas | Scheduler de jobs con prioridad dinámica |
| Árbol Binario / BST | `Option<Box<Node>>`, recorridos | Índice de búsqueda de archivos | BST de rutas con inserción y búsqueda `O(log n)` promedio |
| Grafos (BFS/DFS) | `Vec<Vec<usize>>` o `HashMap` de adyacencia, `VecDeque` / `Vec` stack | Ruteo de red, detección de ciclos | BFS para camino más corto entre dos nodos; DFS para detectar ciclo |
| Tries | Nodos con `HashMap<char, Node>` | Autocompletado de comandos | Trie que devuelve sugerencias con prefijo dado |

## Bloque 3 — Algoritmia y Lógica (Big O, Recursividad, Recorridos)

| Tema | Concepto clave | Caso de uso | Ejercicio |
|------|---------------|-------------|-----------|
| Complejidad Big O | Tiempo y espacio, notación asymptotic | Elegir algoritmo por tamaño de input | Anota Big O de cada ejercicio anterior y compara `Vec` vs `HashMap` |
| Recursividad | Caso base, recursivo, stack overflow, tail recursion | Parsing de JSON anidado | Parseador recursivo de JSON con `match` y `Option` |
| Búsqueda Binaria | Array ordenado, `O(log n)` | Buscar en índice de logs | `binary_search_by` sobre `Vec<LogEntry>` ordenado por timestamp |
| BFS | Cola (`VecDeque`), visitados | Ruta más corta en laberinto | Laberinto en matriz `Vec<Vec<char>>`, BFS devuelve camino |
| DFS | Pila recursiva o `Vec`, visitados | Topological sort de dependencias | Orden de compilación de crates dado grafo de dependencias |
| Ordenamiento | Quicksort/Mergesort conceptualmente (usar `sort_unstable` en práctica) | Ranking de puntuaciones | Implementa `quicksort` genérico y compáralo con `slice::sort_unstable` |
| Programación Dinámica (opcional) | Memoización, tablas | Secuencia de Fibonacci grande | Fibonacci con `HashMap` memoizado vs iterativo |

## Bloque 4 — Buenas Prácticas, Calidad y Testing

| Tema | Concepto clave | Caso de uso | Ejercicio |
|------|---------------|-------------|-----------|
| Testing unitario | `#[cfg(test)]`, `#[test]`, `assert!` | Validar lógica pura | Tests para cada función de algoritmia del Bloque 3 |
| Integration tests | `tests/` carpeta, crates públicos | API de biblioteca | Test de integración contra tu `LogParser` del Bloque 1 |
| Doctests | Ejemplos en doc comments que se ejecutan | Documentación ejecutable | Cada función pública con ejemplo compilable en docs |
| Mocks / Doubles | Traits + mockall o impl manual | Probar sin red/DB | Trait `Storage` con impl real y `MockStorage` en tests |
| TDD | Rojo → Verde → Refactor | Desarrollar feature nueva | Escribe test fallante primero, luego implementa `RingBuffer` |
| Clippy / fmt | `cargo clippy`, `cargo fmt --check` | Calidad de código | Correja todas las advertencias de clippy en tu proyecto |
| Perfilado | `cargo flamegraph`, `criterion` | Detectar cuellos de botella | Benchmark de BFS vs DFS en grafo grande con `criterion` |
| CI | GitHub Actions `cargo test --all-targets` | Calidad continua | Workflow que corre `test`, `clippy`, `fmt` en push |

## Bloque 5 — Patrones de Diseño y Arquitectura

| Tema | Concepto clave | Caso de uso | Ejercicio |
|------|---------------|-------------|-----------|
| Traits como interfaces | Polimorfismo estático/dinámico (`dyn Trait`) | Múltiples backends de storage | Trait `Storage` con impl `FsStorage` y `S3Storage` (`Box<dyn Storage>`) |
| Builder | Métodos encadenados, `Option` campos | Configuración compleja de cliente | `ClientBuilder` para configurar timeout, retries, headers |
| Factory | `fn create(type) -> Box<dyn Trait>` | Elegir implementación en runtime | Factory que devuelve `Box<dyn Compression>` (gzip/zstd/none) |
| Strategy | Trait inyectable, swap de algoritmos | Cambiar cifrado o compresión | Estrategia de serialización: JSON vs MessagePack vs TOML |
| Observer | Callbacks o canal mpsc | Notificaciones de eventos | Bus de eventos con `mpsc::Sender` y suscriptores |
| Adapter | Wrapper que adapta interfaz externa | Usar lib C de terceros | Adaptor seguro sobre binding C de `libcurl` o similar |
| Middleware | Capas que envuelven request/response | Servidor web con auth/logging | Tokio + tower-like middleware chain (impl manual) |
| Clean Architecture / Capas | Domain (no deps), Application, Infrastructure, Entry points | App de tareas con DB y CLI | Separa `domain` (entidades), `usecases`, `infrastructure` (SQLite), `cli` |

## Proyecto Capstone (Bloque 6 opcional)

Un CLI de monitoreo de servicios que use todo lo anterior:
- Async (tokio) para consultas HTTP concurrentes.
- Grafos para dependencias entre servicios (DFS detecta ciclos, BFS para impacto).
- Ownership sin clones en parsing de respuestas.
- Traits para múltiples proveedores (Prometheus, AWS CloudWatch).
- Builder para configuración.
- Tests unitarios + integración + doctests.
- CI con clippy/fmt.

## Flujo sugerido (2026)

1. Instala `rustup`, elige edition 2024.
2. Bloque 1 (2-3 semanas): property-based con `proptest` opcional.
3. Bloque 2-3 (3-4 semanas): implementa cada estructura y algoritmo desde cero sin crates externos salvo `rand`.
4. Bloque 4 (1-2 semanas): escribe tests antes de cada feature nueva.
5. Bloque 5 (2 semanas): refactoriza el capstone aplicando patrones.
6. Capstone: despliega un binario único con `cargo build --release`.

## Código de referencia

Cada bloque tiene su implementación lista en `src/`:

| Bloque | Archivo | Contenido |
|--------|---------|-----------|
| 1. Fundamentos | `src/block1_fundamentos/mod.rs` | Ownership, lifetimes, threads, async tokio, error handling, closures |
| 2. Estructuras de datos | `src/block2_estructuras/mod.rs` | Ring buffer, linked list, VecDeque, HashMap, BinaryHeap, BST, grafo (BFS/DFS), Trie |
| 3. Algoritmia | `src/block3_algoritmia/mod.rs` | Big O, recursividad JSON, búsqueda binaria, BFS laberinto, DFS topo sort, quicksort, Fibonacci memo |
| 4. Pruebas y calidad | `src/block4_practicas/mod.rs` | Unit tests, doctests, mocks (trait Storage), TDD RingBuffer, clippy notas |
| 5. Patrones y arquitectura | `src/block5_patrones/mod.rs` | Traits `Box<dyn>`, Builder, Factory, Strategy, Observer mpsc, Adapter C, Middleware, Clean Architecture |
| Capstone | `src/capstone/main.rs` | CLI de monitoreo: deps grafos (DFS ciclo), checks paralelos con hilos |

Cada módulo incluye `#[cfg(test)]` con sus propios tests. Ejecutar con:

```bash
# Requiere toolchain Rust (edition 2024). Si no está instalada:
#   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
#   rustup default stable

# Compilar un bloque individual (ej. bloque 1):
rustc --edition 2024 src/block1_fundamentos/mod.rs -o /tmp/block1 && /tmp/block1

# Ejecutar tests de un bloque:
rustc --edition 2024 --test src/block1_fundamentos/mod.rs -o /tmp/block1_test && /tmp/block1_test

# Test de integración (bloque 4):
rustc --edition 2024 --test src/block4_practicas/mod.rs tests/integration_test.rs -o /tmp/integration && /tmp/integration

# Capstone:
rustc --edition 2024 src/capstone/main.rs -o /tmp/capstone && /tmp/capstone
```

Con `cargo` (proyecto ya inicializado en este directorio):

```bash
cargo build
cargo test
cargo clippy --all-targets
cargo fmt --check
```
