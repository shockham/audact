/*!
Simple synth and sequencing lib
*/

#![deny(missing_docs)]

extern crate cpal;
extern crate futures;
extern crate rand;

/// Module for the main audact system
pub mod system;
/// Module containing note frequencies
pub mod notes;
