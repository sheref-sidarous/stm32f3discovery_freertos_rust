use std::env;
use std::fs::copy;
use std::path::PathBuf;

fn main() {

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
