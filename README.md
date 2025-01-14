# proc_pidinfo

A Rust library for accessing the `proc_pidinfo` and `proc_pidfdinfo` system calls safely on macOS.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
proc_pidinfo = "0.1"
```

Then, you can use the `proc_pidinfo` and `proc_pidfdinfo` functions to get information about a process's file descriptors.

```rust
use proc_pidinfo::*;

for fd in proc_pidinfo_list_self::<ProcFDInfo>().unwrap() {
    println!("{:?}", fd);
}
```
