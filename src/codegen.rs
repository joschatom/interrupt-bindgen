//! Code generation for the bindings.
//! 
//! This module contains the code generation for the bindings. It is responsible
//! for generating the code for the bindings, and the inline assembly for the
//! bindings.

use crate::{spec::{Bindgen, Binding}, set_current_binding, reset_current_binding};
use super::{Code, verbose_println };

/// Generates the code for the bindings.
/// ===================================
/// 
/// This function generates the code for the bindings. It takes a reference to
/// the bindgen specification, and a reference to the binding specification. It
/// returns the generated code as a string.
/// 
/// # Arguments
/// 
/// * `bindgen` - The bindgen specification.
/// * `binding` - The binding specification.
/// 
/// # Returns
/// 
/// The generated code as a string.
pub fn generate_binding(bindgen: &Bindgen, binding: &Binding) -> Code {
    verbose_println(format!("Generating binding for {}", binding.name));

    set_current_binding(binding.name.clone());

    let mut code = String::new();

    verbose_println("Adding attributes: inline(never), no_mangle");
    code.push_str("#[inline(never)]\n");
    code.push_str("#[no_mangle]\n");

    let doc = format!(
        r#"Calls the Function "{name}" with the arguments "{args}"."#,
        name = binding.name,
        args = binding
            .args
            .iter()
            .map(|a| a.name.clone())
            .collect::<Vec<String>>()
            .join(", ")
    );

    verbose_println(format!("Adding documentation: {}", doc));
    code.push_str(format!("#[doc = {:?}]\n", doc).as_str());

    verbose_println(format!("Starting function definition for {}", binding.name));
    code.push_str(format!("pub unsafe extern \"C\" fn {}(\n", binding.name).as_str());

    for arg in &binding.args {
        verbose_println(format!(
            "Generating argument {} of type {}",
            arg.name, arg.ty
        ));
        code.push_str(format!("{name}: {ty},\n", name = arg.name, ty = arg.ty).as_str());
    }

    verbose_println("Closing function definition");
    code.push_str(format!(") -> {}\n{{\n", binding.ret).as_str());

    verbose_println("Generating inline assembly");
    code.push_str(generate_inline_asm(bindgen, binding).as_str());

    verbose_println("Finalizing function");
    code.push_str(
        format!(
            "return _ret & 0x{:x} }}\n",
            (bindgen.function_sig.map(|m| (m as u32) << 16).unwrap_or(0) | binding.offset as u32)
        )
        .as_str(),
    );

    verbose_println("Binding generation complete");
    
    reset_current_binding();

    code
}

/// Generates the inline assembly for the binding.
/// ==============================================
/// 
/// This function generates the inline assembly for the binding. It takes a
/// reference to the bindgen specification, and a reference to the binding
/// specification. It returns the generated inline assembly as a string.
/// 
/// # Arguments
/// 
/// * `bindgen` - The bindgen specification.
/// * `binding` - The binding specification.
/// 
/// # Returns
/// 
/// The generated inline assembly and rust code wrapping it(e.g for return values) as a string.
/// 
/// # Notes
/// 
/// This function is responsible for generating the inline assembly for the
/// binding, this means it's generating **unsafe** code!
/// 
/// # Safety
/// In general, this function can generate code that leads to **UB** if
/// the binding specification is incorrect.
pub fn generate_inline_asm(bindgen: &Bindgen, binding: &Binding) -> Code {
    verbose_println(format!("Generating inline assembly for {}", binding.name));

    let mut code = format!("let mut _ret: {};\n", binding.ret);
    verbose_println("Adding assembly code");
    code.push_str("::core::arch::asm!(\n");

    verbose_println(format!(
        "Adding interrupt number: 0x{:x}",
        bindgen.interrupt_number
    ));
    code.push_str(format!("\"int 0x{:x}\",\n", bindgen.interrupt_number).as_str());

    let binding_number =
        bindgen.function_sig.map(|m| (m as u32) << 16).unwrap_or(0) | binding.offset as u32;

    verbose_println(format!(
        "Adding function register: {:?}",
        bindgen.function_register
    ));
    code.push_str(
        format!(
            "in({:?}) 0x{:x},\n",
            bindgen.function_register, binding_number
        )
        .as_str(),
    );

    for arg in &binding.args {
        verbose_println(format!(
            "Adding argument: {}, register: {:?}",
            arg.name, arg.reg
        ));
        code.push_str(format!("in({:?}) {},\n", arg.reg, arg.name).as_str());
    }

    verbose_println(format!(
        "Adding lateout register: {:?}",
        bindgen.function_register
    ));
    code.push_str(format!("lateout({:?}) _ret,\n", bindgen.function_register).as_str());

    verbose_println("Adding options: nostack, nomem, raw");
    code.push_str("options(nostack, nomem, raw)\n");

    verbose_println("Closing assembly code");
    code.push_str(");\n");

    verbose_println("Inline assembly generation complete");
    code
}
