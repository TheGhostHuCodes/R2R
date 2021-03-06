fn main() {
    let nginx_dir = "nginx-1.19.3";

    let bindings = bindgen::builder()
        .header("wrapper.h")
        .whitelist_type("ngx_.*")
        .whitelist_function("ngx_.*")
        .whitelist_var("ngx_.*")
        .clang_args(vec![
            format!("-I{}/src/core", nginx_dir),
            format!("-I{}/src/event", nginx_dir),
            format!("-I{}/src/event/modules", nginx_dir),
            format!("-I{}/src/os/unix", nginx_dir),
            format!("-I{}/objs", nginx_dir),
            format!("-I{}/src/http", nginx_dir),
            format!("-I{}/src/http/v2", nginx_dir),
            format!("-I{}/src/http/modules", nginx_dir),
        ])
        .generate()
        .unwrap();

    let out_dir = std::env::var("OUT_DIR").unwrap();
    bindings
        .write_to_file(format!("{}/nginx.rs", out_dir))
        .expect("unable to write bindings");
}
