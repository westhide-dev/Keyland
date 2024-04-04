use insta::assert_snapshot;
use keyland::prelude::*;

#[test]
fn nil() {
    assert_snapshot!(format!("{NIL:?}"));
}
