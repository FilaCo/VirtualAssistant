use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    StreamConfig, SupportedStreamConfig,
};
use tokio::sync::mpsc;
use tokio_stream::Stream;
use va_lib::VAResult;

use self::vo::DeviceName;

pub fn get_cpal_mic_input_stream(
    dev_name: &DeviceName,
    conf: &SupportedStreamConfig,
) -> VAResult<impl Stream<Item = ()>> {
    let host = cpal::default_host(); // TODO: from config

    let mic_dev = if "default" == dev_name.as_str() {
        host.default_input_device().expect("wtf") // TODO: bubble up the error
    } else {
        todo!();
        // host.default_input_device().expect("wtf") // TODO: bubble up the error
    };

    let stream = mic_dev
        .build_input_stream(&(*conf).into(), data_callback, error_callback, timeout)
        .expect("wtf"); // TODO: bubble up the error

    stream.play();
    let (tx, rx) = mpsc::channel(1);

    todo!()
}

pub mod vo {
    pub struct DeviceName {
        val: String,
    }

    impl DeviceName {
        pub fn new(val: &str) -> Self {
            Self {
                val: val.to_string(),
            }
        }

        pub fn as_str(&self) -> &str {
            self.val.as_str()
        }
    }
}
