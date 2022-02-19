use oasiscap;
use oasiscap::v1dot2::*;
use std::fs;
use std::path::Path;

// Ensure we can always parse fixtures/*.xml
#[test]
fn parse_all_fixtures() {
    for entry in fs::read_dir("fixtures").unwrap() {
        let entry = entry.unwrap();
        let file_name = entry.file_name();
        if let Some(name) = file_name.to_str() {
            if name.ends_with(".xml") {
                let path = Path::new("fixtures").join(&file_name);
                let bytes = fs::read(&path).unwrap();
                let string = match String::from_utf8(bytes) {
                    Ok(str) => str,
                    Err(e) => String::from_utf8_lossy(e.as_bytes()).into_owned(),
                };
                match string.parse::<oasiscap::Alert>() {
                    Ok(alert) => test_alert(name, alert),
                    Err(e) => panic!("error parsing {}: {}", name, e),
                };
            }
        }
    }
}

fn test_alert(name: &str, alert: oasiscap::Alert) {
    test_xml_roundtrip(name, &alert);
    test_proto(name, &alert);

    let upgraded_name = format!("upgraded {}", name);
    let upgraded = oasiscap::Alert::from(alert.into_latest());
    test_proto(&upgraded_name, &upgraded);
    test_xml_roundtrip(&upgraded_name, &upgraded);
}

#[cfg(not(feature = "prost"))]
fn test_proto(name: &str, alert: &oasiscap::v1dot2::Alert) {
    // no-op
}

#[cfg(feature = "prost")]
fn test_proto(name: &str, alert: &oasiscap::Alert) {
    use prost::Message;

    // convert to a protobuf::Alert
    let proto: oasiscap::protobuf::Alert = alert.clone().into();

    // roundtrip through bytes
    let bytes = proto.encode_length_delimited_to_vec();
    let reproto = oasiscap::protobuf::Alert::decode_length_delimited(bytes.as_slice())
        .expect("parse protobuf");
    let roundtrip = oasiscap::Alert::try_from(reproto).expect("from proto");
    assert_eq!(
        &roundtrip, alert,
        "mismatch roundtripping {} through protobuf",
        name
    );
}

fn test_xml_roundtrip(name: &str, alert: &oasiscap::Alert) {
    let xml = alert.to_string();
    let roundtrip: oasiscap::Alert = xml
        .parse()
        .map_err(|e| {
            eprintln!("error parsing {}", name);
            e
        })
        .unwrap();
    //println!("<!-- {} -->\n{}\n\n", name, &xml);
    assert_eq!(
        alert, &roundtrip,
        "mismatch roundtripping {} through XML",
        name
    );
}

