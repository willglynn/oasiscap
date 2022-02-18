// Generated from `prost-build`, so:
#![allow(missing_docs)]

// Protocol buffer representation of the CAP spec.
// Supports versions 1.0, 1.1, and 1.2.

/// Represents a group field in the CAP spec, stored to XML as a
/// space-delimited string.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Group {
    #[prost(string, repeated, tag = "1")]
    pub value: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// A (valueName, value) pair within a CAP message.
/// Used for alert/info/eventCode, alert/info/parameter,
/// alert/info/area/geocode.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValuePair {
    #[prost(string, required, tag = "1")]
    pub value_name: ::prost::alloc::string::String,
    #[prost(string, required, tag = "2")]
    pub value: ::prost::alloc::string::String,
}
/// WGS-84 coordinate pair
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Point {
    #[prost(double, required, tag = "1")]
    pub latitude: f64,
    #[prost(double, required, tag = "2")]
    pub longitude: f64,
}
/// The paired values of points defining a polygon that delineates the affected
/// area of the alert message.  A minimum of 4 coordinate pairs MUST be present
/// and the first and last pairs of coordinates MUST be the same.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Polygon {
    #[prost(message, repeated, tag = "1")]
    pub point: ::prost::alloc::vec::Vec<Point>,
}
/// The paired values of a point and radius delineating the affected area of
/// the alert message.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Circle {
    #[prost(message, required, tag = "1")]
    pub point: Point,
    /// Radius is expressed in kilometers.
    #[prost(double, required, tag = "2")]
    pub radius: f64,
}
/// The container for all component parts of the area sub-element of the
/// info sub-element of the alert message.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Area {
    /// A text description of the affected area.
    #[prost(string, required, tag = "1")]
    pub area_desc: ::prost::alloc::string::String,
    /// The paired values of points defining a polygon that delineates the affected
    /// area of the alert message.
    #[prost(message, repeated, tag = "2")]
    pub polygon: ::prost::alloc::vec::Vec<Polygon>,
    /// The paired values of a point and radius delineating the affected area of
    /// the alert message.
    #[prost(message, repeated, tag = "3")]
    pub circle: ::prost::alloc::vec::Vec<Circle>,
    /// The geographic code delineating the affected area of the alert message,
    /// where the content of ?valueName? is a user-assigned string designating
    /// the domain of the code, and the content of ?value? is a string (which
    /// may represent a number) denoting the value itself
    /// (e.g., valueName="SAME" and value="006113").
    /// This element is primarily for compatibility with other systems. Use of
    /// this element presumes knowledge of the coding system on the part of
    /// recipients; therefore, for interoperability, it SHOULD be used in
    /// concert with an equivalent description in the more universally understood
    /// <polygon> and <circle> forms whenever possible.
    #[prost(message, repeated, tag = "4")]
    pub geocode: ::prost::alloc::vec::Vec<ValuePair>,
    /// The specific or minimum altitude of the affected area of the alert
    /// message. If used with the <ceiling> element this value is the lower limit
    /// of a range. Otherwise, this value specifies a specific altitude.
    /// The altitude measure is in feet above mean sea level per the \[WGS-84\]
    /// datum.
    #[prost(double, optional, tag = "5")]
    pub altitude: ::core::option::Option<f64>,
    /// The maximum altitude of the affected area of the alert message.
    /// MUST NOT be used except in combination with the <altitude> element.
    /// The ceiling measure is in feet above mean sea level per the \[WGS-84\]
    /// datum.
    #[prost(double, optional, tag = "6")]
    pub ceiling: ::core::option::Option<f64>,
}
/// The container for all component parts of the resource sub-element of
/// the info sub-element of the alert message.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Resource {
    /// The human-readable text describing the content and kind, such as
    /// "map" or "photo", of the resource file
    #[prost(string, required, tag = "1")]
    pub resource_desc: ::prost::alloc::string::String,
    /// MIME content type and sub-type as described in [RFC 2046].
    /// (As of this document, the current IANA registered MIME types are
    /// listed at <http://www.iana.org/assignments/media-types/>)
    /// Required as of CAP 1.2
    #[prost(string, optional, tag = "2")]
    pub mime_type: ::core::option::Option<::prost::alloc::string::String>,
    /// Approximate size of the resource file in bytes.
    #[prost(int64, optional, tag = "3")]
    pub size: ::core::option::Option<i64>,
    /// A full absolute URI, typically a Uniform Resource Locator that can
    /// be used to retrieve the resource over the Internet
    /// OR
    /// a relative URI to name the content of a <derefUri> element if one is
    /// present in this resource block.
    #[prost(string, optional, tag = "4")]
    pub uri: ::core::option::Option<::prost::alloc::string::String>,
    /// The base-64 encoded data content of the resource file.
    /// MAY be used either with or instead of the <uri> element in messages
    /// transmitted over one-way (e.g., broadcast) data links where retrieval
    /// of a resource via a URI is not feasible.
    /// Clients intended for use with one-way data links MUST support this
    /// element.
    /// This element MUST NOT be used unless the sender is certain that all
    /// direct clients are capable of processing it.
    /// If messages including this element are forwarded onto a two-way
    /// network, the forwarder MUST strip the <derefUri> element and SHOULD
    /// extract the file contents and provide a <uri> link to a
    /// retrievable version of the file.
    /// Providers of one-way data links MAY enforce additional restrictions
    /// on the use of this element, including message-size limits and
    /// restrictions regarding file types.
    /// Added in CAP 1.1
    #[prost(string, optional, tag = "5")]
    pub deref_uri: ::core::option::Option<::prost::alloc::string::String>,
    /// The code representing the digital digest (?hash?) computed from the
    /// resource from the resource file. Calculated using the Secure Hash
    /// Algorithm (SHA-1) per [FIPS 180-2]
    #[prost(string, optional, tag = "6")]
    pub digest: ::core::option::Option<::prost::alloc::string::String>,
}
/// The container for all component parts of the info sub-element of the
/// alert message.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Info {
    /// RFC 3066 language code.
    #[prost(string, optional, tag = "1", default = "en-US")]
    pub language: ::core::option::Option<::prost::alloc::string::String>,
    /// The code denoting the category of the subject event of the alert
    /// message. Required as of CAP 1.1.
    #[prost(enumeration = "info::Category", repeated, packed = "false", tag = "2")]
    pub category: ::prost::alloc::vec::Vec<i32>,
    /// The text denoting the type of the subject event of the alert message.
    #[prost(string, required, tag = "3")]
    pub event: ::prost::alloc::string::String,
    /// The code denoting the type of action recommended for the target
    /// audience. Added in CAP 1.1
    #[prost(
        enumeration = "info::ResponseType",
        repeated,
        packed = "false",
        tag = "4"
    )]
    pub response_type: ::prost::alloc::vec::Vec<i32>,
    /// The code denoting the urgency of the subject event of the alert message.
    #[prost(enumeration = "info::Urgency", required, tag = "5")]
    pub urgency: i32,
    /// The code denoting the severity of the subject event of the alert
    /// message.
    #[prost(enumeration = "info::Severity", required, tag = "6")]
    pub severity: i32,
    /// The code denoting the certainty of the subject event of the alert
    /// message.
    #[prost(enumeration = "info::Certainty", required, tag = "7")]
    pub certainty: i32,
    /// The text describing the intended audience of the alert message.
    #[prost(string, optional, tag = "8")]
    pub audience: ::core::option::Option<::prost::alloc::string::String>,
    /// A system-specific code identifying the event type of the alert message,
    /// where the content of ?valueName? is a user-assigned string designating
    /// the domain of the code, and the content of ?value? is a string (which
    /// may represent a number) denoting the value itself
    /// (e.g., valueName="SAME" and value="CEM")
    #[prost(message, repeated, tag = "9")]
    pub event_code: ::prost::alloc::vec::Vec<ValuePair>,
    /// The effective time of the information of the alert message.
    /// If this item is not included, the effective time SHALL be assumed to be
    /// the same as in Alert#sent.
    /// The date and time is represented in \[dateTime\] format
    /// (e. g., "2002-05-24T16:49:00-07:00" for 24 May 2002 at
    /// 16: 49 PDT).  Alphabetic timezone designators such as "Z"
    /// MUST NOT be used.  The timezone for UTC MUST be represented
    /// as "-00:00" or "+00:00".
    #[prost(string, optional, tag = "10")]
    pub effective: ::core::option::Option<::prost::alloc::string::String>,
    /// The expected time of the beginning of the subject event of alert message
    /// The date and time is represented in \[dateTime\] format
    /// (e. g., "2002-05-24T16:49:00-07:00" for 24 May 2002 at
    /// 16: 49 PDT).  Alphabetic timezone designators such as "Z"
    /// MUST NOT be used.  The timezone for UTC MUST be represented
    /// as "-00:00" or "+00:00".
    #[prost(string, optional, tag = "11")]
    pub onset: ::core::option::Option<::prost::alloc::string::String>,
    /// The expiry time of the information of the alert message.
    /// If this item is not provided, each recipient is free to set its own
    /// policy as to when the message is no longer in effect.
    /// The date and time is represented in \[dateTime\] format
    /// (e. g., "2002-05-24T16:49:00-07:00" for 24 May 2002 at
    /// 16: 49 PDT).  Alphabetic timezone designators such as "Z"
    /// MUST NOT be used.  The timezone for UTC MUST be represented
    /// as "-00:00" or "+00:00".
    #[prost(string, optional, tag = "12")]
    pub expires: ::core::option::Option<::prost::alloc::string::String>,
    /// The human-readable name of the agency or authority issuing this alert.
    #[prost(string, optional, tag = "13")]
    pub sender_name: ::core::option::Option<::prost::alloc::string::String>,
    /// A brief human-readable headline.  Note that some displays (for example,
    /// short messaging service devices) may only present this headline; it
    /// SHOULD be made as direct and actionable as possible while remaining
    /// short.  160 characters MAY be a useful target limit for headline length.
    #[prost(string, optional, tag = "14")]
    pub headline: ::core::option::Option<::prost::alloc::string::String>,
    /// An extended human readable description of the hazard or event that
    /// occasioned this message.
    #[prost(string, optional, tag = "15")]
    pub description: ::core::option::Option<::prost::alloc::string::String>,
    /// An extended human readable instruction to targeted recipients.  (If
    /// different instructions are intended for different recipients, they
    /// should be represented by use of multiple <info> blocks.
    #[prost(string, optional, tag = "16")]
    pub instruction: ::core::option::Option<::prost::alloc::string::String>,
    /// A full, absolute URI for an HTML page or other text resource with
    /// additional or reference information regarding this alert.
    #[prost(string, optional, tag = "17")]
    pub web: ::core::option::Option<::prost::alloc::string::String>,
    /// The text describing the contact for follow-up and confirmation of
    /// the alert message.
    #[prost(string, optional, tag = "18")]
    pub contact: ::core::option::Option<::prost::alloc::string::String>,
    /// A system-specific additional parameter associated with the alert
    /// message, where the content of ?valueName? is a user-assigned string
    /// designating the domain of the code, and the content of ?value? is a
    /// string (which may represent a number) denoting the value itself
    /// (e.g., valueName="SAME" and value="CEM")
    #[prost(message, repeated, tag = "19")]
    pub parameter: ::prost::alloc::vec::Vec<ValuePair>,
    /// Refers to an additional file with supplemental information related to
    /// this <info> element; e.g. an image or audio file
    #[prost(message, repeated, tag = "20")]
    pub resource: ::prost::alloc::vec::Vec<Resource>,
    /// Multiple occurrences permitted, in which case the target area for the
    /// <info> block is the union of all the included <area> blocks
    /// If multiple <polygon>, <circle> or <geocode> elements are included, the
    /// area described by this <area> is the union of those represented by the
    /// included elements.
    #[prost(message, repeated, tag = "21")]
    pub area: ::prost::alloc::vec::Vec<Area>,
}
/// Nested message and enum types in `Info`.
pub mod info {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Category {
        /// Geophysical (inc. landslide)
        Geo = 0,
        /// Meteorological (inc. flood)
        Met = 1,
        /// General emergency and public safety
        Safety = 2,
        /// Law enforcement, military, homeland and local/private
        Security = 3,
        /// security
        ///
        /// Rescue and recovery
        Rescue = 4,
        /// Fire suppression and rescue
        Fire = 5,
        /// Medical and public health
        Health = 6,
        /// Pollution and other environmental
        Env = 7,
        /// Public and private transportation
        Transport = 8,
        /// Utility, telecommunication, other non-transport
        Infra = 9,
        /// infrastructure
        ///
        /// Chemical, Biological, Radiological, Nuclear or
        Cbrne = 10,
        /// High-Yield Explosive threat or attack
        /// Added in CAP 1.1
        ///
        /// Other events
        Other = 11,
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ResponseType {
        /// Take shelter in place or per <instruction>
        Shelter = 0,
        /// Relocate as instructed in the <instruction>
        Evacuate = 1,
        /// Make preparations per the <instruction>
        Prepare = 2,
        /// Execute a pre-planned activity identified in
        Execute = 3,
        /// <instruction>
        ///
        /// Avoid the subject event as per the <instruction>
        Avoid = 4,
        /// Added in CAP 1.2.
        ///
        /// Attend to information sources as described in
        Monitor = 5,
        /// <instruction>
        ///
        /// Evaluate the information in this message. (This value
        Assess = 6,
        /// SHOULD NOT be used in public warning applications.)
        ///
        /// The subject event no longer poses a thread or concern
        AllClear = 7,
        /// and any follow on action is described in <instruction>.
        /// Added in CAP 1.2.
        ///
        /// No action recommended.
        None = 8,
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Urgency {
        /// Responsive action SHOULD be taken immediately
        Immediate = 0,
        /// Responsive action SHOULD be taken soon
        Expected = 1,
        /// (within next hour)
        ///
        /// Responsive action SHOULD be taken in the near future
        Future = 2,
        /// Responsive action is no longer required
        Past = 3,
        /// Urgency not known
        UnknownUrgency = 4,
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Severity {
        /// Extraordinary threat to life or property
        Extreme = 0,
        /// Significant threat to life or property
        Severe = 1,
        /// Possible threat to life or property
        Moderate = 2,
        /// Minimal threat to life or property
        Minor = 3,
        /// Severity unknown
        UnknownSeverity = 4,
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Certainty {
        /// Determined to have occurred or to be ongoing.
        Observed = 0,
        /// DEPRECATED as of CAP 1.1, use Likely instead
        VeryLikely = 1,
        /// Likely (p > ~50%)
        Likely = 2,
        /// Possible but not likely (p <= ~50%)
        Possible = 3,
        /// Not expected to occur (p ~ 0)
        Unlikely = 4,
        /// Certainty unknown
        UnknownCertainty = 5,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Alert {
    /// XML Namespace of the alert.
    /// 1.0: xmlns="<http://www.incident.com/cap/1.0">
    /// 1.1: xmlns="urn:oasis:names:tc:emergency:cap:1.1"
    /// 1.2: xmlns="urn:oasis:names:tc:emergency:cap:1.2"
    #[prost(string, required, tag = "1")]
    pub xmlns: ::prost::alloc::string::String,
    /// A number or string uniquely identifying this message, assigned by the
    /// sender. MUST NOT include spaces, commas or restricted characters (< and &)
    #[prost(string, required, tag = "2")]
    pub identifier: ::prost::alloc::string::String,
    /// Identifies the originator of this alert. Guaranteed by assigner to be
    /// unique globally; e.g., may be based on an Internet domain name.
    /// MUST NOT include spaces, commas or restricted characters (< and &)
    #[prost(string, required, tag = "3")]
    pub sender: ::prost::alloc::string::String,
    /// DEPRECATED as of CAP 1.1 and a security risk in CAP 1.0
    #[deprecated]
    #[prost(string, optional, tag = "4")]
    pub password: ::core::option::Option<::prost::alloc::string::String>,
    /// The time and date of the origination of the alert message.
    /// The date and time is represented in \[dateTime\] format
    /// (e. g., "2002-05-24T16:49:00-07:00" for 24 May 2002 at
    /// 16: 49 PDT).  Alphabetic timezone designators such as "Z"
    /// MUST NOT be used.  The timezone for UTC MUST be represented
    /// as "-00:00" or "+00:00".
    #[prost(string, required, tag = "5")]
    pub sent: ::prost::alloc::string::String,
    /// The code denoting the appropriate handling of the alert message.
    #[prost(enumeration = "alert::Status", required, tag = "6")]
    pub status: i32,
    /// The code denoting the nature of the alert message.
    #[prost(enumeration = "alert::MsgType", required, tag = "7")]
    pub msg_type: i32,
    /// The text identifying the source of the alert message.
    #[prost(string, optional, tag = "8")]
    pub source: ::core::option::Option<::prost::alloc::string::String>,
    /// The code denoting the intended distribution of the alert message.
    /// Mandatory as of CAP 1.1.
    #[prost(enumeration = "alert::Scope", optional, tag = "9")]
    pub scope: ::core::option::Option<i32>,
    /// The text describing the rule for limiting distribution of the restricted
    /// alert message.
    /// Used when <scope> value is "Restricted"
    #[prost(string, optional, tag = "10")]
    pub restriction: ::core::option::Option<::prost::alloc::string::String>,
    /// The group listing of intended recipients of the private alert message.
    /// Used when <scope> value is "Private". Each recipient SHALL be identified
    /// by an identifier or an address.
    #[prost(message, optional, tag = "11")]
    pub addresses: ::core::option::Option<Group>,
    /// Any user-defined flag or special code used to flag the alert message for
    /// special handling.
    #[prost(string, repeated, tag = "12")]
    pub code: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The message note is primarily intended for use with <status> "Exercise"
    /// and <msgtype> "Error".
    #[prost(string, optional, tag = "13")]
    pub note: ::core::option::Option<::prost::alloc::string::String>,
    /// The group listing identifying earlier message(s) referenced by the alert
    /// message. The extended message identifier(s) (in the form
    /// sender,identifier,sent) of an earlier CAP message or messages referenced
    /// by this one. If multiple messages are referenced, they SHALL be separated
    /// by whitespace.
    #[prost(message, optional, tag = "14")]
    pub references: ::core::option::Option<Group>,
    /// The group listing naming the referent incident(s) of the alert message.
    /// Used to collate multiple messages referring to different aspects of the
    /// same incident.
    #[prost(message, optional, tag = "15")]
    pub incidents: ::core::option::Option<Group>,
    /// The container for all component parts of the info sub-element of the
    /// alert message.
    /// If targeting of multiple "info" blocks in the same language overlaps,
    /// information in later blocks may expand but may not override the
    /// corresponding values in earlier ones. Each set of "info" blocks
    /// containing the same language identifier SHALL be treated as a separate
    /// sequence.
    #[prost(message, repeated, tag = "16")]
    pub info: ::prost::alloc::vec::Vec<Info>,
}
/// Nested message and enum types in `Alert`.
pub mod alert {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Status {
        /// Actionable by all targeted recipients
        Actual = 0,
        /// Actionable only by designated exercise participants;
        Exercise = 1,
        /// exercise identifier SHOULD appear in <note>
        ///
        /// For messages that support alert network internal functions.
        System = 2,
        /// Technical testing only, all recipients disregard.
        Test = 3,
        /// A preliminary template or draft, not actionable in its
        Draft = 4,
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum MsgType {
        /// Initial information requiring attention by targeted
        Alert = 0,
        /// recipients
        ///
        /// Updates and supercedes the earlier message(s) identified in
        Update = 1,
        /// <references>
        ///
        /// Cancels the earlier message(s) identified in <references>
        Cancel = 2,
        /// Acknowledges receipt and acceptance of the message(s))
        Ack = 3,
        /// identified in <references>
        ///
        /// Indicates rejection of the message(s) identified in
        Error = 4,
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Scope {
        /// For general dissemination to unrestricted audiences
        Public = 0,
        /// For dissemination only to users with a known operational
        Restricted = 1,
        /// requirement (see <restriction>, below)
        ///
        /// For dissemination only to specified addresses
        Private = 2,
    }
}
