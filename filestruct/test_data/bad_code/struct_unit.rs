use filestruct::FromDir;

fn main() {
    #[allow(dead_code)]
    #[derive(FromDir)]
    struct F;
}
