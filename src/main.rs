//! VORTEX Shell - Main Entry Point
//! A living, intelligent desktop shell for Windows

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    vortex_lib::run();
}
