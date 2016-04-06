#![feature(plugin)]
#![cfg_attr(test, plugin(stainless))]


extern crate taxonomy_mock as taxonomy;
extern crate openzwave_adapter;

#[cfg(test)]
describe! library_integration_test {
    before_each {
        use openzwave_adapter::OpenzwaveAdapter;
        use taxonomy::adapter::AdapterManagerHandleImpl;

        use std::sync::Arc;
    }

    it "should init" {
        let m = AdapterManagerHandleImpl;
        let arc = Arc::new(m);
        let adapter = OpenzwaveAdapter::init(&arc).unwrap();
    }
}
