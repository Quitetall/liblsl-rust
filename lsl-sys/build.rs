use std::env;

fn main() {
    // TODO: find out if liblsl already present on system and usable (if so, link to that instead)
    // println!("cargo:warning={}", "rebuilding...");
    build_liblsl();
}

// Build the liblsl library from source using cmake
fn build_liblsl() {
    let target = env::var("TARGET").unwrap();
    
    // build with cmake
    let mut cfg = cmake::Config::new("liblsl");
    cfg
        .define("LSL_NO_FANCY_LIBNAME", "ON")
        .define("LSL_BUILD_STATIC", "ON");
    if target.contains("msvc") {
        // override some C/CXX flags that the cmake crate splices in on Windows
        // (these cause the build to fail)...
        // * /EHsc sets the correct exception handling mode
        // * /GR enables RTTI
        // * /MD links in the msvcrt as a DLL instead of statically
        let cxx_args = " /nologo /EHsc /MD /GR";
        cfg 
            .define("WIN32", "1")
            .define("_WINDOWS", "1")
            .define("CMAKE_C_FLAGS", cxx_args)
            .define("CMAKE_CXX_FLAGS", cxx_args)
            .define("CMAKE_C_FLAGS_DEBUG", cxx_args)
            .define("CMAKE_CXX_FLAGS_DEBUG", cxx_args)
            .define("CMAKE_C_FLAGS_RELEASE", cxx_args)
            .define("CMAKE_CXX_FLAGS_RELEASE", cxx_args);
    }
    let install_dir = cfg.build();

    // emit link directives
    let libdir = install_dir.join("lib");
    let libname = "lsl";
    println!(
        "cargo:rustc-link-search=native={}",
        libdir.to_str().unwrap()
    );
    println!("cargo:rustc-link-lib=static={}", libname);

    // make sure we also link some additional libs
    if target.contains("linux") {
        println!("cargo:rustc-link-lib=dylib=stdc++");
    } else if target.contains("windows") {
        // Static CMake link-interface dependencies are not propagated to rustc.
        // MSVC embeds its C++ runtime DEFAULTLIB records; GNU needs stdc++ named.
        if target.contains("gnu") {
            println!("cargo:rustc-link-lib=dylib=stdc++");
        }
        // Mirrors liblsl's Windows CMake target; bcrypt remains an lsl-sys need.
        println!("cargo:rustc-link-lib=dylib=bcrypt");
        println!("cargo:rustc-link-lib=dylib=iphlpapi");
        println!("cargo:rustc-link-lib=dylib=winmm");
        println!("cargo:rustc-link-lib=dylib=mswsock");
        println!("cargo:rustc-link-lib=dylib=ws2_32");
    } else {
        println!("cargo:rustc-link-lib=dylib=c++");
    }
}
