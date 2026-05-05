use logweave::{classify, score, Signal};
#[test]
fn fixture_decisions() {
    let signal = Signal { demand: 55, capacity: 100, latency: 8, risk: 14, weight: 8 };
    assert_eq!(score(signal), 170);
    assert_eq!(classify(signal), "review");
    let signal = Signal { demand: 79, capacity: 71, latency: 10, risk: 9, weight: 12 };
    assert_eq!(score(signal), 223);
    assert_eq!(classify(signal), "accept");
    let signal = Signal { demand: 90, capacity: 72, latency: 18, risk: 6, weight: 8 };
    assert_eq!(score(signal), 214);
    assert_eq!(classify(signal), "accept");
}
