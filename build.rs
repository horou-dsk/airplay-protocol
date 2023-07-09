use std::path::Path;

macro_rules! p {
    ($($tokens: tt)*) => {
        println!("cargo:warning={}", format!($($tokens)*))
    }
}

fn main() {
    println!("cargo:rerun-if-changed=libc/foo.c");
    // 获取当前编译目标
    let target = std::env::var("TARGET").unwrap();
    let mut builder = cc::Build::new();
    if target == "aarch64-linux-android" {
        let ndk = std::env::var("ANDROID_NDK")
            .unwrap_or_else(|_| std::env::var("NDK").expect("ANDROID_NDK or NDK not set"));
        let target_os = std::env::var("OS").unwrap();
        let ndk_path = Path::new(&ndk);
        p!("target_os: {}", target_os);
        p!("Using Android NDK: {:?}", ndk_path);
        let clang_path = if target_os.to_lowercase().contains("windows") {
            "toolchains/llvm/prebuilt/windows-x86_64/bin/clang.exe"
        } else {
            "toolchains/llvm/prebuilt/linux-x86_64/bin/clang"
        };
        builder.compiler(ndk_path.join(clang_path));
    }
    builder.file("libc/foo.c").compile("foo");
}
