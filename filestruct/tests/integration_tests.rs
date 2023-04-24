use filestruct::FromDir;
use std::path::{Path, PathBuf};

fn get_test_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("test_data")
}

#[test]
fn bare() {
    #[derive(FromDir)]
    struct F {
        t_u64_ok: u64,
        t_string_ok: String,
        t_string_ok_chomp: String,
    }

    let f = F::from_dir(get_test_dir()).unwrap();
    assert_eq!(f.t_u64_ok, u64::MAX);
    assert_eq!(f.t_string_ok, "ĩ ľ𝝸ᶄ𝙚 ѕ𝓉ř⍳𝕟ℊ𝚜, ṁ𝚎\n");
    assert_eq!(f.t_string_ok_chomp, "ĩ ľ𝝸ᶄ𝙚 ѕ𝓉ř⍳𝕟ℊ𝚜, ṁ𝚎");
}

#[test]
fn attr_file() {
    #[derive(FromDir)]
    struct F {
        #[filestruct(file = "t_string_ok")]
        renamed: String,
    }

    let f = F::from_dir(get_test_dir()).unwrap();
    assert_eq!(f.renamed, "ĩ ľ𝝸ᶄ𝙚 ѕ𝓉ř⍳𝕟ℊ𝚜, ṁ𝚎\n");
}
