// This test walks over a downloaded copy of the FEMA IPAWS archive:
//
//   https://www.fema.gov/openfema-data-page/ipaws-alerts-repository-pulled-common-alerting-protocol-cap-aggregator-capv12
//
// This archive is >14 GB. If you dare:
//
// ```shell
// $ mkdir -p fixtures/ipaws_archive
// $ (cd fixtures/ipaws_archive; wget https://www.fema.gov/api/open/v1/IpawsArchivedAlerts.json)
// $ cargo test --test ipaws_archive --release -- --nocapture --ignored
// ```
//
// It spits out progress and error messages as it goes. Any alerts which fail to parse are written
// to `fixtures/ipaws/`, which is `.gitignore`d. Most of these are legitimate failures: the alert
// actually _isn't_ valid CAP. Investigate as you see fit and expand the test suite as appropriate.

use serde::de::{SeqAccess, Visitor};
use serde::{Deserialize, Deserializer};
use std::fs::File;
use std::io::BufReader;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Message {
    id: String,
    original_message: String,
}

impl Message {
    pub(crate) fn test(&self) {
        match self.original_message.parse::<oasiscap::Alert>() {
            Ok(oasiscap::Alert::V1dot0(_alert)) => {}
            Ok(oasiscap::Alert::V1dot1(_alert)) => {}
            Ok(oasiscap::Alert::V1dot2(alert)) => {
                let _ = alert;
                /*
                if alert
                    .info
                    .iter()
                    .any(|i| i.resources.iter().any(|r| r.digest.is_some()))
                {
                    std::fs::write(
                        format!("fixtures/digest-ipaws-{}.xml", self.id),
                        self.original_message.as_bytes(),
                    )
                    .expect("write");
                }
                 */
            }
            Err(e) => {
                eprintln!("{}: {}", self.id, e);
                std::fs::write(
                    format!("fixtures/ipaws/ipaws-{}.xml", self.id),
                    self.original_message.as_bytes(),
                )
                .expect("write");
            }
        }
    }
}

struct MessagesVistor;
impl<'de> Visitor<'de> for MessagesVistor {
    type Value = Messages;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("array of IPAWS messages")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let mut count = 0;

        while let Some(message) = seq.next_element::<Message>()? {
            if count % 1000 == 0 {
                println!("parsing #{}", count);
            }
            message.test();
            count += 1;
        }

        Ok(Messages { count })
    }
}

struct Messages {
    count: usize,
}
impl<'de> Deserialize<'de> for Messages {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_seq(MessagesVistor)
    }
}

#[test]
#[ignore]
fn scan() {
    let file = File::open("fixtures/ipaws_archive/IpawsArchivedAlerts.json").expect("open file");
    std::fs::create_dir_all("fixtures/ipaws").expect("create output");

    #[derive(Deserialize)]
    struct Archive {
        #[serde(rename = "IpawsArchivedAlerts")]
        alerts: Messages,
    }

    let archive: Archive = serde_json::from_reader(BufReader::new(file)).expect("parse");
    println!("tested against {} messages", archive.alerts.count);
}
