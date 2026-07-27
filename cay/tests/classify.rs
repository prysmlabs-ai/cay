use cay::usb::{classify, State, DFU_PRODUCT, DFU_VENDOR, RUNTIME_PRODUCT, RUNTIME_VENDOR};

#[test]
fn classifies_known_states() {
    assert_eq!(classify(DFU_VENDOR, DFU_PRODUCT), Some(State::Dfu));
    assert_eq!(
        classify(RUNTIME_VENDOR, RUNTIME_PRODUCT),
        Some(State::Runtime)
    );
    assert_eq!(classify(0x1234, 0x5678), None);
    assert_eq!(classify(RUNTIME_VENDOR, DFU_PRODUCT), None);
}
