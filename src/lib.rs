//! # Talos
//! An Experimental Symmetric Encryption Algorithm base on Cellular Automata
//!
//! Implementation of a novel [cellular
//! automata](https://en.wikipedia.org/wiki/Cellular_automaton) based symmetric encryption
//! algorithm.
#![feature(trait_alias)]

/// Module containing toroidal automata implementations.
pub mod automata;
/// Module exposing bit access and mutation methods for unsigned integer types.
pub mod bitwise;
/// High-level subroutines for encryption per the Talos protocol.
pub mod encrypt;
/// TODO!
pub mod key;
/// Module implementing binary matrix interfaces and implementations.
pub mod matrix;
/// Utilities for parsing String representations of binary matrices to binary matrix states.
/// See page 3 of RFC-0 for an example of such a String representation.
pub mod parse;
