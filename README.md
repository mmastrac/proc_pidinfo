# proc_pidinfo

A Rust library for accessing the `proc_pidinfo`, `proc_pidfdinfo`, and `proc_pidfileportinfo` system calls safely on macOS.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
proc_pidinfo = "0.1"
```

Then, you can use the `proc_*` functions to get information about a process (including task info, file descriptors, etc).

```rust
use proc_pidinfo::*;

for fd in proc_pidinfo_list_self::<ProcFDInfo>().unwrap() {
    println!("{:?}", fd);
}
```
