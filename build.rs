fn main() {
    println!("cargo:rerun-if-changed=libc/foo.c");
    cc::Build::new().file("libc/foo.c").compile("foo");
}
