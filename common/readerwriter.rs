//! This module defines common traits for reading and writing objects to an SNES game.

/// Reads all objects of the given type.
pub trait ReadObject<T> {
    fn read_objects(&self) -> T;
}

/// Writes all objects of the given type.
pub trait WriteObject<T> {
    fn write_objects(&mut self, objects: &T);
}
