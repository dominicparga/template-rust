use std::{env, fs, path::PathBuf};

mod defaults {
    pub const FOO: &str = "42";
}

fn main() {
    let out_dir = {
        let out_dir = env::var_os("OUT_DIR").expect("Env-var OUT_DIR is not set.");
        PathBuf::from(&out_dir)
    };

    // read in graph-dim from environment
    let foo: usize = {
        let foo = env::var_os("FOO").unwrap_or(defaults::FOO.into());
        let foo = foo.to_string_lossy();
        foo.parse().expect(&format!(
            "The provided env-var FOO should be usize, but isn't (FOO={}).",
            foo,
        ))
    };

    // https://stackoverflow.com/a/37528134
    //
    // write compiler-constants into file
    fs::write(
        out_dir.join("compiler.rs"),
        format!(
            "pub(crate) const FOO: usize = {};
            ",
            foo
        ),
    )
    .expect("Writing compiler.rs didn't work.");

    // reruns
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-env-changed=GRAPH_DIM");
}
