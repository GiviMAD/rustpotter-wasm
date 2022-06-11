use rustpotter::{
    DetectedWakeword, NoiseDetectionMode as RustpotterNoiseDetectionMode,
    SampleFormat as RustpotterSampleFormat, WakewordDetector, WakewordDetectorBuilder,
};
use wasm_bindgen::prelude::*;
mod utils;
// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct RustpotterJS {
    detector: WakewordDetector,
}
#[wasm_bindgen]
#[allow(non_snake_case)]
impl RustpotterJS {
    #[cfg(feature = "build-model")]
    /// Loads a wakeword from its model path.
    pub fn add_wakeword_with_wav_buffers(
        &mut self,
        name: &str,
        sample1Name: Option<String>,
        sample1: Option<Vec<u8>>,
        sample2Name: Option<String>,
        sample2: Option<Vec<u8>>,
        sample3Name: Option<String>,
        sample3: Option<Vec<u8>>,
        sample4Name: Option<String>,
        sample4: Option<Vec<u8>>,
        sample5Name: Option<String>,
        sample5: Option<Vec<u8>>,
        sample6Name: Option<String>,
        sample6: Option<Vec<u8>>,
    ) -> Result<(), String> {
        let mut samples: Vec<(String, Vec<u8>)> = Vec::new();
        fn tryAdd(
            collection: &mut Vec<(String, Vec<u8>)>,
            name_option: Option<String>,
            value_option: Option<Vec<u8>>,
        ) {
            if name_option.is_some() && value_option.is_some() {
                collection.push((name_option.unwrap(), value_option.unwrap()));
            };
        }
        tryAdd(&mut samples, sample1Name, sample1);
        tryAdd(&mut samples, sample2Name, sample2);
        tryAdd(&mut samples, sample3Name, sample3);
        tryAdd(&mut samples, sample4Name, sample4);
        tryAdd(&mut samples, sample5Name, sample5);
        tryAdd(&mut samples, sample6Name, sample6);
        if samples.is_empty() {
            self.detector
                .add_wakeword_with_wav_buffers(&name.to_string(), true, None, None, samples)
                .map_err(|err| err.to_string())
        } else {
            Err("No samples provided".to_owned())
        }
    }
    /// Loads a wakeword from its model bytes.
    pub fn addWakewordModelBytes(&mut self, data: Vec<u8>) -> Result<String, String> {
        self.detector
            .add_wakeword_from_model_bytes(data, true)
            .map_err(|err| err.to_string())
    }
    /// Process i32 audio chunks.
    ///
    /// Asserts that the audio chunk length should match the return
    /// of the get_samples_per_frame method.
    ///
    /// Assumes sample rate match the configured for the detector.
    ///
    /// Asserts that detector bits_per_sample is one of: 8, 16, 24, 32.
    ///
    /// Asserts that detector sample_format is 'int'.
    pub fn processInt32(&mut self, buffer: &[i32]) -> Option<RustpotterDetection> {
        self.detector.process_i32(buffer).map(transform_detection)
    }
    /// Process i16 audio chunks.
    ///
    /// Asserts that the audio chunk length should match the return
    /// of the get_samples_per_frame method.
    ///
    /// Assumes sample rate match the configured for the detector.
    ///
    /// Asserts that detector bits_per_sample is one of: 8, 16.
    ///
    /// Asserts that detector sample_format is 'int'.
    pub fn processInt16(&mut self, buffer: &[i16]) -> Option<RustpotterDetection> {
        self.detector.process_i16(buffer).map(transform_detection)
    }
    /// Process i8 audio chunks.
    ///
    /// Asserts that the audio chunk length should match the return
    /// of the get_samples_per_frame method.
    ///
    /// Assumes sample rate match the configured for the detector.
    ///
    /// Asserts that detector bits_per_sample is 8.
    ///
    /// Asserts that detector sample_format is 'int'.
    pub fn processInt8(&mut self, buffer: &[i8]) -> Option<RustpotterDetection> {
        self.detector.process_i8(buffer).map(transform_detection)
    }
    /// Process f32 audio chunks.
    ///
    /// Asserts that the audio chunk length should match the return
    /// of the get_samples_per_frame method.
    ///
    /// Assumes sample rate match the configured for the detector.
    ///
    /// Asserts that detector bits_per_sample is 32.
    ///
    /// Asserts that detector sample_format is 'float'.
    pub fn processFloat32(&mut self, buffer: &[f32]) -> Option<RustpotterDetection> {
        self.detector.process_f32(buffer).map(transform_detection)
    }
    /// Process bytes buffer.
    ///
    /// Asserts that the buffer length should match the return
    /// of the get_bytes_per_frame method.
    ///
    /// Assumes sample rate match the configured for the detector.
    ///
    /// Assumes buffer endianness matches the configured for the detector.
    ///
    /// Asserts that detector bits_per_sample is 8, 16, 24 or 32 (float format only allows 32).
    ///
    pub fn processBuffer(&mut self, buffer: &[u8]) -> Option<RustpotterDetection> {
        self.detector
            .process_buffer(buffer)
            .map(transform_detection)
    }
    /// Returns the desired chunk size.
    pub fn getFrameSize(&self) -> usize {
        self.detector.get_samples_per_frame()
    }
    /// Returns the desired buffer size for the processBuffer method.
    pub fn getByteFrameSize(&self) -> usize {
        self.detector.get_bytes_per_frame()
    }
}
fn transform_detection(detection: DetectedWakeword) -> RustpotterDetection {
    RustpotterDetection {
        name: detection.wakeword,
        score: detection.score,
    }
}
#[wasm_bindgen]
pub struct RustpotterDetection {
    name: String,
    score: f32,
}
#[wasm_bindgen]
#[allow(non_snake_case)]
impl RustpotterDetection {
    /// Get detected wakeword name
    pub fn getName(&self) -> String {
        self.name.clone()
    }
    /// Get detected wakeword score
    pub fn getScore(&self) -> f32 {
        self.score
    }
}

