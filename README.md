# serde-rlp

[![Crates.io](https://img.shields.io/crates/v/serde-rlp.svg)](https://crates.io/crates/serde-rlp) [![Build Status](https://travis-ci.org/althea-mesh/serde-rlp.svg?branch=master)](https://travis-ci.org/althea-mesh/serde-rlp) [![Docs.rs](https://docs.rs/serde-rlp/badge.svg)](https://docs.rs/serde-rlp)

Ethereum's RLP encoding implemented as a Serde data format

This code is part of `clarity` - an effort to implement lightweight Ethereum transaction signing. WIP.

# Releasing

To release new version of `serde-rlp` do the following steps:

```sh
# Do a signed tag for new version with annotation "YYYY-MM-DD, Version v$VERSION"
git tag v1.0.0 -a -s "2018-09-04, Version v1.0.0"
git push origin --follow-tags
```

# Examples

You should be able to use this crate same way as you'd use other serialization formats in serde.

## Serialize

The key to serialization is `serde_rlp::ser::to_bytes` which does the magic.

```rust
extern crate serde_rlp;
use serde_rlp::ser::to_bytes;

// Basic key values
let data = vec![vec!["key1", "value1"], vec!["key2", "value2"]];
let bytes = to_bytes(&data).expect("Unable to serialize data");
println!("Serialized data: {:?}", bytes);
```

## Deserialize

To deserialize data back into an object you should use `serde_rlp::de::from_bytes`.

```rust
extern crate serde_rlp;
use serde_rlp::de::from_bytes;

// Deserialize string "abc" encoded as RLP
let foo: String = from_bytes(&[0x83, 0x61, 0x62, 0x63]).unwrap();
println!("{}", foo);
```

An useful pattern is to deserialize into `Vec<Bytes>`, which will correctly deserialize elements of a RLP list.