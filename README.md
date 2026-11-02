# learn-rust

Plan de aprendizaje Rust 2026 (edition 2024). Ejercicios organizados por bloques:

| Bloque | Contenido | Archivo |
|--------|-----------|---------|
| 1 | Fundamentos (ownership, lifetimes, threads, async, errores, closures) | `src/block1_fundamentos/mod.rs` |
| 2 | Estructuras de datos (ring buffer, listas, mapas, heap, BST, grafo, trie) | `src/block2_estructuras/mod.rs` |
| 3 | Algoritmia (Big O, recursividad, búsqueda, ordenado, DP) | `src/block3_algoritmia/mod.rs` |
| 4 | Pruebas y calidad (unit, integración, doctests, mocks, TDD) | `src/block4_practicas/mod.rs` |
| 5 | Patrones de diseño y arquitectura | `src/block5_patrones/mod.rs` |
| Capstone | CLI de monitoreo de servicios | `src/capstone/main.rs` |

## Requisitos

- Rust toolchain (edition 2024): https://rustup.rs/
  ```bash
  rustup default stable
  ```

## Compilar y ejecutar

Cada bloque es un archivo independiente compilable con `rustc`:

```bash
# Bloque 1
rustc --edition 2024 src/block1_fundamentos/mod.rs -o /tmp/block1 && /tmp/block1

# Tests de un bloque
rustc --edition 2024 --test src/block1_fundamentos/mod.rs -o /tmp/block1_test && /tmp/block1_test

# Test de integración (bloque 4)
rustc --edition 2024 --test src/block4_practicas/mod.rs tests/integration_test.rs -o /tmp/integration && /tmp/integration

# Capstone
rustc --edition 2024 src/capstone/main.rs -o /tmp/capstone && /tmp/capstone
```

Más detalles en `PLAN_RUST_2026.md`.

## Licencia

MIT
