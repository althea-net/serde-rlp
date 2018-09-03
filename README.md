# serde-rlp

Ethereum's RLP encoding implemented as a Serde data format

This code is part of `clarity` - an effort to implement lightweight Etehereum transaction signing. WIP.

# Examples

You should be able to use this crate same way as you'd use other serialization formats in serde. The key to serialization is `serde_rlp::ser::to_bytes` which does the magic.

```rust
extern crate serde_rlp;
use serde_rlp::ser::to_bytes;

// Basic key values
let data = vec![vec!["key1", "value1"], vec!["key2", "value2"]];
let bytes = to_bytes(&data).expect("Unable to serialize data");
println!("Serialized data: {:?}", bytes);
```