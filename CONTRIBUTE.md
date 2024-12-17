## Local Test
```bash
# at root dir
cargo run -- <command> [args...]


cargo build # build binary
```

## Release
```bash
# 0. assume you are in root dir
# 1. publish core package firstly and publish npm package respectively
cargo publish -p gito_core
cd packages/core && npm run publish:allProd

# 2. update gito_core version in cli/Cargo.toml, then publish cli package
cargo publish -p gito

```