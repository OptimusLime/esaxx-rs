#[cfg(feature = "cpp")]
fn main() {
    let target = std::env::var("TARGET").unwrap_or_default();

    // For Windows GNU targets
    if target.contains("windows-gnu") {
        cc::Build::new()
            .cpp(true)
            .flag("-std=c++11")
            // No -stdlib=libc++ for Windows GNU
            .static_crt(true)
            .file("src/esaxx.cpp")
            .include("src")
            .compile("esaxx");

        return;
    }

    // For Windows MSVC targets
    if target.contains("windows-msvc") {
        let mut build = cc::Build::new();
        build
            .cpp(true)
            .static_crt(true)
            .file("src/esaxx.cpp")
            .include("src");

        // Enable exceptions for MSVC builds
        // MSVC-style flags
        build.flag("/EHsc");

        // Also try GCC-style flags as fallback for clang-cl
        build.flag_if_supported("-fexceptions");
        build.compile("esaxx");
        return;
    }

    // For macOS targets
    if target.contains("apple") {
        cc::Build::new()
            .cpp(true)
            .flag("-std=c++11")
            .flag("-stdlib=libc++")
            .static_crt(true)
            .file("src/esaxx.cpp")
            .include("src")
            .compile("esaxx");

        return;
    }

    // For other targets (Linux, etc.)
    cc::Build::new()
        .cpp(true)
        .flag("-std=c++11")
        // No -stdlib=libc++ for other platforms
        .static_crt(true)
        .file("src/esaxx.cpp")
        .include("src")
        .compile("esaxx");
}

#[cfg(not(feature = "cpp"))]
fn main() {}
