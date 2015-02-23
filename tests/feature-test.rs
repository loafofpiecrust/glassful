extern crate glassful;

use std::borrow::ToOwned;

const TEST_PROG: &'static str = r#"
#![version="330"]


"#;

#[test]
fn feature_test() {
	glassful::translate(TEST_PROG.to_owned());
}