#[test]
fn google_paaq_4_mg5a94() {
    // https://developers.google.com/public-alerts/samples/alert-updates#example_update
    let message = include_str!("../fixtures/google-PAAQ-4-mg5a94.xml");

    let alert: oasiscap::Alert = message.parse().expect("parse");
    let alert = match alert {
        oasiscap::Alert::V1dot2(alert) => alert,
        other => panic!("unexpected alert: {:?}", other),
    };
    assert_eq!(
        alert,
        oasiscap::v1dot2::Alert {
            identifier: "PAAQ-4-mg5a94".parse().unwrap(),
            sender: "wcatwc@noaa.gov".parse().unwrap(),
            sent: chrono::DateTime::parse_from_rfc3339("2013-01-05T10:58:23Z")
                .unwrap()
                .into(),
            status: Status::Actual,
            message_type: MessageType::Update,
            source: Some("WCATWC".into()),
            scope: Scope::Public,
            restriction: None,
            addresses: None,
            codes: vec!["IPAWSv1.0".into()],
            note: None,
            references: Some("wcatwc@noaa.gov,PAAQ-1-mg5a94,2013-01-05T09:01:16-00:00 wcatwc@noaa.gov,PAAQ-2-mg5a94,2013-01-05T09:30:16-00:00 wcatwc@noaa.gov,PAAQ-3-mg5a94,2013-01-05T10:17:31-00:00".parse().unwrap()),
            incidents: Some(vec!["mg5a94"].try_into().unwrap()),
            info: vec![oasiscap::v1dot2::Info{
                language: Default::default(),
                categories: vec![oasiscap::v1dot2::Category::Geo],
                event: "Tsunami Cancellation".into(),
                response_type: vec![oasiscap::v1dot2::ResponseType::None],
                urgency: Urgency::Past,
                severity: Severity::Unknown,
                certainty: Certainty::Unlikely,
                audience: None,
                event_codes: Default::default(),
                effective: None,
                onset: Some(chrono::DateTime::parse_from_rfc3339("2013-01-05T10:58:23Z").unwrap().into()),
                expires: Some(chrono::DateTime::parse_from_rfc3339("2013-01-05T10:58:23Z").unwrap().into()),
                sender_name: Some("NWS West Coast/Alaska Tsunami Warning Center Palmer AK".into()),
                headline: Some("The tsunami Warning is canceled for the coastal areas of British Columbia and Alaska from the north tip of Vancouver Island, British Columbia to Cape Fairweather, Alaska (80 miles SE of Yakutat).".into()),
                description: Some("The tsunami Warning is canceled for the coastal areas of British Columbia and Alaska from the north tip of Vancouver Island, British Columbia to Cape Fairweather, Alaska (80 miles SE of Yakutat). - Event details: Preliminary magnitude 7.5 (Mw) earthquake / Lat: 55.300, Lon: -134.900 at 2013-01-05T08:58:20Z Tsunami cancellations indicate the end of the damaging tsunami threat.  A cancellation is issued after an evaluation of sea level data confirms that a destructive tsunami will not impact the alerted region, or after tsunami levels have subsided to non-damaging levels.".into()),
                instruction: Some("Recommended Actions:   Do not re-occupy hazard zones until local emergency officials indicate it is safe to do so. This will be the last West Coast/Alaska Tsunami Warning Center message issued for this event.  Refer to the internet site ntwc.arh.noaa.gov for more information.".into()),
                web: Some("http://ntwc.arh.noaa.gov/events/PAAQ/2013/01/05/mg5a94/4/WEAK51/WEAK51.txt".parse().unwrap()),
                contact: None,
                parameters: oasiscap::v1dot2::Map::from_iter(vec![
                    ("EventLocationName", "95 miles NW of Dixon Entrance, Alaska"),
                    ("EventPreliminaryMagnitude", "7.5"),
                    ("EventPreliminaryMagnitudeType", "Mw"),
                    ("EventOriginTime", "2013-01-05T08:58:20-00:00"),
                    ("EventDepth", "5 kilometers"),
                    ("EventLatLon", "55.300,-134.900 0.000"),
                    ("VTEC", "/O.CAN.PAAQ.TS.W.0001.000000T0000Z-000000T0000Z/"),
                    ("NWSUGC", "BCZ220-210-922-912-921-911-110-AKZ026>029-023-024-019>022-025-051258-"),
                    ("ProductDefinition", "Tsunami cancellations indicate the end of the damaging tsunami threat.  A cancellation is issued after an evaluation of sea level data confirms that a destructive tsunami will not impact the alerted region, or after tsunami levels have subsided to non-damaging levels."),
                    ("WEAK51", "Public Tsunami Warnings, Watches, and Advisories for AK, BC, and US West Coast"),
                    ("EAS-ORG", "WXR"),
                ]),
                resources: vec![
                    oasiscap::v1dot2::Resource {
                        description: "Event Data as a JSON document".into(),
                        mime_type: "application/json".into(),
                        size: None,
                        uri: Some("http://ntwc.arh.noaa.gov/events/PAAQ/2013/01/05/mg5a94/4/WEAK51/PAAQ.json".parse().unwrap()),
                        embedded_content: None,
                        digest: None
                    },
                ],
                areas: vec![
                    oasiscap::v1dot2::Area {
                        description: "95 miles NW of Dixon Entrance, Alaska".into(),
                        polygons: vec![],
                        circles: vec![
                            "55.3,-134.9 0.0".parse().unwrap(),
                        ],
                        geocode: Default::default(),
                        altitude: None,
                        ceiling: None
                    }
                ]
            }]
        }
    );
}
