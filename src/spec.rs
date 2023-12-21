//! Bindgen Spec File
//! 
//! This file is used to configure bindgen for the project. It is used to

// Path: src/spec.rs

use std::path::PathBuf;


#[derive(Debug, Clone, serde::Deserialize)]
pub struct ArgSpec{
    pub name: String,
    pub reg: String,
    pub ty: String,
}

/// A binding specification
#[derive(Debug, Clone, serde::Deserialize)]
pub struct Binding{
    pub name: String,
    pub offset: u32,
    pub args: Vec<ArgSpec>,
    pub ret: String,
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct Bindgen{
    pub interrupt_number: u16,
    pub function_sig: Option<u16>,
    pub bindings: Vec<Binding>,
    pub function_register: String,
}

pub fn load(file: PathBuf) -> Bindgen{
    toml::from_str(
        std::fs::read_to_string(file).unwrap().as_str()
    ).unwrap()
}
