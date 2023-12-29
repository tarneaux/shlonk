#!/bin/sh -e
cargo update
cargo publish
version=$(grep version Cargo.toml | cut -d "\"" -f 2 | head -n 1)
major=$(echo "${version}" | cut -d "." -f 1)
minor=$(echo "${version}" | cut -d "." -f 2)
patch=$(echo "${version}" | cut -d "." -f 3)
podman build -t "ghcr.io/tarneaux/shlonk:latest" .
podman tag "ghcr.io/tarneaux/shlonk:latest" "ghcr.io/tarneaux/shlonk:${major}"
podman tag "ghcr.io/tarneaux/shlonk:latest" "ghcr.io/tarneaux/shlonk:${major}.${minor}"
podman tag "ghcr.io/tarneaux/shlonk:latest" "ghcr.io/tarneaux/shlonk:${major}.${minor}.${patch}"
podman push "ghcr.io/tarneaux/shlonk:latest"
podman push "ghcr.io/tarneaux/shlonk:${major}"
podman push "ghcr.io/tarneaux/shlonk:${major}.${minor}"
podman push "ghcr.io/tarneaux/shlonk:${major}.${minor}.${patch}"

