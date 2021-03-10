fn main() {
    let mut builder = cc::Build::new();
    let build = builder
        .files([
            "src/vendor/evdi/library/evdi_lib.c",
            "src/c_wrapper/wrapper.c"
        ].iter())
        .include("src/vendor/evdi/module");

    build.compile("foo");
}
