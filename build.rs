use std::env;

fn main() {
    println!("cargo:rerun-if-changed=libtess2");
    if cfg!(feature = "update-bindings") {
        println!("cargo:rerun-if-changed=generated");
    }

    cc::Build::new()
        //.cpp(true)
        .opt_level(3)
        .include("libtess2/Include/")
        .files([
            "libtess2/Source/bucketalloc.c",
            "libtess2/Source/dict.c",
            "libtess2/Source/geom.c",
            "libtess2/Source/mesh.c",
            "libtess2/Source/priorityq.c",
            "libtess2/Source/sweep.c",
            "libtess2/Source/tess.c",
            ])
        .flag_if_supported("-std=c++14")
        .compile("libtess2");

    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();
    let target_env = env::var("CARGO_CFG_TARGET_ENV").unwrap();

    match (target_os.as_str(), target_env.as_str()) {
        ("linux", _) | ("windows", "gnu") |  ("android", _)  => println!("cargo:rustc-link-lib=dylib=stdc++"),
        ("macos", _) | ("ios", _) => println!("cargo:rustc-link-lib=dylib=c++"),
        ("windows", "msvc") => {}
        _ => unimplemented!(
            "target_os: {}, target_env: {}",
            target_os.as_str(),
            target_env.as_str()
        ),
    }

    #[cfg(feature = "generate-bindings")]
    {
        let bindings = bindgen::Builder::default()
            .header("libtess2/Include/tesselator.h")
            .allowlist_type("TessWindingRule")
            .allowlist_type("TessElementType")
            .allowlist_type("TessOption")
            .allowlist_type("TESSreal")
            .allowlist_type("TESSindex")
            .allowlist_type("TESStesselator")
            .allowlist_type("TESSalloc")
            .allowlist_function("tessNewTess")
            .allowlist_function("tessDeleteTess")
            .allowlist_function("tessAddContour")
            .allowlist_function("tessSetOption")
            .allowlist_function("tessTesselate")
            .allowlist_function("tessGetVertexCount")
            .allowlist_function("tessGetVertices")
            .allowlist_function("tessGetVertexIndices")
            .allowlist_function("tessGetElementCount")
            .allowlist_function("tessGetElements")
            .size_t_is_usize(true)
            .generate()
            .expect("unable to generate bindings");

        let out_path = if cfg!(feature = "update-bindings") {
            std::path::PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join("generated")
        } else {
            std::path::PathBuf::from(env::var("OUT_DIR").unwrap())
        };

        bindings
            .write_to_file(out_path.join("bindings.rs"))
            .expect("couldn't write bindings!");
    }
}