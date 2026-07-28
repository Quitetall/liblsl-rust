use lsl;
use lsl::{ExPushable, Pullable};

// Compile-time assertion: signed 64-bit transport must exist on every target.
fn assert_i64_transport_traits()
where
    lsl::StreamOutlet: ExPushable<Vec<i64>>,
    lsl::StreamInlet: Pullable<i64>,
{
}

#[test]
fn i64_transport_traits_are_available() {
    assert_i64_transport_traits();
}

#[test]
fn clock_is_working() {
    assert_ne!(lsl::local_clock(), 0.0);
}

#[test]
fn streaminfo_basic() {
    let info = lsl::StreamInfo::new("MyStream", "EEG", 8, 100.0, lsl::ChannelFormat::Float32, "12345").unwrap();
    assert_eq!(info.stream_name(), "MyStream");
    assert_eq!(info.stream_type(), "EEG");
    assert_eq!(info.channel_count(), 8);
    assert_eq!(info.nominal_srate(), 100.0);
    assert_eq!(info.channel_format(), lsl::ChannelFormat::Float32);
    assert_eq!(info.source_id(), "12345");
    assert!(info.matches_query("name='MyStream' and type='EEG'"));
    assert!(!info.matches_query("name='MyStream' and type='ECG'"));
    let info2 = info.clone();
    assert_eq!(info2.stream_name(), "MyStream");
}

#[test]
fn streaminfo_xml() {
    let mut info = lsl::StreamInfo::new("MyStream", "EEG", 8, 100.0, lsl::ChannelFormat::Float32, "12345").unwrap();

    let mut channels = info.desc().append_child("channels");
    let mut chn = channels.append_child("channel");
    chn.append_child_value("label", "MyChannel");
    assert_eq!(channels.child("channel").child_value_named("label"), "MyChannel");

    let xml = info.to_xml().unwrap();
    assert!(xml.contains("<name>MyStream</name>"));
    assert!(xml.contains("<label>MyChannel</label>"));
}

#[test]
fn i64_sample_roundtrip() {
    let info = lsl::StreamInfo::new(
        "RustInt64RoundTrip",
        "Test",
        3,
        lsl::IRREGULAR_RATE,
        lsl::ChannelFormat::Int64,
        "liblsl-rust-int64-roundtrip",
    )
    .unwrap();
    let outlet = lsl::StreamOutlet::new(&info, 0, 16).unwrap();
    let inlet = lsl::StreamInlet::new(&info, 16, 1, false).unwrap();
    inlet.open_stream(5.0).unwrap();
    assert!(outlet.wait_for_consumers(5.0));

    let expected = vec![i64::MIN, 0, i64::MAX];
    outlet
        .push_sample_ex(&expected, lsl::local_clock(), true)
        .unwrap();
    let (actual, timestamp): (Vec<i64>, _) = inlet.pull_sample(5.0).unwrap();

    assert_eq!(actual, expected);
    assert!(timestamp > 0.0);
}
