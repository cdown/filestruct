# structfromdir

A Rust derive macro which permits reading struct values from a directory.

Not ready for production use, still in heavy development and many things are
not yet implemented or will unexpectedly blow up.

# Usage

```rust
use structfromdir::FromDir;

#[derive(FromDir, Debug)]
struct Files {
    capacity: u8,
    energy_now: u64,
}

fn main() {
    let files = Files::from_dir("/sys/class/power_supply/BAT0");
    println!("{:?}", files);
}
```

Results in:

```rust
Ok(Files { capacity: 68, energy_now: 38780000 })
```
