// Test de integración para el bloque 4.
// Ubicación: tests/integration_test.rs
// Compilar: rustc --edition 2024 block4_practicas/mod.rs tests/integration_test.rs -o integration

use block4_practicas::integration::add;

#[test]
fn integration_add() {
    assert_eq!(add(2, 3), 5);
    assert_eq!(add(-1, 1), 0);
}
