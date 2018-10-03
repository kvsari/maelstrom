//! # Maelstrom
//! An attempt at a multiple producer multiple consumer channel in rust.

/// # Inlet
/// It is the transmission (tx) pair in the channel.
#[derive(Clone)]
pub struct Inlet;

/// # Outlet
/// It is the receiver (rx) pair in the channel.
#[derive(Clone)]
pub struct Outlet;

/// Create a maelstrom channel. The transmitter `Inlet` and the receiver `Outlet` 
pub fn maelstrom() -> (Inlet, Outlet) {
    (Inlet, Outlet)
}
