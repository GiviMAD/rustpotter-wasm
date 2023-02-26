mod utils;
use std::collections::HashMap;

#[cfg(feature = "build_models")]
use rustpotter::Wakeword as WakewordImpl;
use rustpotter::{
    Rustpotter as RustpotterImpl, RustpotterConfig, RustpotterDetection as RustpotterDetectionImpl,
    SampleFormat as RustpotterSampleFormat, ScoreMode as RustpotterScoreMode,
};
use wasm_bindgen::prelude::*;
// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[cfg(feature = "build_models")]
#[wasm_bindgen]
pub struct Wakeword {
    name: String,
    files: HashMap<String, Vec<u8>>,
}

#[cfg(feature = "build_models")]
#[wasm_bindgen]
#[allow(non_snake_case)]
/// Utility for creating wakeword models.
impl Wakeword {
    /// Creates a new instance.
    pub fn new(name: String) -> Self {
        Self {
            name: name,
            files: HashMap::new(),
        }
    }
    /// Add a wav file with name and data
    pub fn addFile(&mut self, name: String, buffer: Vec<u8>) {
        self.files.insert(name, buffer);
    }
    /// Remove a wav file by name
    pub fn removeFile(&mut self, name: &str) {
        self.files.remove(name);
    }
    /// Returns the model file bytes
    pub fn saveToBytes(&mut self) -> Result<Vec<u8>, String> {
        WakewordImpl::new_from_sample_buffers(self.name.clone(), None, None, self.files.clone())?
            .save_to_buffer()
    }
}
#[wasm_bindgen]
pub struct Rustpotter {
    detector: RustpotterImpl,
}
#[wasm_bindgen]
#[allow(non_snake_case)]
impl Rustpotter {
    /// Loads a wakeword from its model bytes.
    pub fn addWakeword(&mut self, bytes: Vec<u8>) -> Result<(), String> {
        self.detector.add_wakeword_from_buffer(&bytes)
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
        self.detector.process_int_buffer(buffer).map(|d| d.into())
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
        self.detector.process_short_buffer(buffer).map(|d| d.into())
    }
    /// Process f32 audio chunks.
    ///
    /// Asserts that the audio chunk length should match the return
    /// of the get_samples_per_frame method.
    ///
    /// Assumes sample rate match the configured for the detector.
    ///
    /// Assumes that detector bits_per_sample is 32.
    ///
    /// Assumes that detector sample_format is 'float'.
    pub fn processFloat32(&mut self, buffer: &[f32]) -> Option<RustpotterDetection> {
        self.detector.process_float_buffer(buffer).map(|d| d.into())
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
    /// Assumes that detector bits_per_sample is 8, 16, 32.
    ///
    pub fn processBuffer(&mut self, buffer: &[u8]) -> Option<RustpotterDetection> {
        self.detector.process_byte_buffer(buffer).map(|d| d.into())
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
impl From<RustpotterDetectionImpl> for RustpotterDetection {
    fn from(value: RustpotterDetectionImpl) -> Self {
        RustpotterDetection {
            name: value.name,
            avg_score: value.avg_score,
            score: value.score,
            scores: value.scores,
            counter: value.counter,
        }
    }
}
#[wasm_bindgen]
pub struct RustpotterDetection {
    /// Detected wakeword name.
    name: String,
    /// Detection score against the averaged template.
    avg_score: f32,
    /// Detection score.
    score: f32,
    /// Detection scores against each template.
    scores: HashMap<String, f32>,
    /// Partial detections counter.
    counter: usize,
}

#[wasm_bindgen]
#[allow(non_snake_case)]
impl RustpotterDetection {
    /// Get detection name
    pub fn getName(&self) -> String {
        self.name.clone()
    }
    /// Get detection score
    pub fn getScore(&self) -> f32 {
        self.score
    }
    /// Get detection avg score
    pub fn getAvgScore(&self) -> f32 {
        self.avg_score
    }
    /// Get score file names as a comma separated string
    pub fn getScoreNames(&self) -> String {
        self.scores
            .keys()
            .into_iter()
            .map(|v| v.to_string())
            .collect::<Vec<String>>()
            .join(",")
    }
    /// Get detection score by file name
    pub fn getScoreByName(&self, name: &str) -> Option<f32> {
        self.scores.get(name).map(|v| *v)
    }
    /// Get detection scores filenames
    pub fn getScoreValues(&self) -> Vec<f32> {
        self.scores.values().into_iter().map(|v| *v).collect()
    }
    /// Get partial detections counter
    pub fn getCounter(&self) -> usize {
        self.counter
    }
}

#[wasm_bindgen]
pub struct RustpotterBuilder {
    config: RustpotterConfig,
}
#[wasm_bindgen]
#[allow(non_snake_case)]
impl RustpotterBuilder {
    pub fn new() -> Self {
        #[cfg(feature = "console_error_panic_hook")]
        utils::set_panic_hook();
        #[cfg(feature = "log")]
        utils::set_logger();
        Self {
            config: RustpotterConfig::default(),
        }
    }
    /// Configures the detector threshold,
    /// is the min score (in range 0. to 1.) that some of
    /// the wakeword templates should obtain to trigger a detection.
    ///
    /// Defaults to 0.5, wakeword defined value takes prevalence if present.
    pub fn setThreshold(&mut self, value: f32) {
        self.config.detector.threshold = value;
    }
    /// Configures the detector averaged threshold,
    /// is the min score (in range 0. to 1.) that  
    /// the averaged wakeword template should obtain to allow
    /// to continue with the detection. This way it can prevent to
    /// run the comparison of the current frame against each of the wakeword templates.
    /// If set to 0. this functionality is disabled.
    ///
    /// Defaults to half of the configured threshold, wakeword defined value takes prevalence if present.
    pub fn setAveragedThreshold(&mut self, value: f32) {
        self.config.detector.avg_threshold = value;
    }
    /// Configures the required number of partial detections
    /// to consider a partial detection as a real detection.
    ///
    /// Defaults to 10
    pub fn setMinScores(&mut self, value: usize) {
        self.config.detector.min_scores = value;
    }
    /// Configures the score operation to unify the score values
    /// against each wakeword template.
    ///
    /// Defaults to max
    pub fn setScoreMode(&mut self, value: ScoreMode) {
        self.config.detector.score_mode = value.into();
    }
    /// Use a gain-normalization filter to dynamically change the input volume level.
    ///
    /// Defaults to false
    pub fn setGainNormalizerEnabled(&mut self, value: bool) {
        self.config.filters.gain_normalizer.enabled = value;
    }
    /// Use a gain-normalization filter to dynamically change the input volume level.
    ///
    /// Defaults to false
    pub fn setGainNormalizerRMSLevelRef(&mut self, value: Option<f32>) {
        self.config.filters.gain_normalizer.rms_level_ref = value;
    }
    /// Use a band-pass filter to attenuate frequencies
    /// out of the configured range.
    ///
    /// Defaults to false
    pub fn setBandPassEnabled(&mut self, value: bool) {
        self.config.filters.band_pass.enabled = value;
    }
    /// Configures the low-cutoff frequency for the band-pass
    /// filter.
    ///
    /// Defaults to 80.0
    pub fn setBandPassLowCutoff(&mut self, value: f32) {
        self.config.filters.band_pass.low_cutoff = value;
    }
    /// Configures the high-cutoff frequency for the band-pass
    /// filter.
    ///
    /// Defaults to 400.0
    pub fn setBandPassHighCutoff(&mut self, value: f32) {
        self.config.filters.band_pass.high_cutoff = value;
    }
    /// Configures the detector expected bit per sample for the audio chunks to process.
    ///
    /// When sample format is set to 'float' this is ignored as only 32 is supported.
    ///
    /// Defaults to 16; Allowed values: 8, 16, 24, 32.
    pub fn setBitsPerSample(&mut self, value: u16) {
        self.config.fmt.bits_per_sample = value;
    }
    /// Configures the detector expected sample rate for the audio chunks to process.
    ///
    /// Defaults to 48000
    pub fn setSampleRate(&mut self, value: usize) {
        self.config.fmt.sample_rate = value;
    }
    /// Configures the detector expected sample format for the audio chunks to process.
    ///
    /// Defaults to int
    pub fn setSampleFormat(&mut self, value: SampleFormat) {
        self.config.fmt.sample_format = value.into();
    }
    /// Configures the detector expected number of channels for the audio chunks to process.
    /// Rustpotter will only use data for first channel.
    ///
    /// Defaults to 1
    pub fn setChannels(&mut self, value: u16) {
        self.config.fmt.channels = value;
    }
    /// Configures the band-size for the comparator used to match the samples.
    ///
    /// Defaults to 6
    pub fn setComparatorBandSize(&mut self, value: u16) {
        self.config.detector.comparator_band_size = value;
    }
    /// Configures the reference for the comparator used to match the samples.
    ///
    /// Defaults to 0.22
    pub fn setComparatorRef(&mut self, value: f32) {
        self.config.detector.comparator_reference = value;
    }
    // /// Noise/silence ratio in the last second to consider noise detected.
    // ///
    // /// Defaults to 0.5.
    // ///
    // /// Only applies if noise mode is set.
    // pub fn setNoiseSensitivity(&mut self, value: f32) {
    //     self.builder.set_noise_sensitivity(value);
    // }
    /// construct the wakeword detector
    pub fn build(&self) -> Result<Rustpotter, String> {
        Ok(Rustpotter {
            detector: RustpotterImpl::new(&self.config)?,
        })
    }
}
#[wasm_bindgen]
#[allow(non_camel_case_types)]
/// Detection score mode.
pub enum ScoreMode {
    /// Use max value of the scores.
    max,
    /// Use average value of the scores.
    avg,
    /// Use median value of the scores.
    median,
}
impl From<ScoreMode> for RustpotterScoreMode {
    fn from(value: ScoreMode) -> Self {
        match value {
            ScoreMode::max => RustpotterScoreMode::Max,
            ScoreMode::median => RustpotterScoreMode::Median,
            ScoreMode::avg => RustpotterScoreMode::Average,
        }
    }
}
#[wasm_bindgen]
#[allow(non_camel_case_types)]
pub enum SampleFormat {
    int,
    float,
}
impl From<SampleFormat> for RustpotterSampleFormat {
    fn from(value: SampleFormat) -> Self {
        match value {
            SampleFormat::int => RustpotterSampleFormat::Int,
            SampleFormat::float => RustpotterSampleFormat::Float,
        }
    }
}
