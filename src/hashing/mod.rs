pub mod p_hash;
pub mod d_hash;

// (Opcjonalnie, ale zalecane dla wygody)
// Re-eksportuje funkcję p_hash, aby była dostępna bezpośrednio
// jako `crate::hashing::p_hash` zamiast `crate::hashing::pHash::p_hash`
pub use p_hash::p_hash;
pub use d_hash::d_hash;