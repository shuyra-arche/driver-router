#![allow(dead_code)]

extern crate self as ipc;
extern crate self as resource;

#[cfg(test)]
use env::{Mark, PieToken, TaskId};
#[cfg(test)]
use std::sync::atomic::Ordering;

mod facts;
pub mod raw;
pub mod session;

pub(crate) use facts::ACCEPTED;
#[cfg(test)]
pub(crate) use facts::{register, reset, token};

#[path = "../../client/src/handoff.rs"]
mod client_handoff;
#[path = "../../server/src/adapt/event/handoff.rs"]
mod handoff;

#[cfg(test)]
mod tests;
