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
### publish by CI
This way publish pakcages by git commit message with github CI, for example:
- git message is `1.0.0`, then the CI will publish the 1.0.0 with the `latest` tag
- if the git message is `1.0.0-beta.0`, then this version will be publish with tag `beta`

### maunal steps
we use napi-rs to build js version of core, if you need to update it, please follow the steps below
```bash
npm run build
npm version patch
npm run publish:allPre/allProd
```