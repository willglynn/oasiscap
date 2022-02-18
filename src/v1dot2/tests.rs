use super::*;
use chrono::prelude::*;

#[test]
fn parse_appendix_adot1() {
    let alert: Alert = include_str!("../../fixtures/v1dot2_appendix_adot1.xml")
        .parse()
        .unwrap();
    assert_eq!(alert.identifier, "43b080713727");
    assert_eq!(alert.sender, "hsas@dhs.gov");
    assert_eq!(
        alert.sent,
        crate::v1dot2::DateTime::from(
            FixedOffset::west(5 * 3600)
                .from_local_datetime(&NaiveDateTime::new(
                    NaiveDate::from_ymd(2003, 4, 2),
                    NaiveTime::from_hms(14, 39, 1)
                ))
                .unwrap()
        )
    );
    assert_eq!(alert.status, Status::Actual);
    assert_eq!(alert.message_type, MessageType::Alert);
    assert_eq!(alert.scope, Scope::Public);
    assert_eq!(alert.info.len(), 1);
    let info = alert.info.first().unwrap();
    assert_eq!(info.categories, vec![Category::Security]);
    assert_eq!(info.event, "Homeland Security Advisory System Update");
    assert_eq!(info.urgency, Urgency::Immediate);
    assert_eq!(info.severity, Severity::Severe);
    assert_eq!(info.certainty, Certainty::Likely);
    assert_eq!(
        info.sender_name.as_ref().unwrap(),
        "U.S. Government, Department of Homeland Security"
    );
    assert_eq!(
        info.headline.as_ref().unwrap(),
        "Homeland Security Sets Code ORANGE"
    );
    assert_eq!(info.description.as_ref().unwrap().split_whitespace().collect::<Vec<_>>(), "The Department of Homeland Security has elevated the Homeland Security Advisory System threat level to ORANGE / High in response to intelligence which may indicate a heightened threat of terrorism.".split_whitespace().collect::<Vec<_>>());
    assert_eq!(info.instruction.as_ref().unwrap().split_whitespace().collect::<Vec<_>>(), "A High Condition is declared when there is a high risk of terrorist attacks. In addition to the Protective Measures taken in the previous Threat Conditions, Federal departments and agencies should consider agency-specific Protective Measures in accordance with their existing plans.".split_whitespace().collect::<Vec<_>>());
    assert_eq!(
        info.web.as_ref().unwrap(),
        &"http://www.dhs.gov/dhspublic/display?theme=29"
            .parse::<url::Url>()
            .unwrap()
    );

    assert_eq!(info.parameters.len(), 1);
    assert_eq!(info.parameters.get("HSAS"), Some("ORANGE"));
    let param = info.parameters.iter().next().unwrap();
    assert_eq!(param, ("HSAS", "ORANGE")); // http://www.zefrank.com/redalert/

    assert_eq!(info.resources.len(), 1);
    let resource = info.resources.iter().next().unwrap();
    assert_eq!(resource.description, "Image file (GIF)");
    assert_eq!(resource.mime_type, "image/gif");
    assert_eq!(
        resource.uri.as_ref().unwrap().as_str(),
        "http://www.dhs.gov/dhspublic/getAdvisoryImage"
    );

    assert_eq!(info.areas.len(), 1);
    let area = info.areas.iter().next().unwrap();
    assert_eq!(area.description, "U.S. nationwide and interests worldwide");
}

#[test]
fn parse_appendix_adot2() {
    let alert: Alert = include_str!("../../fixtures/v1dot2_appendix_adot2.xml")
        .parse()
        .unwrap();
    assert_eq!(alert.identifier, "KSTO1055887203");
}

#[test]
fn parse_appendix_adot3() {
    let alert: Alert = include_str!("../../fixtures/v1dot2_appendix_adot3.xml")
        .parse()
        .unwrap();
    assert_eq!(alert.identifier, "TRI13970876.2");
}

#[test]
fn parse_appendix_adot4() {
    let alert: Alert = include_str!("../../fixtures/v1dot2_appendix_adot4.xml")
        .parse()
        .unwrap();
    assert_eq!(alert.identifier, "KAR0-0306112239-SW");
}
