# filestruct | [![Tests](https://img.shields.io/github/actions/workflow/status/cdown/filestruct/ci.yml?branch=master)](https://github.com/cdown/filestruct/actions?query=branch%3Amaster)

A Rust derive macro which permits reading struct values from a directory.

Not ready for production use, still in heavy development and many things are
not yet implemented or will unexpectedly blow up.

## Usage

```rust
use filestruct::FromDir;

#[derive(FromDir, Debug)]
struct Files {
    comm: String,
    #[filestruct(file = "comm", trim = true)]
    comm_trimmed: String,
    oom_score: u32,
    does_not_exist: Option<u32>,
    #[filestruct(file = "oom_score_adj")]
    does_not_exist_but_renamed: Option<u32>,
    #[filestruct(relative_dir = "..", trim = true)]
    uptime: String,
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
        comm: "pdm-bin\n",
        comm_trimmed: "pdm-bin",
        oom_score: 800,
        does_not_exist: None,
        does_not_exist_but_renamed: Some(
            200,
        ),
        uptime: "177405.74 822813.82",
    },
)
```

## Releases

Releases are a little complicated because filestruct_derive and filestruct are
separate crates. Use `cargo release`:

```
cargo release --execute -- minor
```
