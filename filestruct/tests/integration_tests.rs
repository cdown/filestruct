use filestruct::FromDir;
use std::path::{Path, PathBuf};
use std::str::FromStr;

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
fn option() {
    #[derive(FromDir)]
    struct F {
        t_u64_ok: Option<u64>,
        t_string_ok: Option<String>,
        t_string_ok_chomp: Option<String>,
    }

    let f = F::from_dir(get_test_dir()).unwrap();
    assert_eq!(f.t_u64_ok, Some(u64::MAX));
    assert_eq!(f.t_string_ok, Some("ĩ ľ𝝸ᶄ𝙚 ѕ𝓉ř⍳𝕟ℊ𝚜, ṁ𝚎\n".to_string()));
    assert_eq!(f.t_string_ok_chomp, Some("ĩ ľ𝝸ᶄ𝙚 ѕ𝓉ř⍳𝕟ℊ𝚜, ṁ𝚎".to_string()));
}

#[test]
fn err_doesnt_exist() {
    #[allow(dead_code)]
    #[derive(FromDir)]
    struct F {
        does_not_exist: u64,
    }

    let f = F::from_dir(get_test_dir());
    assert!(matches!(f, Err(filestruct::Error::Io { .. })));
}

#[test]
fn ok_doesnt_exist_but_option() {
    #[derive(FromDir)]
    struct F {
        does_not_exist: Option<u64>,
    }

    let f = F::from_dir(get_test_dir()).unwrap();
    assert_eq!(f.does_not_exist, None);
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

#[test]
fn trim_string() {
    #[derive(FromDir)]
    struct F {
        #[filestruct(trim = true)]
        t_string_ok: String,
    }

    let f = F::from_dir(get_test_dir()).unwrap();
    assert_eq!(f.t_string_ok, "ĩ ľ𝝸ᶄ𝙚 ѕ𝓉ř⍳𝕟ℊ𝚜, ṁ𝚎");
}

#[test]
fn from_cwd() {
    #[derive(FromDir)]
    struct F {
        t_string_ok: String,
    }
    std::env::set_current_dir(get_test_dir()).unwrap();
    let f = F::from_cwd().unwrap();
    assert_eq!(f.t_string_ok, "ĩ ľ𝝸ᶄ𝙚 ѕ𝓉ř⍳𝕟ℊ𝚜, ṁ𝚎\n");
}

#[derive(Debug, PartialEq, Eq)]
struct StealthyString(String);

impl FromStr for StealthyString {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(StealthyString(s.to_string()))
    }
}

#[test]
fn trim_non_string_by_default() {
    #[derive(FromDir)]
    struct F {
        t_string_ok: StealthyString,
    }

    let f = F::from_dir(get_test_dir()).unwrap();
    assert_eq!(
        f.t_string_ok,
        StealthyString("ĩ ľ𝝸ᶄ𝙚 ѕ𝓉ř⍳𝕟ℊ𝚜, ṁ𝚎".to_string())
    );
}

#[test]
fn attr_trim_false() {
    #[derive(FromDir)]
    struct F {
        #[filestruct(trim = false)]
        t_string_ok: StealthyString,
    }

    let f = F::from_dir(get_test_dir()).unwrap();
    assert_eq!(
        f.t_string_ok,
        StealthyString("ĩ ľ𝝸ᶄ𝙚 ѕ𝓉ř⍳𝕟ℊ𝚜, ṁ𝚎\n".to_string())
    );
}

#[test]
fn relative_dir() {
    #[derive(FromDir)]
    struct F {
        #[filestruct(relative_dir = "inner")]
        t_string_ok: String,
    }

    let f = F::from_dir(get_test_dir()).unwrap();
    assert_eq!(f.t_string_ok, "inner\n");
}

#[test]
fn all_attributes_together() {
    #[derive(FromDir)]
    struct F {
        #[filestruct(relative_dir = "inner", file = "t_string_ok", trim = true)]
        does_not_exist: String,
    }

    let f = F::from_dir(get_test_dir()).unwrap();
    assert_eq!(f.does_not_exist, "inner");
}

#[test]
fn compile_failures() {
    let t = trybuild::TestCases::new();
    let dir = get_test_dir();
    t.compile_fail(dir.join("bad_code/*.rs"));
}
