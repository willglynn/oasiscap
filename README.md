# `oasiscap`

Types for the OASIS [Common Alerting Protocol].

# Example

```rust
let alert: oasiscap::Alert = r#"
<?xml version = "1.0" encoding = "UTF-8"?>
<alert xmlns = "urn:oasis:names:tc:emergency:cap:1.2">
  <identifier>43b080713727</identifier>
  <sender>hsas@dhs.gov</sender>
  <sent>2003-04-02T14:39:01-05:00</sent>
  <status>Actual</status>
  <msgType>Alert</msgType>
  <scope>Public</scope>
  <info>
    <!-- … -->
  </info>
</alert>
"#.parse().expect("parse CAP");

match &alert {
    oasiscap::Alert::V1dot0(alert) => println!("CAP v1.0: {:?}", alert),
    oasiscap::Alert::V1dot1(alert) => println!("CAP v1.1: {:?}", alert),
    oasiscap::Alert::V1dot2(alert) => println!("CAP v1.2: {:?}", alert),
}

// Upgrade to the latest CAP version
let alert: oasiscap::v1dot2::Alert = alert.into_latest();

// Convert back to XML again
let alert_xml = alert.to_string();
```

# Conformance

The CAP specifications are split between human- and machine-readable components. CAP v1.2 § 4.2
explains:

> An XML 1.0 document is a conforming CAP V1.2 Message if and only if:
>
> a) it is valid according to [the schema] and
> 
> b) the content of its elements and the values of its attributes meet all the additional
>    mandatory requirements specified in Section 3.

Consider the `<polygon>` element. The machine-readable XML schema says that a polygon is just a
string:

```xml
<element name = "polygon" type = "xs:string" minOccurs = "0" maxOccurs = "unbounded"/>
```

The human-readable document says that a polygon is specifically a string describing a closed
polyline in a particular geospatial reference frame, and imposes the following requirements
in section 3:

> (1) Code Values: The geographic polygon is represented by a whitespace-delimited list of WGS
> 84 coordinate pairs. (See WGS 84 Note at end of this section)
>
> (2) A minimum of 4 coordinate pairs MUST be present and the first and last pairs of
> coordinates MUST be the same.

This crate implements those rules from section 3:

```rust
use oasiscap::v1dot2::Polygon;

// 4 points, where the last point is the first point, makes a Polygon:
assert!("1,1 2,2 3,3 1,1".parse::<Polygon>().is_ok());

// 4 points where the last point differs does not make a Polygon:
assert!("1,1 2,2 3,3 4,4".parse::<Polygon>().is_err());

// 3 points does not make a Polygon:
assert!("1,1 2,2 1,1".parse::<Polygon>().is_err());

// invalid WGS-84 coordinates do not make a Polygon:
assert!("100,100 200,200 300,300 100,100".parse::<Polygon>().is_err());
```

All of those strings are permitted by the XML schema, but only the first one makes sense as a
polygon. This crate therefore accepts the first string and rejects the others.

Having said that, some real-world CAP alerts violate the requirements in section 3 but _do_
still make sense:

```xml
<polygon></polygon>
```

Polygons are optional, so the element can and should have been omitted in its entirety. On the
other hand, an empty string _is_ valid according to the XML schema, and its intent is
unambiguous even if it is technically non-conforming. This crate therefore accepts an empty
polygon element as a synonym for omitting the polygon, rather than returning an error.

This crate intends to always parse conforming CAP messages and to always generate conforming CAP
messages. At the same time, this crate intends to be pedantic to preserve _meaning_, not to be
pendantic for pedantry's sake. It therefore does not reject all non-conforming CAP messages,
particularly for common implementation mistakes which have reasonable and unambiguous
interpretations.

# Performance

`oasiscap` prioritizes being correct over being fast, but it is still reasonably fast. On an
industry standard developer's laptop using unspecified versions of this library, Rust, and the
underlying operating system, parsing a typical `oasiscap::Alert` from XML takes approximately
55µs, for a throughput of roughly 18,000 alerts per second per core. Generating XML from a
typical `oasiscap::Alert` takes approximately 27µs, for a throughput of roughly 38,000 alerts
per second per core.

Clone the repository and run `cargo bench` to see how it performs in your environment.

[Common Alerting Protocol]: https://en.wikipedia.org/wiki/Common_Alerting_Protocol
[xml_serde]: https://crates.io/crates/xml_serde
[the schema]: http://docs.oasis-open.org/emergency/cap/v1.2/CAP-v1.2.xsd
