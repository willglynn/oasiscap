use super::*;

#[test]
fn parse_appendix_adot1() {
    let alert: Alert = include_str!("../../fixtures/v1dot0_appendix_adot1.xml")
        .parse()
        .unwrap();
    assert_eq!(alert.identifier, "43b080713727");
}

#[test]
fn parse_appendix_adot2() {
    let alert: Alert = include_str!("../../fixtures/v1dot0_appendix_adot2.xml")
        .parse()
        .unwrap();
    assert_eq!(alert.identifier, "KSTO1055887203");
}

#[test]
fn parse_appendix_adot3() {
    let alert: Alert = include_str!("../../fixtures/v1dot0_appendix_adot3.xml")
        .parse()
        .unwrap();
    assert_eq!(alert.identifier, "TRI13970876.1");
}

#[test]
fn parse_appendix_adot4() {
    let alert: Alert = include_str!("../../fixtures/v1dot0_appendix_adot4.xml")
        .parse()
        .unwrap();
    assert_eq!(alert.identifier, "KAR0-0306112239-SW");
}
