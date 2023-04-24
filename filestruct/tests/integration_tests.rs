use filestruct::FromDir;
use std::path::{Path, PathBuf};

fn get_test_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("test_data")
}

#[derive(FromDir)]
struct FPrimitive {
    t_u64_ok: u64,
}

#[test]
fn struct_basic() {
    let f = FPrimitive::from_dir(get_test_dir()).unwrap();
    assert_eq!(f.t_u64_ok, u64::MAX);
}
