use anyhow::Result;
use std::env;
use std::io::{self, Write};
use std::path::PathBuf;

use swipl::context::Context;
use swipl::init::{activate_main, initialize_swipl};
use swipl_info::{get_swipl_info, SwiplInfo};

mod prolog_engine;

#[cfg(test)]
mod tests;

fn main() {
    if let Err(err) = run() {
        eprintln!("error: {err}");
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let info = get_swipl_info();
    configure_runtime(&info);

    let activation = initialize_swipl().unwrap_or_else(|| activate_main());
    let context: Context<_> = activation.into();

    // Initialize Prolog environment
    prolog_engine::PrologEngine::initialize(&context)?;

    // Load core modules
    prolog_engine::PrologEngine::load_core_modules(&context)?;

    // Load ontologies
    prolog_engine::PrologEngine::load_ontologies(&context, "rdf/ontologies")?;

    // Optionally load default scenarios
    // prolog_engine::PrologEngine::load_scenario(&context, "rdf/scenarios/network/simple_network.ttl")?;

    println!(
        "Embedded SWI-Prolog {} ({}) - type halt. to exit.",
        info.version, info.arch
    );
    println!("Ontologies loaded. Ready for queries.");
    io::stdout().flush().ok();

    let status = unsafe { swipl::fli::PL_toplevel() };
    drop(context);

    if status <= 0 {
        anyhow::bail!("Prolog toplevel exited with status {}", status);
    }

    Ok(())
}

fn configure_runtime(info: &SwiplInfo) {
    let boot_file = boot_file_path(&info.swi_home);

    set_env_var("SWI_HOME_DIR", &info.swi_home);
    set_env_var("PLBASE", &info.swi_home);
    set_env_var("SWI_LIB_DIR", &info.lib_dir);
    set_env_var("PLLIBDIR", &info.lib_dir);
    set_env_var("SWI_PACK_SO_DIR", &info.pack_so_dir);
    set_env_var("SWIPL_BOOT_FILE", &boot_file);
}

fn set_env_var(key: &str, value: &str) {
    unsafe { env::set_var(key, value) };
}

fn boot_file_path(swi_home: &str) -> String {
    let mut path = PathBuf::from(swi_home);
    path.push("boot.prc");
    path.to_string_lossy().into_owned()
}
