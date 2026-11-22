//! Cáscara de ciclo de vida y persistencia del Core de escritorio.
//!
//! El estado de readiness en memoria describe el proceso vivo. Este crate
//! conserva aparte las transiciones para que Tauri pueda reanudar un stream
//! después de reiniciar el Core.

mod persistence;

pub mod public_interface;
