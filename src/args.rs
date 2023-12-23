/// Make the clap command for the interrupt-bindgen tool.
/// 
/// Returns:
/// 
/// * The clap command
pub fn make_command() -> clap::Command{
    clap::Command::new("interrupt-bindgen")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Joscha Egloff <joscha.egloff@pm.me>")
        .about(include_str!("../about.txt"))
        .arg(
            clap::Arg::new("spec")
                .help("The specification file to use.")
                .required(true)
                .index(1)
        )
        .arg(
            clap::Arg::new("output")
                .help("The output file to use.")
                .required(false)
                .long("output")
                .short('o')
                .default_value("bindings.rs")
        )
        .arg(
            clap::Arg::new("target")
                .help("The target triple to use.")
                .required(false)
                .long("target")
                .default_value("x86_64-unknown-none")
        )
        .arg(
            clap::Arg::new("build-output")
                .help("The name of the output file to build.")
                .short('O')
                .long("build-output")
                .default_value("bindings.a")
        )
        .arg(
            clap::Arg::new("build-type")
                .help("The type of the output file to build.")
                .long("type")
                .default_value("staticlib")
        )
        .arg(
            clap::Arg::new("no-build")
                .help("Only generate the output file, don't build it.")
                .short('c')
                .long("no-build")
                .num_args(0)
                .action(clap::ArgAction::SetTrue)
        )
        .arg(
            clap::Arg::new("no-format")
                .help("Don't format the output file.")
                .short('g')
                .long("no-format")
                .num_args(0)
                .action(clap::ArgAction::SetTrue)
        )
        .arg(
            clap::Arg::new("verbose")
                .help("Print verbose output.")
                .short('v')
                .long("verbose")
                .num_args(0)
                .action(clap::ArgAction::SetTrue)
        )
}