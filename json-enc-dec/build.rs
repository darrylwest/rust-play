

fn main() {

    println!("cargo:rerun-iuf-changed=clib/hello.c");

    cc::Build::new()
        .file("clib/hello.c")
        .compile("hello");

}
