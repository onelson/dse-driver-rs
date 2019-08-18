#[cfg(feature = "pkg-config")]
extern crate pkg_config;

use std::env;

#[cfg(feature = "pkg-config")]
pub fn configured_by_pkg_config(statik: bool) -> bool {
    pkg_config::probe_library(format!("dse{}", if statik { "_static" } else { "" }).as_str())
        .is_ok()
}

#[cfg(not(feature = "pkg-config"))]
fn configured_by_pkg_config(statik: bool) -> bool {
    false
}

fn configure_by_env(dse_static: bool) {
    // FIXME: get a DIR env var for libuv too?
    if let Ok(path) = env::var("DSE_LIB_DIR") {
        println!("cargo:rustc-link-search=native={}", path);
    }

    let (libname, link_type) = if dse_static {
        ("dse_static", "static=")
    } else {
        ("dse", "")
    };
    println!("cargo:rustc-link-lib={}{}", link_type, libname);
    println!("cargo:rustc-link-lib={}{}", link_type, "uv");
}

fn main() {
    let dse_static = env::var("DSE_LIB_STATIC")
        .ok()
        .map(|s| !s.trim().is_empty())
        .unwrap_or(false);

    if !configured_by_pkg_config(dse_static) {
        configure_by_env(dse_static)
    }
}
