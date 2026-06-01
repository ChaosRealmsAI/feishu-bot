pub(super) const AI_USAGE: &str = include_str!("../../docs/AI-USAGE.md");

mod communication;
mod data;
mod documents;
mod enterprise;
mod entry;

pub(super) use communication::*;
pub(super) use data::*;
pub(super) use documents::*;
pub(super) use enterprise::*;
pub(super) use entry::*;
