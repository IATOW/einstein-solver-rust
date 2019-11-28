use crate::utils;

#[test]
fn random_layout() {
    for _ in 0..10 {
        let s = utils::random_layout();
        let temp = s.contains("1") &&
            s.contains("2") && 
            s.contains("3") &&
            s.contains("4") &&
            s.contains("5") &&
            s.contains("6") &&
            s.len() == 6;
        assert!(temp);
    }
}