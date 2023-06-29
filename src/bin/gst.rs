use std::net::SocketAddr;
use std::str::FromStr;
use std::sync::Arc;

use airplay2_protocol::airplay::airplay_consumer::{AirPlayConsumer, ArcAirPlayConsumer};
use airplay2_protocol::airplay::AirPlayConfig;
use airplay2_protocol::airplay_bonjour::AirPlayBonjour;
use airplay2_protocol::control_handle::ControlHandle;
use airplay2_protocol::net::server::Server as MServer;
use airplay2_protocol::setup_log;
use crossbeam::channel::Sender;
use gst::Caps;
use gstreamer::{self as gst, prelude::*};
use gstreamer_app::{AppSrc, AppStreamType};
// use env_logger::Env;

struct VideoConsumer {
    tx: Sender<Vec<u8>>,
}

impl VideoConsumer {
    pub fn new() -> Self {
        let (tx, rx) = crossbeam::channel::unbounded();
        tokio::task::spawn_blocking(move || {
            gst::init().unwrap();
            let caps = Caps::from_str("audio/x-alac,mpegversion=(int)4,channels=(int)2,rate=(int)48000,stream-format=raw,codec_data=(buffer)00000024616c616300000000000001600010280a0e0200ff00000000000000000000ac44").unwrap();
            let pipeline = gst::Pipeline::default();

            let appsrc = AppSrc::builder()
                .is_live(true)
                .stream_type(AppStreamType::Stream)
                .caps(&caps)
                .format(gst::Format::Time)
                .build();

            let avdec_alac = gst::ElementFactory::make("avdec_alac").build().unwrap();
            let audioconvert = gst::ElementFactory::make("audioconvert").build().unwrap();
            let audioresample = gst::ElementFactory::make("audioresample").build().unwrap();
            let autoaudiosink = gst::ElementFactory::make("autoaudiosink")
                .property("sync", false)
                .build()
                .unwrap();

            pipeline
                .add_many(&[
                    appsrc.upcast_ref(),
                    &avdec_alac,
                    &audioconvert,
                    &audioresample,
                    &autoaudiosink,
                ])
                .unwrap();
            gst::Element::link_many(&[
                appsrc.upcast_ref(),
                &avdec_alac,
                &audioconvert,
                &audioresample,
                &autoaudiosink,
            ])
            .unwrap();

            // let pipeline_weak = pipeline.downgrade();
            appsrc.set_callbacks(
                gstreamer_app::AppSrcCallbacks::builder()
                    .need_data(move |appsrc, _size| {
                        // let pipeline = match pipeline_weak.upgrade() {
                        //     Some(pipeline) => pipeline,
                        //     None => return,
                        // };

                        if let Ok(buffer) = rx.recv() {
                            let buffer = gstreamer::Buffer::from_mut_slice(buffer);
                            let _ = appsrc.push_buffer(buffer);
                        }

                        // pipeline.set_state(gstreamer::State::Playing).unwrap();
                    })
                    .build(),
            );

            let bus = pipeline.bus().unwrap();
            pipeline
                .set_state(gst::State::Playing)
                .expect("Unable to set the pipeline to the `Playing` state");

            for msg in bus.iter_timed(gst::ClockTime::NONE) {
                use gst::MessageView;

                match msg.view() {
                    MessageView::Eos(..) => break,
                    MessageView::Error(err) => {
                        println!(
                            "Error from {:?}: {} ({:?})",
                            err.src().map(|s| s.path_string()),
                            err.error(),
                            err.debug()
                        );
                        break;
                    }
                    MessageView::StateChanged(state_changed) =>
                    // We are only interested in state-changed messages from playbin
                    {
                        if state_changed.src().map(|s| s == &pipeline).unwrap_or(false)
                            && state_changed.current() == gst::State::Playing
                        {
                            println!("StateChanged....");
                            // Generate a dot graph of the pipeline to GST_DEBUG_DUMP_DOT_DIR if defined
                            let pipeline: &gst::Element = pipeline.upcast_ref();
                            let bin_ref = pipeline.downcast_ref::<gst::Bin>().unwrap();
                            bin_ref.debug_to_dot_file(gst::DebugGraphDetails::all(), "PLAYING");
                        }
                    }

                    _ => (),
                }
            }

            pipeline
                .set_state(gst::State::Null)
                .expect("Unable to set the pipeline to the `Null` state");
        });

        Self { tx }
    }
}

impl AirPlayConsumer for VideoConsumer {
    fn on_video(&self, _bytes: Vec<u8>) {
        log::info!("on_video...");
    }

    fn on_video_format(
        &self,
        _video_stream_info: airplay2_protocol::airplay::lib::video_stream_info::VideoStreamInfo,
    ) {
        log::info!("on_video format...");
    }

    fn on_video_src_disconnect(&self) {
        log::info!("on_video disconnect...");
    }

    fn on_audio_format(
        &self,
        audio_stream_info: airplay2_protocol::airplay::lib::audio_stream_info::AudioStreamInfo,
    ) {
        log::info!(
            "on_audio_format... type = {:?}",
            audio_stream_info.compression_type
        );
    }

    fn on_audio(&self, bytes: Vec<u8>) {
        self.tx.send(bytes).unwrap();
        // log::info!("on_audio...");
    }

    fn on_audio_src_disconnect(&self) {}
}

#[tokio::main]
async fn main() -> tokio::io::Result<()> {
    setup_log();
    let port = 31927;
    let name = "RustAirplay";

    // pin码认证功能缺失...
    let _air = AirPlayBonjour::new(name, port, false);

    let addr: SocketAddr = ([0, 0, 0, 0], port).into();
    let airplay_config = AirPlayConfig {
        server_name: name.to_string(),
        width: 1920,
        height: 1080,
        fps: 30,
        port,
    };
    let video_consumer: ArcAirPlayConsumer = Arc::new(Box::new(VideoConsumer::new()));
    let mserver = MServer::bind(
        addr,
        ControlHandle::new(airplay_config, video_consumer.clone(), video_consumer),
    );
    mserver.run().await?;
    Ok(())
}
