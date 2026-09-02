// src/render/mod.rs
//! The renderer: turns scene + camera + config into pixels.
mod config;
mod cpu;
mod framebuffer;
pub mod shading;

pub use config::RenderConfig;
pub use cpu::render;
pub use framebuffer::Framebuffer;
