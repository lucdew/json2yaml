# Description

Dummy, quick and dirty json2yaml executable


To compile:
```
cargo build --release && strip target/release/json2yaml
```
The binary json2yaml is in target/release

# Usage

To display the result to stdout:
```
./json2yaml -s myfile.json
```

To write the result into a file:

```
./json2yaml -s myfile.json -d myfile.yaml
```
