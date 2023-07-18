use std::process::Command;

///
/// Returns a description of the current git repo state.
/// Preferred the current git tag.
///
fn get_git_describe() -> String {
    // check if there is an override via env (for docker environments)
    if let Ok(ver) = std::env::var("BUILD_VERSION_TAG") {
        return ver;
    }

    let out = Command::new("git")
        .args(["describe", "--tags", "--always", "--dirty"])
        .output()
        .expect("'git describe' failed! Cannot determine version.");
    String::from_utf8(out.stdout).unwrap()
}

fn main() {
    // this ensures, that we can access the git tag, that was current at compile time,
    // via `env!("GIT_VERSION_TAG")`
    println!("cargo:rustc-env=GIT_VERSION_TAG={}", get_git_describe());

    // ensure cargo re-compiles, if the migrations changed
    //      cf. https://docs.rs/sqlx/latest/sqlx/macro.migrate.html#triggering-recompilation-on-migration-changes
    println!("cargo:rerun-if-changed=migrations");
}
