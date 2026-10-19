#![deny(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::unimplemented,
    clippy::todo,
    clippy::unreachable
)]

use std::collections::BTreeMap;
use std::env;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode};

const CRATES: &[&str] = &[
    "automation",
    "codegen",
    "diagram",
    "engine",
    "format",
    "geom",
    "history",
    "io",
    "layout",
    "model",
    "plugins",
    "render",
    "testkit",
    "ui",
    "validate",
];

const APPS: &[&str] = &["umlforge-cli", "umlforge-desktop", "umlforge-web"];

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("xtask: {error}");
            ExitCode::FAILURE
        }
    }
}

fn run() -> Result<(), String> {
    let mut arguments = env::args().skip(1);
    match arguments.next().as_deref() {
        Some("layers") => check_layers(),
        Some("wasm") => check_wasm(),
        Some("parity") => check_parity(),
        Some("test-report") => {
            println!("Test report collection is not implemented yet.");
            Ok(())
        }
        Some("help" | "--help" | "-h") | None => {
            println!(
                "UMLForge workspace tasks:\n  cargo xtask layers\n  cargo xtask wasm\n  cargo xtask parity\n  cargo xtask test-report"
            );
            Ok(())
        }
        Some(task) => Err(format!("unknown task '{task}'. Run 'cargo xtask --help'.")),
    }
}

fn workspace_root() -> Result<PathBuf, String> {
    let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    manifest
        .parent()
        .map(Path::to_path_buf)
        .ok_or_else(|| "could not locate the workspace root".to_owned())
}

fn check_layers() -> Result<(), String> {
    let root = workspace_root()?;
    let mut missing = Vec::new();

    for name in CRATES {
        if !root.join("crates").join(name).join("Cargo.toml").is_file() {
            missing.push(format!("crates/{name}/Cargo.toml"));
        }
    }

    for name in APPS {
        if !root.join("apps").join(name).join("Cargo.toml").is_file() {
            missing.push(format!("apps/{name}/Cargo.toml"));
        }
    }

    if !missing.is_empty() {
        return Err(format!(
            "workspace members are missing: {}",
            missing.join(", ")
        ));
    }

    println!(
        "Layer scaffold OK: {} crates and {} applications registered.",
        CRATES.len(),
        APPS.len()
    );
    println!("Dependency boundaries will be enforced as crate layers are implemented.");
    Ok(())
}

fn check_wasm() -> Result<(), String> {
    let root = workspace_root()?;
    println!("Checking WebAssembly-compatible workspace members...");
    let status = Command::new("cargo")
        .current_dir(root)
        .args([
            "check",
            "--workspace",
            "--exclude",
            "umlforge-desktop",
            "--exclude",
            "umlforge-ui",
            "--target",
            "wasm32-unknown-unknown",
        ])
        .status()
        .map_err(|error| format!("could not run cargo check: {error}"))?;

    if status.success() {
        Ok(())
    } else {
        Err(format!("WebAssembly check failed with {status}"))
    }
}

fn check_parity() -> Result<(), String> {
    let root = workspace_root()?;
    let checklist = std::fs::read_to_string(root.join("Parity-checklist.md"))
        .map_err(|error| format!("could not read Parity-checklist.md: {error}"))?;
    let plan = std::fs::read_to_string(root.join("Build-plan.MD"))
        .map_err(|error| format!("could not read Build-plan.MD: {error}"))?;

    let checklist_ids = checklist
        .lines()
        .filter_map(checklist_feature_id)
        .filter(|id| !id.starts_with("X-"))
        .collect::<Vec<_>>();
    let planned_ids = plan.lines().flat_map(plan_feature_ids).collect::<Vec<_>>();
    let mut checklist_counts = BTreeMap::new();
    let mut planned_counts = BTreeMap::new();

    for id in checklist_ids.iter().copied() {
        *checklist_counts.entry(id).or_insert(0_usize) += 1;
    }
    for id in planned_ids.iter().copied() {
        *planned_counts.entry(id).or_insert(0_usize) += 1;
    }

    let missing = checklist_counts
        .keys()
        .filter(|id| !planned_counts.contains_key(*id))
        .copied()
        .collect::<Vec<_>>();
    let untracked = planned_counts
        .keys()
        .filter(|id| !checklist_counts.contains_key(*id))
        .copied()
        .collect::<Vec<_>>();
    let duplicates = checklist_counts
        .iter()
        .filter(|(_, count)| **count != 1)
        .map(|(id, _)| *id)
        .chain(
            planned_counts
                .iter()
                .filter(|(_, count)| **count != 1)
                .map(|(id, _)| *id),
        )
        .collect::<Vec<_>>();
    let (done_rows_without_evidence, evidence_tests) = missing_evidence_tests(&checklist, &root)?;

    if !missing.is_empty() || !untracked.is_empty() || !duplicates.is_empty() {
        return Err(format!(
            "parity mismatch; missing from build plan: [{}]; absent from checklist: [{}]; duplicate rows: [{}]",
            missing.join(", "),
            untracked.join(", "),
            duplicates.join(", ")
        ));
    }

    if !done_rows_without_evidence.is_empty() {
        return Err(format!(
            "done checklist rows must name existing tests: {}",
            done_rows_without_evidence.join(", ")
        ));
    }

    println!(
        "Parity tracking OK: {} checklist rows appear exactly once in the build plan.",
        checklist_counts.len()
    );
    println!(
        "Verified {} test names in done checklist rows.",
        evidence_tests
    );
    println!("Checklist summary JSON will be generated in a later phase.");
    Ok(())
}

