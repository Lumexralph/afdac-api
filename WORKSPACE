#######################
# Rust Toolchain
#######################
load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")

http_archive(
    name = "rules_rust",
    sha256 = "75177226380b771be36d7efc538da842c433f14cd6c36d7660976efb53defe86",
    urls = ["https://github.com/bazelbuild/rules_rust/releases/download/0.34.1/rules_rust-v0.34.1.tar.gz"],
)

load("@rules_rust//rust:repositories.bzl", "rules_rust_dependencies", "rust_register_toolchains")

rules_rust_dependencies()

rust_register_toolchains(
    edition = "2021",
    versions = ["1.71.0"],
)

load("@rules_rust//crate_universe:repositories.bzl", "crate_universe_dependencies")

crate_universe_dependencies()

load("@rules_rust//crate_universe:defs.bzl", "crates_repository", "crate")

crates_repository(
    name = "crate_index",
    annotations = {
        "headless_chrome": [crate.annotation(
            crate_features = [
                "fetch",
                "auto_generate_cdp/offline",
            ],
        )],
    },
    cargo_lockfile = "//:Cargo.lock",
    lockfile = "//:cargo.bazel.lock",
    manifests = [
        "//:Cargo.toml",
        "//:scraper_rs/Cargo.toml",
    ],
)

load("@crate_index//:defs.bzl", "crate_repositories")

crate_repositories()

# Load gazelle_rust transitive dependencies (includes gazelle). You can also load gazelle yourself,
# before these macros.