#[wasm_bindgen]
pub struct RustpotterJSBuilder {
    builder: WakewordDetectorBuilder,
}
#[wasm_bindgen]
#[allow(non_snake_case)]
impl RustpotterJSBuilder {
    pub fn new() -> Self {
        #[cfg(feature = "console_error_panic_hook")]
        utils::set_panic_hook();
        #[cfg(feature = "log")]
        utils::set_logger();
        Self {
            builder: WakewordDetectorBuilder::new(),
        }
    }
    /// Configures the detector threshold,
    /// is the min score (in range 0. to 1.) that some of
    /// the wakeword templates should obtain to trigger a detection.
    ///
    /// Defaults to 0.5, wakeword defined value takes prevalence if present.
    pub fn setThreshold(&mut self, value: f32) {
        self.builder.set_threshold(value);
    }
    /// Configures the detector threshold,
    /// is the min score (in range 0. to 1.) that  
    /// the averaged wakeword template should obtain to allow
    /// to continue with the detection. This way it can prevent to
    /// run the comparison of the current frame against each of the wakeword templates.
    /// If set to 0. this functionality is disabled.
    ///
    /// Defaults to half of the configured threshold, wakeword defined value takes prevalence if present.
    pub fn setAveragedThreshold(&mut self, value: f32) {
        self.builder.set_averaged_threshold(value);
    }
    /// Configures the detector expected bit per sample for the audio chunks to process.
    ///
    /// When sample format is set to 'float' this is ignored as only 32 is supported.
    ///
    /// Defaults to 16; Allowed values: 8, 16, 24, 32.
    pub fn setBitsPerSample(&mut self, value: u16) {
        self.builder.set_bits_per_sample(value);
    }
    /// Configures the detector expected sample rate for the audio chunks to process.
    ///
    /// Defaults to 48000
    pub fn setSampleRate(&mut self, value: usize) {
        self.builder.set_sample_rate(value);
    }
    /// Configures the detector expected sample format for the audio chunks to process.
    ///
    /// Defaults to int
    pub fn setSampleFormat(&mut self, value: SampleFormat) {
        self.builder.set_sample_format(transform_format(value));
    }
    /// Configures the detector expected number of channels for the audio chunks to process.
    /// Rustpotter will only use data for first channel.
    ///
    /// Defaults to 1
    pub fn setChannels(&mut self, value: u16) {
        self.builder.set_channels(value);
    }
    /// Configures the band-size for the comparator used to match the samples.
    ///
    /// Defaults to 6
    pub fn setComparatorBandSize(&mut self, value: usize) {
        self.builder.set_comparator_band_size(value);
    }
    /// Configures the reference for the comparator used to match the samples.
    ///
    /// Defaults to 0.22
    pub fn setComparatorRef(&mut self, value: f32) {
        self.builder.set_comparator_ref(value);
    }
    /// Enables eager mode.
    /// Terminate the detection as son as one result is above the score,
    /// instead of wait to see if the next frame has a higher score.
    ///
    /// Recommended for real usage.
    ///
    /// Defaults to false.
    pub fn setEagerMode(&mut self, value: bool) {
        self.builder.set_eager_mode(value);
    }
    /// Unless enabled the comparison against multiple wakewords run
    /// in separate threads.
    ///
    /// Defaults to false.
    ///
    /// Only applies when more than a wakeword is loaded.
    pub fn setSingleThread(&mut self, value: bool) {
        self.builder.set_single_thread(value);
    }
    /// Noise/silence ratio in the last second to consider voice detected.
    ///
    /// Defaults to 0.5.
    ///
    /// Only applies if noise mode is set.
    pub fn setNoiseSensitivity(&mut self, value: f32) {
        self.builder.set_noise_sensitivity(value);
    }
    /// Use build-in noise detection to reduce computation on absence of noise.
    ///
    /// Configures how difficult is to considering a frame as noise (the required noise lever).
    ///
    /// Unless specified the noise detection is disabled.
    pub fn setNoiseMode(&mut self, value: NoiseDetectionMode) {
        self.builder.set_noise_mode(transform_noise_mode(value));
    }
    /// construct the wakeword detector
    pub fn build(&self) -> RustpotterJS {
        RustpotterJS {
            detector: self.builder.build(),
        }
    }
}
#[wasm_bindgen]
#[allow(non_camel_case_types)]
pub enum NoiseDetectionMode {
    easiest,
    easy,
    normal,
    hard,
    hardest,
}
fn transform_noise_mode(mode: NoiseDetectionMode) -> RustpotterNoiseDetectionMode {
    match mode {
        NoiseDetectionMode::easiest => RustpotterNoiseDetectionMode::Easiest,
        NoiseDetectionMode::easy => RustpotterNoiseDetectionMode::Easy,
        NoiseDetectionMode::normal => RustpotterNoiseDetectionMode::Normal,
        NoiseDetectionMode::hard => RustpotterNoiseDetectionMode::Hard,
        NoiseDetectionMode::hardest => RustpotterNoiseDetectionMode::Hardest,
    }
}
#[wasm_bindgen]
#[allow(non_camel_case_types)]
pub enum SampleFormat {
    int,
    float,
}

fn transform_format(format: SampleFormat) -> RustpotterSampleFormat {
    match format {
        SampleFormat::int => RustpotterSampleFormat::Int,
        SampleFormat::float => RustpotterSampleFormat::Float,
    }
}
