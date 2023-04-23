# filestruct

A Rust derive macro which permits reading struct values from a directory.

Not ready for production use, still in heavy development and many things are
not yet implemented or will unexpectedly blow up.

## Usage

```rust
use filestruct::FromDir;

#[derive(FromDir, Debug)]
struct Files {
    comm: String,
    #[filestruct(file = "comm", trim)]
    comm_trimmed: String,
    oom_score: u32,

    does_not_exist: Option<u32>,
    #[filestruct(file = "oom_score_adj")]
    does_not_exist_but_renamed: Option<u32>,
}

fn main() {
    let files = Files::from_dir("/proc/self");
    println!("{:#?}", files);
}
```

Results in:

```rust
Ok(
    Files {
        comm: "somecomm\n",
        comm_trimmed: "somecomm",
        oom_score: 800,
        does_not_exist: None,
        does_not_exist_but_renamed: Some(
            200,
        ),
    },
)
```

## Releases

Releases are a little complicated because filestruct_derive and filestruct are
separate crates. Use `cargo release`:

```
cargo release --execute -- minor
```
