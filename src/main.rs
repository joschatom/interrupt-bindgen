//! Interrupt Bindgen Tool
//!
//! MIT License
//!
//! Copyright (c) 2023 Joscha Egloff
//!
//! Permission is hereby granted, free of charge, to any person obtaining a copy
//! of this software and associated documentation files (the "Software"), to deal
//! in the Software without restriction, including without limitation the rights
//! to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
//! copies of the Software, and to permit persons to whom the Software is
//! furnished to do so, subject to the following conditions:
//!
//! The above copyright notice and this permission notice shall be included in all
//! copies or substantial portions of the Software.
//!
//! THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
//! IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
//! FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
//! AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
//! LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
//! OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
//! SOFTWARE.
//!

use std::{path::PathBuf, str::FromStr};

mod args;
mod codegen;
mod spec;

/// The type for code
///
/// # Example
/// ```rust
/// let code: Code = Code::from_str("println!(\"{}\", 0)").unwrap();
/// ```
///
/// # Output
/// ```rust
/// println!("{}", 0)
/// ```
pub type Code = String;

static mut VERBOSE_MODE: bool = false;
static mut CURRENT_BINDING: Option<String> = None;

/// Prints a message if verbose mode is enabled
///
/// # Arguments
///
/// * `s` - The message to print
///
///
/// # Example
/// verbose_println("Hello World");
/// ```
///
/// # Output
///
/// ```text
/// [BINDGEN] Hello World
/// ```
fn verbose_println<T: core::fmt::Display>(s: T) {
    unsafe {
        if VERBOSE_MODE {
            println!("[{}] {}", get_current_binding(), s);
        }
    }
}

/// Gets the current binding name
///
/// # Example
///
/// ```rust
/// set_current_binding("TEST".to_string());
/// assert_eq!(get_current_binding(), "TEST");
/// verbose_println("Hello World");
/// ```
///
/// # Output
/// [BINDGEN | TEST] Hello World
/// ```text
fn get_current_binding() -> String {
    unsafe { CURRENT_BINDING.clone().unwrap_or("BINDGEN".to_string()) }
}

/// Resets the current binding name
///
/// # Example
///
/// ```rust
///
/// set_current_binding("TEST".to_string());
/// assert_eq!(get_current_binding(), "TEST");
/// reset_current_binding();
/// ```
fn reset_current_binding() {
    unsafe {
        CURRENT_BINDING = None;
    }
}

/// Sets the current binding name
///
/// # Arguments
///
/// * `binding` - The binding name to set
///
/// # Example
///
/// ```rust
///
/// set_current_binding("TEST".to_string());
/// assert_eq!(get_current_binding(), "TEST");
///
/// ```
///
/// # Output
///
/// ```text
/// [BINDGEN | TEST] { ... }
/// ```
fn set_current_binding(binding: String) {
    unsafe {
        CURRENT_BINDING = Some(format!("BINDGEN | {}", binding.to_uppercase()));
    }
}

/// Main function
///
/// Binds the command line arguments and runs the bindgen
///
/// Arguments:
///
/// * `spec` - The specification file to use, defaults to bindgen.toml
/// * `output` - The output file to use, defaults to bindings.rs
/// * `target` - The target triple to use, defaults to x86_64-unknown-none
/// * `no-build` - Only compile the output file, don't build it
/// * `no-format` - Only format the output file, don't build it
/// * `verbose` - Print verbose output
///
/// It's not recommended to use this function directly, use the command line interface instead.
fn main() {
    let args_cmd = args::make_command();
    let args = args_cmd.get_matches();

    unsafe {
        VERBOSE_MODE = args.get_flag("verbose");
    }

    let spec_file = args.get_one::<String>("spec").unwrap();
    let output_file = args.get_one::<String>("output").unwrap();
    let build_type = args.get_one::<String>("build-type").unwrap();
    let build_output_file = args.get_one::<String>("build-output").unwrap();
    let target = args.get_one::<String>("target").unwrap();

    verbose_println("Loading spec file");
    let bindgen = spec::load(std::path::PathBuf::from(
        PathBuf::from_str(spec_file).unwrap(),
    ));

    let mut file_code = String::new();

    verbose_println("Adding header");
    format!(
        r#"//`! Bindgen Generated File, do not edit by hand!
        //! Bindgen Version: {}
        //! Bindgen Spec:
        {}
    "#,
        env!("CARGO_PKG_VERSION"),
        std::fs::read_to_string(spec_file)
            .unwrap()
            .lines()
            .map(|s| format!("//! {}", s))
            .collect::<Vec<String>>()
            .join("\n")
    );

    verbose_println("Adding no_std attribute");
    file_code.push_str("#![no_std]\nextern crate core;\n");

    for binding in &bindgen.bindings {
        file_code.push_str(codegen::generate_binding(&bindgen, binding).as_str());
    }

    verbose_println(
        "Adding panic handler, this is just for the compiler, it should never be called.",
    );
    file_code.push_str(
        r#"
    #[panic_handler]
    fn panic(_info: &core::panic::PanicInfo) -> !{
        unsafe { ::core::hint::unreachable_unchecked() }
    }
    "#,
    );

    println!("Writing output file");
    std::fs::write(output_file.clone(), file_code).unwrap();

    if !args.get_flag("no-format") {
        println!("Formatting output file");
        std::process::Command::new("rustfmt")
            .arg(output_file.clone())
            .status()
            .unwrap();
    }

    if !args.get_flag("no-build") {
        println!("Building output file");
        std::process::Command::new("rustc")
            .args(&[
                "--crate-type",
                build_type,
                "--edition",
                "2021",
                "--target",
                target.as_str(),
                "-C",
                "panic=abort",
                "-o",
                build_output_file,
                output_file.as_str(),
            ])
            .status()
            .unwrap();
    }

    println!(
        "[BINDGEN] Bindgen complete, library file: {}, output file: {}!",
        format!("{}.a", output_file.trim_end_matches(".rs")).as_str(),
        output_file.as_str()
    );

    reset_current_binding();

    println!("Done");
}
