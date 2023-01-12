use std::env;
use std::fs::copy;
use std::path::PathBuf;

fn main() {
    // Allows to show relevant environment variables for debugging purpose
    //print_env();

    let target = env::var("TARGET").unwrap_or_default();
    let out_dir = env::var("OUT_DIR").unwrap();

    let mut b = freertos_cargo_build::Builder::new();

    b.freertos("FreeRTOS-Kernel/");

    if target == "thumbv7em-none-eabihf" {
        b.freertos_config("src");
        copy(
            "src/memory.x",
            PathBuf::from(out_dir.as_str()).join("memory.x"),
        )
        .unwrap();
    } else {
        panic!("unexpected traget");
    }

    b.compile().unwrap_or_else(|e| { panic!("{}", e.to_string()) });
}

/// Print relevant environment variables
fn print_env() {
    let env_keys = ["TARGET", "OUT_DIR", "HOST"];
    env::vars().for_each(|(key, val)| {
        if key.starts_with("CARGO") {
            println!("cargo:warning={}={}", key, val);
        } else if env_keys.contains(&key.as_str()) {
            println!("cargo:warning={}={}", key, val);
        } else {
            // println!("cargo:warning={}={}", key, val);
        }
    });
}
