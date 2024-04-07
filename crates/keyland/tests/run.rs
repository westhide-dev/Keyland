use insta::assert_snapshot;
use kcommon::nil::NIL;

#[test]
fn nil() {
    assert_snapshot!(format!("{NIL:?}"));
}
