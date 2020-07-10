// Due to this bug: https://github.com/rust-lang/cargo/issues/4866, we cannot
// specify different features based on different targets now in cargo file. We
// have to keep features always on, and do conditional compilation within the
// source code

fn main() {
    use std::env;

    let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap();
    let target_pointer_width = env::var("CARGO_CFG_TARGET_POINTER_WIDTH").unwrap();
    let target_family = env::var("CARGO_CFG_TARGET_FAMILY").unwrap_or_default();
    let is_windows = target_family == "windows";
    let is_unix = target_family == "unix";
    let can_enable_asm = (target_pointer_width == "64") && (is_windows || is_unix);

    if cfg!(feature = "asm") && (!can_enable_asm) {
        panic!("asm feature can only be enabled on 64-bit Linux, macOS and Windows platforms!");
    }

    if cfg!(any(feature = "asm", feature = "detect-asm")) && can_enable_asm {
        use cc::Build;
        use std::path::Path;
        use std::process::Command;

        fn run_command(mut c: Command) {
            let status = c.status().unwrap_or_else(|e| {
                panic!("Error running command: {:?} error: {:?}", c, e);
            });
            if !status.success() {
                panic!(
                    "Command {:? }exits with non-success status: {:?}",
                    c, status
                );
            }
        }

        let mut build = Build::new();

        if is_windows {
            let out_dir = env::var("OUT_DIR").unwrap();
            let expand_path = Path::new(&out_dir).join("execute-expanded.S");
            let mut expand_command = Command::new("clang");
            expand_command
                .arg("-E")
                .arg("src/machine/asm/execute.S")
                .arg("-o")
                .arg(&expand_path);
            run_command(expand_command);

            let compile_path = Path::new(&out_dir).join("execute.o");
            let mut compile_command = Command::new("yasm");
            compile_command
                .arg("-p")
                .arg("gas")
                .arg("-f")
                .arg("x64")
                .arg("-m")
                .arg("amd64")
                .arg(&expand_path)
                .arg("-o")
                .arg(&compile_path);
            run_command(compile_command);

            build
                .object(&compile_path)
                .file("src/machine/aot/aot.x64.win.compiled.c");
        } else {
            match &target_arch[..] {
                "x86_64" => build.file("src/machine/asm/execute.S"),
                "aarch64" => build.file("src/machine/asm/execute_aarch64.S"),
                _ => panic!("asm feature is only available for x86_64 and aarch64 architecture!"),
            };
            build.file("src/machine/aot/aot.x64.compiled.c");
        }

        build
            .include("dynasm")
            .include("src/machine/asm")
            .compile("asm");

        println!("cargo:rustc-cfg=has_asm")
    }
}
