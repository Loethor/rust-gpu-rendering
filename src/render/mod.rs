// src/render/mod.rs
//! The renderer: turns scene + camera + config into pixels.
mod config;
mod cpu;

pub use config::RenderConfig;
pub use cpu::render;
