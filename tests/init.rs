extern crate taxonomy_mock as taxonomy;
extern crate openzwave_adapter;

use openzwave_adapter::OpenzwaveAdapter;
use taxonomy::adapter::AdapterManagerHandleImpl;

use std::sync::Arc;

#[test]
fn test_init() {
    let m = AdapterManagerHandleImpl;
    let arc = Arc::new(m);
    let adapter = OpenzwaveAdapter::init(&arc);
    assert!(true);
}
