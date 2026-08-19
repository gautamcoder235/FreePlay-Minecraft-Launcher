use crate::manifest::Rule;

pub fn is_rule_allowed(rules: Option<&[Rule]>, target_os: &str, target_arch: &str) -> bool {
    let rules = match rules {
        Some(r) => r,
        None => return true,
    };

    let mut allowed = false;

    for rule in rules {
        let mut matches = true;

        if let Some(ref os) = rule.os {
            if let Some(ref name) = os.name {
                let os_match = match name.as_str() {
                    "windows" => target_os == "windows",
                    "osx" | "macos" => target_os == "macos",
                    "linux" => target_os == "linux",
                    _ => false,
                };
                if !os_match {
                    matches = false;
                }
            }

            if let Some(ref arch) = os.arch {
                if arch != target_arch {
                    matches = false;
                }
            }
        }

        if matches {
            allowed = rule.action == "allow";
        }
    }

    allowed
}
