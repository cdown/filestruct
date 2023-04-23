# filestruct

A Rust derive macro which permits reading struct values from a directory.

Not ready for production use, still in heavy development and many things are
not yet implemented or will unexpectedly blow up.

# Usage

```rust
use filestruct::FromDir;

#[derive(FromDir, Debug)]
struct Files {
    capacity: u8,
    energy_now: u64,
    does_not_exist: Option<u64>,
    #[filestruct(file = "energy_full")]
    does_not_exist_but_renamed: Option<u64>,
}

fn main() {
    let files = Files::from_dir("/sys/class/power_supply/BAT0");
    println!("{:#?}", files);
}
```

Results in:

```rust
Ok(
    Files {
        capacity: 67,
        energy_now: 38460000,
        does_not_exist: None,
        does_not_exist_but_renamed: Some(
            56970000,
        ),
    },
)
```