fn missing_evidence_tests(checklist: &str, root: &Path) -> Result<(Vec<String>, usize), String> {
    let mut missing_rows = Vec::new();
    let mut evidence_count = 0;
    for line in checklist.lines().filter(|line| line.starts_with('|')) {
        let Some(id) = checklist_feature_id(line) else {
            continue;
        };
        if !line.contains("✅") {
            continue;
        }
        let columns = line.split('|').map(str::trim).collect::<Vec<_>>();
        let Some(notes) = columns.iter().rev().nth(1).copied() else {
            continue;
        };
        let names = notes
            .split_whitespace()
            .filter(|word| word.starts_with("`test_"))
            .filter_map(|word| word.strip_prefix('`')?.strip_suffix('`'))
            .collect::<Vec<_>>();
        if names.is_empty() || names.iter().any(|name| !test_name_exists(root, name)) {
            missing_rows.push(id.to_owned());
            continue;
        }
        evidence_count += names.len();
    }
    Ok((missing_rows, evidence_count))
}

fn test_name_exists(root: &Path, test_name: &str) -> bool {
    let source_roots = [root.join("crates"), root.join("apps"), root.join("xtask")];
    source_roots
        .iter()
        .any(|path| source_file_contains_test_name(path, test_name))
}

fn source_file_contains_test_name(path: &Path, test_name: &str) -> bool {
    let Ok(entries) = std::fs::read_dir(path) else {
        return false;
    };
    entries.flatten().any(|entry| {
        let child = entry.path();
        if child.is_dir() {
            source_file_contains_test_name(&child, test_name)
        } else if child.extension().is_some_and(|extension| extension == "rs") {
            std::fs::read_to_string(child).is_ok_and(|contents| contents.contains(test_name))
        } else {
            false
        }
    })
}

fn checklist_feature_id(line: &str) -> Option<&str> {
    let id = line.split('|').nth(1)?.trim();
    (is_feature_id(id) && !id.starts_with("X-")).then_some(id)
}

fn plan_feature_ids(line: &str) -> Vec<&str> {
    let Some((_, rows)) = line.split_once("**Rows:**") else {
        return Vec::new();
    };
    rows.split(',')
        .map(str::trim)
        .filter(|id| is_feature_id(id) && !id.starts_with("X-"))
        .collect()
}

fn is_feature_id(id: &str) -> bool {
    let Some((prefix, number)) = id.split_once('-') else {
        return false;
    };
    !prefix.is_empty()
        && prefix
            .chars()
            .all(|character| character.is_ascii_uppercase())
        && number.len() == 3
        && number.chars().all(|character| character.is_ascii_digit())
}

#[cfg(test)]
mod tests {
    use super::{
        check_layers, check_parity, checklist_feature_id, is_feature_id, missing_evidence_tests,
        plan_feature_ids, test_name_exists,
    };

    #[test]
    fn feature_id_accepts_known_phase_feature() {
        assert!(is_feature_id("CLS-004"));
    }

    #[test]
    fn feature_id_format_accepts_out_of_scope_decision_id() {
        assert!(is_feature_id("X-001"));
    }

    #[test]
    fn feature_id_rejects_malformed_row() {
        assert!(!is_feature_id("CLS-4"));
    }

    #[test]
    fn plan_row_extracts_only_feature_ids() {
        let ids = plan_feature_ids("**Rows:** APP-001, CLS-004, X-001");

        assert_eq!(ids, ["APP-001", "CLS-004"]);
    }

    #[test]
    fn checklist_row_ignores_non_feature_tables() {
        let feature = checklist_feature_id("| CLS-004 | Association | P0 | ⬜ | | |");
        let decision = checklist_feature_id("| X-001 | Licence keys | Out | ⛔ | | |");

        assert_eq!(feature, Some("CLS-004"));
        assert_eq!(decision, None);
    }

    #[test]
    fn parity_check_flags_done_rows_without_test_evidence() -> Result<(), String> {
        let checklist = "| CLS-004 | Association | P0 | ✅ | | no command or test listed |";
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap_or_else(|| std::path::Path::new("."));
        let (missing, count) = missing_evidence_tests(checklist, root)?;

        assert_eq!(missing, ["CLS-004"]);
        assert_eq!(count, 0);
        Ok(())
    }

    #[test]
    fn test_evidence_search_finds_existing_test_name() {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap_or_else(|| std::path::Path::new("."));

        assert!(test_name_exists(
            root,
            "feature_id_accepts_known_phase_feature"
        ));
    }

    #[test]
    fn layer_check_accepts_registered_workspace() {
        assert_eq!(check_layers(), Ok(()));
    }

    #[test]
    fn parity_check_accepts_current_checklist_and_build_plan() {
        assert_eq!(check_parity(), Ok(()));
    }

    #[test]
    fn parity_check_counts_existing_test_evidence() -> Result<(), String> {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap_or_else(|| std::path::Path::new("."));
        let checklist = "| APP-001 | New project | P0 | ✅ | | `test_evidence_search_finds_existing_test_name` |";
        let (missing, count) = missing_evidence_tests(checklist, root)?;

        assert!(missing.is_empty());
        assert_eq!(count, 1);
        Ok(())
    }

    #[test]
    fn test_evidence_search_rejects_missing_directory() {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap_or_else(|| std::path::Path::new("."));

        assert!(!test_name_exists(
            &root.join("missing-directory"),
            "anything"
        ));
    }
}
