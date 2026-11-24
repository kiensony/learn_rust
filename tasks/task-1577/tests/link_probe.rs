use std::{env, fs, path::PathBuf, process::Command};
struct Scratch(PathBuf);
impl Drop for Scratch {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

#[test]
fn native_client_links() {
    let dir = env::temp_dir().join(format!(
        "track-link-{}-{}",
        env!("CARGO_PKG_NAME"),
        std::process::id()
    ));
    fs::create_dir(&dir).unwrap();
    let dir = Scratch(dir);
    let rustc = env::var_os("RUSTC").unwrap_or_else(|| "rustc".into());
    let build = Command::new(&rustc)
        .args([
            "--edition",
            "2024",
            "--crate-type",
            "staticlib",
            "--crate-name",
            "track_metrics_v1",
            "fixtures/support.rs",
            "--out-dir",
        ])
        .arg(&dir.0)
        .output()
        .unwrap();
    assert!(
        build.status.success(),
        "{}",
        String::from_utf8_lossy(&build.stderr)
    );
    let archive = task_1577::native_archive("metrics-v1");
    let out = Command::new(rustc)
        .args(["--edition", "2024", "fixtures/client.rs", "-L"])
        .arg(&dir.0)
        .arg(format!("-lstatic={archive}"))
        .arg("-o")
        .arg(dir.0.join(format!("probe{}", env::consts::EXE_SUFFIX)))
        .output()
        .unwrap();
    assert!(
        out.status.success(),
        "native link failed for {archive}: {}",
        String::from_utf8_lossy(&out.stderr)
    );
}
