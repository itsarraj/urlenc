# urlenc

A tiny, fast CLI to safely URL-encode or decode strings or stdin streams natively, without relying on python or awkward shell scripts.

## Status

**built, untested**: the URL encoding/decoding logic is built.

## Installation

```sh
cargo install --path .
```

## Usage

```sh
urlenc "hello world"
urlenc --mode decode "hello%20world"
echo "data" | urlenc
```
