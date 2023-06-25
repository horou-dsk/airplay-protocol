use super::AirPlayConfig;
use bytes::{BufMut, Bytes};
use plist::Value::{self, Dictionary};
use sha1::{Digest, Sha1};

pub fn prepare_info_response(airplay_config: &AirPlayConfig) -> Bytes {
    let mut audio_format_100 = plist::Dictionary::default();
    audio_format_100.insert("audioInputFormats".to_string(), 67108860.into());
    audio_format_100.insert("audioOutputFormats".to_string(), 67108860.into());
    audio_format_100.insert("type".to_string(), 100.into());

    let mut audio_format_101 = plist::Dictionary::default();

    audio_format_101.insert("audioInputFormats".to_string(), 67108860.into());
    audio_format_101.insert("audioOutputFormats".to_string(), 67108860.into());
    audio_format_101.insert("type".to_string(), 101.into());

    let audio_formats = plist::Value::Array(vec![
        Dictionary(audio_format_100),
        Dictionary(audio_format_101),
    ]);

    let mut audio_latency_100 = plist::Dictionary::default();
    audio_latency_100.insert(
        "audioType".to_string(),
        Value::String("default".to_string()),
    );
    audio_latency_100.insert("inputLatencyMicros".to_string(), false.into());
    audio_latency_100.insert("type".to_string(), 100.into());

    let mut audio_latency_101 = plist::Dictionary::default();
    audio_latency_101.insert(
        "audioType".to_string(),
        Value::String("default".to_string()),
    );
    audio_latency_101.insert("inputLatencyMicros".to_string(), false.into());
    audio_latency_101.insert("type".to_string(), 101.into());

    let audio_latencies = Value::Array(vec![
        Dictionary(audio_latency_100),
        Dictionary(audio_latency_101),
    ]);

    let mut display = plist::Dictionary::default();
    display.insert("features".to_string(), 14.into());
    display.insert("height".to_string(), airplay_config.height.into());
    display.insert("heightPhysical".to_string(), false.into());
    display.insert("heightPixels".to_string(), airplay_config.height.into());
    display.insert("maxFPS".to_string(), airplay_config.fps.into());
    display.insert("overscanned".to_string(), false.into());
    display.insert("refreshRate".to_string(), 60.into());
    display.insert("rotation".to_string(), false.into());
    display.insert(
        "uuid".to_string(),
        Value::String("e5f7a68d-7b0f-4305-984b-974f677a150b".to_string()),
    );
    display.insert("width".to_string(), airplay_config.width.into());
    display.insert("widthPhysical".to_string(), false.into());
    display.insert("widthPixels".to_string(), airplay_config.width.into());

    let displays = Value::Array(vec![Value::Dictionary(display)]);

    let mut response = plist::Dictionary::default();
    response.insert("audioFormats".to_string(), audio_formats);
    response.insert("audioLatencies".to_string(), audio_latencies);
    response.insert("displays".to_string(), displays);
    response.insert("features".to_string(), 130367356919i64.into());
    response.insert("keepAliveSendStatsAsBody".to_string(), 1.into());
    response.insert("model".to_string(), Value::String("AppleTV3,2".to_string()));
    response.insert("name".to_string(), Value::String("Apple TV".to_string()));
    response.insert(
        "pi".to_string(),
        Value::String("b08f5a79-db29-4384-b456-a4784d9e6055".to_string()),
    );
    response.insert(
        "sourceVersion".to_string(),
        Value::String("220.68".to_string()),
    );
    response.insert("statusFlags".to_string(), 68.into());
    response.insert("vv".to_string(), 2.into());

    let b = bytes::BytesMut::default();
    let mut write = b.writer();

    plist::to_writer_binary(&mut write, &response).unwrap();
    write.into_inner().freeze()
}

pub fn prepare_setup_audio_response(data_port: u16, control_port: u16) -> Bytes {
    let mut data_stream = plist::Dictionary::default();
    data_stream.insert("dataPort".to_string(), data_port.into());
    data_stream.insert("type".to_string(), 96.into());
    data_stream.insert("controlPort".to_string(), control_port.into());
    let streams = Value::Array(vec![Value::Dictionary(data_stream)]);
    let mut response = plist::Dictionary::default();
    response.insert("streams".to_string(), streams);
    let mut writer = bytes::BytesMut::default().writer();
    plist::to_writer_binary(&mut writer, &response).unwrap();
    writer.into_inner().freeze()
}

pub fn prepare_setup_video_response(data_port: u16, event_port: u16, timing_port: u16) -> Bytes {
    let mut data_stream = plist::Dictionary::default();
    data_stream.insert("dataPort".to_string(), data_port.into());
    data_stream.insert("type".to_string(), 110.into());
    // data_stream.insert("controlPort".to_string(), control_port.into());
    let streams = Value::Array(vec![Value::Dictionary(data_stream)]);
    let mut response = plist::Dictionary::default();
    response.insert("streams".to_string(), streams);
    response.insert("eventPort".to_string(), event_port.into());
    response.insert("timingPort".to_string(), timing_port.into());
    let mut writer = bytes::BytesMut::default().writer();
    plist::to_writer_binary(&mut writer, &response).unwrap();
    writer.into_inner().freeze()
}

pub fn compute_m2(salt: &[u8], client_pk: &[u8], client_proof: &[u8]) -> Vec<u8> {
    let mut hasher = Sha1::new();
    hasher.update(salt);
    hasher.update([0, 0, 0, 0]);
    let k1 = hasher.finalize_reset();

    hasher.update(salt);
    hasher.update([0, 0, 0, 1]);
    let k2 = hasher.finalize_reset();

    hasher.update(client_pk);
    hasher.update(client_proof);
    hasher.update(k1);
    hasher.update(k2);

    hasher.finalize().to_vec()
}
