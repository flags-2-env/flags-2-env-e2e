use std::path::PathBuf;

use flags2env::BundledFlags2Env;

fn sidecar_contract() -> PathBuf {
    let checkout = std::env::var_os("FLAGS2ENV_SIDECAR_CHECKOUT")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("upstream/sidecar"));
    checkout.join(".cli-flags.toml")
}

fn argv(items: &[&str]) -> Vec<String> {
    items.iter().map(|item| (*item).to_owned()).collect()
}

#[test]
fn landed_core_audits_and_parses_landed_sidecar_contract() {
    let contract = sidecar_contract();
    assert!(
        contract.is_file(),
        "missing sidecar checkout contract: {contract:?}"
    );
    let path = contract.to_str().expect("UTF-8 sidecar contract path");
    let parser = BundledFlags2Env::new();

    parser
        .audit_config(Some(path))
        .expect("sidecar contract audit");
    let parsed = parser
        .parse_structured(
            &argv(&[
                "flags2env-platform-sidecar",
                "preflight",
                "--bind=127.0.0.1:19090",
            ]),
            Some(path),
        )
        .expect("structured parse");

    assert!(
        parsed.errors.is_empty(),
        "parser errors: {}",
        parsed.errors.len()
    );
    assert!(parsed.unknown_options.is_empty());
    assert!(parsed.extras.is_empty());
    assert_eq!(parsed.command, "preflight");
    assert_eq!(
        parsed
            .provided_flags
            .get("FLAGS_2_ENV_SIDECAR_BIND")
            .map(String::as_str),
        Some("127.0.0.1:19090")
    );
    assert!(
        parsed.dotenv.is_empty(),
        "sidecar contract must not load ambient dotenv files"
    );
}

#[test]
fn landed_core_normalizes_health_probe_alias() {
    let contract = sidecar_contract();
    let path = contract.to_str().expect("UTF-8 sidecar contract path");
    let parsed = BundledFlags2Env::new()
        .parse_structured(
            &argv(&["flags2env-platform-sidecar", "probe-healthz"]),
            Some(path),
        )
        .expect("structured parse");

    assert!(parsed.errors.is_empty());
    assert!(parsed.unknown_options.is_empty());
    assert!(parsed.extras.is_empty());
    assert_eq!(parsed.command, "probe");
}

#[test]
fn landed_sidecar_contract_rejects_unknown_argv() {
    let contract = sidecar_contract();
    let path = contract.to_str().expect("UTF-8 sidecar contract path");
    let parsed = BundledFlags2Env::new()
        .parse_structured(
            &argv(&["flags2env-platform-sidecar", "--definitely-not-declared"]),
            Some(path),
        )
        .expect("structured parse");

    assert!(parsed.errors.is_empty());
    assert_eq!(parsed.unknown_options.len(), 1);
    assert!(parsed.extras.is_empty());
}
