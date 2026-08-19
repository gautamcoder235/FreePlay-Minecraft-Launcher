use std::path::{Path, PathBuf};
use std::process::Command;
use freeplay_application::JavaRuntimeInfo;

pub fn detect_java_installations() -> Vec<JavaRuntimeInfo> {
    let mut detected = Vec::new();
    let mut candidates: Vec<PathBuf> = Vec::new();

    // 1. JAVA_HOME environment variable
    if let Ok(java_home) = std::env::var("JAVA_HOME") {
        let p = Path::new(&java_home).join("bin").join(if cfg!(windows) { "java.exe" } else { "java" });
        if p.exists() {
            candidates.push(p);
        }
    }

    // 2. PATH environment
    if let Ok(path_var) = std::env::var("PATH") {
        for dir in std::env::split_paths(&path_var) {
            let p = dir.join(if cfg!(windows) { "java.exe" } else { "java" });
            if p.exists() {
                candidates.push(p);
            }
        }
    }

    // 3. Windows standard paths
    #[cfg(windows)]
    {
        let roots = [
            r"C:\Program Files\Java",
            r"C:\Program Files\Eclipse Adoptium",
            r"C:\Program Files\Microsoft",
            r"C:\Program Files\BellSoft",
            r"C:\Program Files\Amazon Corretto",
            r"C:\Program Files\Zulu",
        ];

        for root in roots {
            let root_path = Path::new(root);
            if root_path.exists() {
                if let Ok(entries) = std::fs::read_dir(root_path) {
                    for entry in entries.flatten() {
                        let java_exe = entry.path().join("bin").join("java.exe");
                        if java_exe.exists() {
                            candidates.push(java_exe);
                        }
                    }
                }
            }
        }
    }

    // Deduplicate candidates
    candidates.sort();
    candidates.dedup();

    for candidate in candidates {
        if let Some(info) = probe_java_binary(&candidate) {
            detected.push(info);
        }
    }

    detected
}

fn probe_java_binary(path: &Path) -> Option<JavaRuntimeInfo> {
    let output = Command::new(path).arg("-version").output().ok()?;
    let stderr_str = String::from_utf8_lossy(&output.stderr);
    let stdout_str = String::from_utf8_lossy(&output.stdout);
    let full_version_text = format!("{}\n{}", stderr_str, stdout_str);

    let major_version = parse_java_major_version(&full_version_text)?;
    let is_64bit = full_version_text.contains("64-Bit") || full_version_text.contains("x86_64") || full_version_text.contains("amd64");

    Some(JavaRuntimeInfo {
        path: path.to_string_lossy().to_string(),
        major_version,
        is_64bit,
    })
}

fn parse_java_major_version(output: &str) -> Option<u32> {
    // Look for patterns like "version \"1.8.0_351\"" or "version \"17.0.2\"" or "version \"21.0.1\""
    for line in output.lines() {
        if line.contains("version \"") {
            if let Some(start) = line.find("version \"") {
                let rest = &line[start + 9..];
                if let Some(end) = rest.find('"') {
                    let version_str = &rest[..end];
                    if let Some(major) = version_str.split('.').next() {
                        if major == "1" {
                            // "1.8" -> 8
                            if let Some(minor) = version_str.split('.').nth(1) {
                                return minor.parse::<u32>().ok();
                            }
                        } else {
                            return major.parse::<u32>().ok();
                        }
                    }
                }
            }
        }
    }
    None
}
