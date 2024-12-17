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

# 2. update gito_core version in cli/Cargo.toml, then publish cli package
cargo publish -p gito

```

## core npm
we use napi-rs to build js version of core, if you need to update it, please follow the steps below
```bash
npm run build
npm version patch
npm run publish:allPre/allProd
```