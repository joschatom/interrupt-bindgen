use std::{path::PathBuf, str::FromStr};

mod codegen;
mod spec;

pub type Code = String;



static mut VERBOSE_MODE: bool = false;
static mut CURRENT_BINDING: Option<String> = None;

fn verbose_println<T: core::fmt::Display>(s: T){
    unsafe {
        if VERBOSE_MODE{
            println!("[{}] {}", get_current_binding(), s);
        }
    }
}

fn get_current_binding() -> String{
    unsafe {
        CURRENT_BINDING.clone().unwrap_or("BINDGEN".to_string())
    }
}

fn reset_current_binding(){
    unsafe {
        CURRENT_BINDING = None;
    }
}

fn set_current_binding(binding: String){
    unsafe {
        CURRENT_BINDING = Some(format!("BINDGEN | {}", binding.to_uppercase()));
    }
}


fn main() {

    let  mut args_cmd = clap::Command::new("bindgen")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Joscha Egloff <joscha.egloff@pm.me>")
        .about("Generates Bindings for Interrupts that call Functions using Offsets")
        .arg(
            clap::Arg::new("spec")
                .help("The specification file to use, defaults to bindgen.toml")
                .required(false)
                .default_value("bindgen.toml")
                .index(1)
        )
        .arg(
            clap::Arg::new("output")
                .help("The output file to use, defaults to bindings.rs")
                .required(false)
                .default_value("bindings.rs")
                .index(2)
        )
        .arg(
            clap::Arg::new("target")
                .help("The target triple to use, defaults to x86_64-unknown-none")
                .required(false)
                .default_value("x86_64-unknown-none")
                .index(3)
        )
        .arg(
            clap::Arg::new("no-build")
                .help("Only compile the output file, don't build it")
                .short('c')
                .long("no-build")
                .num_args(0)
                .action(clap::ArgAction::SetTrue)
        )
        .arg(
            clap::Arg::new("no-format")
                .help("Only format the output file, don't build it")
                .short('g')
                .long("no-format")
                .num_args(0)
                .action(clap::ArgAction::SetTrue)
        )
        .arg(
            clap::Arg::new("verbose")
                .help("Print verbose output")
                .short('v')
                .long("verbose")
                .num_args(0)
                .action(clap::ArgAction::SetTrue)
        );

    let mut args = args_cmd.get_matches();

    unsafe {
        VERBOSE_MODE = args.get_flag("verbose");
    }

    let spec_file = args.get_one::<String>("spec").unwrap();
    let output_file = args.get_one::<String>("output").unwrap();
    let target = args.get_one::<String>("target").unwrap();


    verbose_println("Loading spec file");
    let bindgen = spec::load(std::path::PathBuf::from(PathBuf::from_str(spec_file).unwrap()));

    let mut file_code = String::new();

    verbose_println("Adding header");
    format!(
        r#"//`! Bindgen Generated File, do not edit by hand!
        //! Bindgen Version: {}
        //! Bindgen Spec:
        {}
    "#,
        env!("CARGO_PKG_VERSION"),
        std::fs::read_to_string(spec_file).unwrap().lines().map(|s| format!("//! {}", s)).collect::<Vec<String>>().join("\n")
    );

    verbose_println("Adding no_std attribute");
    file_code.push_str(
        "#![no_std]\nextern crate core;\n"
    );

    for binding in &bindgen.bindings{
        file_code.push_str(
            codegen::generate_binding(&bindgen, binding).as_str()
        );
    }
    
    verbose_println("Adding panic handler, this is just for the compiler, it should never be called.");
    file_code.push_str(r#"
    #[panic_handler]
    fn panic(_info: &core::panic::PanicInfo) -> !{
        unsafe { ::core::hint::unreachable_unchecked() }
    }
    "#);

    println!("Writing output file");
    std::fs::write(output_file.clone(), file_code).unwrap();

    if !args.get_flag("no-format"){
        println!("Formatting output file");
        std::process::Command::new("rustfmt").arg(output_file.clone()).status().unwrap();
    }
    
    if !args.get_flag("no-build"){
        println!("Building output file");
        std::process::Command::new("rustc").args(&["--crate-type", "staticlib", "--edition", "2021", "--target", target.as_str(), "-C", "panic=abort", "-o",  format!("lib{}.a",output_file.as_str()).as_str(), output_file]).status().unwrap();
    }

    println!("[BINDGEN] Bindgen complete, library file: {}, output file: {}!", format!("lib{}.a",output_file.trim_end_matches(".rs")).as_str(), output_file.as_str());

    reset_current_binding();

    println!("Done");
}
