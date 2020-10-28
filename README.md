# Description

Dummy, quick and dirty json to yaml converter
It can do also yaml to json conversion

To compile:

```
rustup target add x86_64-unknown-linux-musl
RUSTFLAGS="-C target-feature=+crt-static"  cargo build --target x86_64-unknown-linux-musl --release && strip target/x86_64-unknown-linux-musl/release/json2yaml
```

The binary json2yaml is in target/x86_64-unknown-linux-musl/release

# Usage

To display the result to stdout:

```
./json2yaml -s myfile.json
```

To write the result into a file:

```
./json2yaml -s myfile.json -d myfile.yaml
```

To perform yaml to json:

    ./json2yaml -i -s myfile.yaml
