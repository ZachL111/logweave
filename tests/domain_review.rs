use logweave::domain_review::{review_lane, review_score, DomainCase};

#[test]
fn domain_review_case_is_stable() {
    let case = DomainCase { signal: 71, slack: 49, drag: 22, confidence: 92 };
    assert_eq!(review_score(case), 217);
    assert_eq!(review_lane(case), "ship");
}
