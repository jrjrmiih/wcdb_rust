# Reconstructed Dockerfile for harbor.rongcloud.net/library/rust/wcdb:0.1.4
#
# Derived from .gitlab-ci.yml tool usage. Not the original — inferred from
# every command the pipeline runs (cargo/fmt/test, autocorrect, commitlint,
# clang-format, colordiff, git submodules, and building the openssl/sqlcipher/
# zstd submodules from source which need a full C/C++ toolchain).
#
# Build:
#   docker build -f Dockerfile.ci -t harbor.rongcloud.net/library/rust/wcdb:0.1.4 .

# Official Rust image (Debian bookworm). Pin to a concrete Rust version.
FROM rust:1.82-bookworm

ARG NODE_MAJOR=20

# --- System / C-C++ build deps for openssl, sqlcipher, zstd from source ---
RUN apt-get update && apt-get install -y --no-install-recommends \
        build-essential \
        gcc g++ make cmake \
        perl \
        tcl \
        pkg-config \
        clang-format \
        colordiff \
        git \
        curl ca-certificates gnupg \
    && rm -rf /var/lib/apt/lists/*

# --- Node.js + npm (commitlint via npx) ---
RUN mkdir -p /etc/apt/keyrings \
    && curl -fsSL https://deb.nodesource.com/gpgkey/nodesource-repo.gpg.key \
        | gpg --dearmor -o /etc/apt/keyrings/nodesource.gpg \
    && echo "deb [signed-by=/etc/apt/keyrings/nodesource.gpg] https://deb.nodesource.com/node_${NODE_MAJOR}.x nodistro main" \
        > /etc/apt/sources.list.d/nodesource.list \
    && apt-get update && apt-get install -y --no-install-recommends nodejs \
    && rm -rf /var/lib/apt/lists/*

# --- Rust components used by CI ---
RUN rustup component add rustfmt clippy

# --- autocorrect (text linter, `autocorrect --lint`) ---
# The crates.io `autocorrect` crate is library-only (no binary), so install
# the prebuilt CLI binary from GitHub releases into /usr/local/bin.
RUN curl -fsSL https://raw.githubusercontent.com/huacnlee/autocorrect/main/install | sh

# cc / c++ symlinks exist via build-essential (cc->gcc, c++->g++),
# matching CI's `export CC=$(which cc)` / `export CXX=$(which c++)`.

WORKDIR /work
CMD ["bash"]
