extern crate core;

use cmake::Config;
use std::process::Command;
use std::{
    env, fs,
    path::{Path, PathBuf},
};

fn main() {
    let vendored = env::var("CARGO_FEATURE_VENDORED").is_ok();
    let lib_dir_isset = env::var("KHAIII_LIB_DIR").is_ok();

    let use_local_khaiii = !vendored;
    if use_local_khaiii {
        if lib_dir_isset {
            let lib_dir = env::var("KHAIII_LIB_DIR").unwrap();

            println!("cargo:rustc-link-search=native={}", lib_dir);
        } else {
            println!("cargo:rustc-link-search=native=/usr/local/lib/");
        }

        println!("cargo:rustc-link-lib=dylib=khaiii");
        println!("cargo:warning=Using unknown Khaiii version.");

        return;
    } else {
        println!("cargo:rustc-cfg=khaiii_vendored");

        build_khaiii();
    }
}

fn build_khaiii() {
    if !Path::new("khaiii/src").exists() {
        let _ = Command::new("git")
            .args(&["submodule", "update", "--init", "khaiii"])
            .status();
    }

    let dst = PathBuf::from(env::var("OUT_DIR").unwrap());
    let lib_output_dir = dst.join("build/share/khaiii");
    fs::create_dir_all(lib_output_dir).unwrap();

    let dst = Config::new("khaiii").cxxflag("-w").build();

    println!("cargo:rustc-link-search=native={}", dst.display());
    println!("cargo:rustc-link-lib=dylib=khaiii");
}
