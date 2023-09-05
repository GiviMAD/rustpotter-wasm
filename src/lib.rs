#[cfg(feature = "build_refs")]
use std::collections::HashMap;

use rustpotter::{
    Rustpotter as RustpotterImpl, RustpotterConfig as RustpotterConfigImpl,
    RustpotterDetection as RustpotterDetectionImpl, SampleFormat as RustpotterSampleFormat,
    ScoreMode as RustpotterScoreMode, VADMode as RustpotterVADMode,
};
#[cfg(feature = "build_refs")]
use rustpotter::{WakewordRef, WakewordRefBuildFromBuffers, WakewordSave};
use wasm_bindgen::prelude::*;
// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
#[cfg(feature = "build_refs")]
#[wasm_bindgen]
pub struct WakewordRefCreator {
    name: String,
    files: HashMap<String, Vec<u8>>,
}
#[cfg(feature = "build_refs")]
#[wasm_bindgen]
#[allow(non_snake_case)]
/// Utility for creating wakeword references.
impl WakewordRefCreator {
    /// Creates a wakeword reference.
    pub fn new(name: String) -> Self {
        Self {
            name: name,
            files: HashMap::new(),
        }
    }
    /// Add a wav file with name and data.
    pub fn addFile(&mut self, name: String, buffer: Vec<u8>) {
        self.files.insert(name, buffer);
    }
    /// Remove a wav file by name.
    pub fn removeFile(&mut self, name: &str) {
        self.files.remove(name);
    }
    /// Returns the model file bytes.
    pub fn saveToBytes(&mut self) -> Result<Vec<u8>, String> {
        WakewordRef::new_from_sample_buffers(self.name.clone(), None, None, self.files.clone(), 16)?
            .save_to_buffer()
    }
}
#[wasm_bindgen]
pub struct Rustpotter {
    lib: RustpotterImpl,
}
#[wasm_bindgen]
#[allow(non_snake_case)]
impl Rustpotter {
    /// Creates a rustpotter instance.
    pub fn new(config: &RustpotterConfig) -> Result<Rustpotter, String> {
        Ok(Rustpotter {
            lib: RustpotterImpl::new(&config.lib_config)?,
        })
    }
    /// Process int 32 bit audio chunks.
    ///
    /// The buffer length should match the return of the getSamplesPerFrame method.
    pub fn processI32(&mut self, buffer: &[i32]) -> Option<RustpotterDetection> {
        self.lib
            .process_samples::<i32>(buffer.into())
            .map(|d| d.into())
    }
    /// Process int 16 bit audio chunks.
    ///
    /// The buffer length should match the return of the getSamplesPerFrame method.
    pub fn processI16(&mut self, buffer: &[i16]) -> Option<RustpotterDetection> {
        self.lib
            .process_samples::<i16>(buffer.into())
            .map(|d| d.into())
    }
    /// Process float 32 bit audio chunks.
    ///
    /// The buffer length should match the return of the getSamplesPerFrame method.
    pub fn processF32(&mut self, buffer: &[f32]) -> Option<RustpotterDetection> {
        self.lib
            .process_samples::<f32>(buffer.into())
            .map(|d| d.into())
    }
    /// Process byte buffer.
    ///
    /// The buffer length should match the return of the getByteFrameSize method.
    pub fn processBytes(&mut self, buffer: &[u8]) -> Option<RustpotterDetection> {
        self.lib.process_bytes(buffer).map(|d| d.into())
    }
    /// Loads a wakeword from its model bytes.
    pub fn addWakeword(&mut self, key: &str, bytes: Vec<u8>) -> Result<(), String> {
        self.lib.add_wakeword_from_buffer(key, &bytes)
    }
    /// Removes a wakeword by key.
    pub fn removeWakeword(&mut self, key: &str) -> bool {
        self.lib.remove_wakeword(key)
    }
    /// Removes all wakewords.
    pub fn removeWakewords(&mut self) -> bool {
        self.lib.remove_wakewords()
    }
    /// Returns the required number of samples.
    pub fn getSamplesPerFrame(&self) -> usize {
        self.lib.get_samples_per_frame()
    }
    /// Returns the required number of bytes.
    pub fn getBytesPerFrame(&self) -> usize {
        self.lib.get_bytes_per_frame()
    }
    /// Updates detector and audio filter options
    pub fn updateConfig(&mut self, config: &RustpotterConfig) {
        self.lib.update_config(&config.lib_config);
    }
    /// Reset internal state.
    pub fn reset(&mut self) {
        self.lib.reset()
    }
}
impl From<RustpotterDetectionImpl> for RustpotterDetection {
    fn from(detection: RustpotterDetectionImpl) -> Self {
        RustpotterDetection { detection }
    }
}
#[wasm_bindgen]
pub struct RustpotterDetection {
    detection: RustpotterDetectionImpl,
}
#[wasm_bindgen]
#[allow(non_snake_case)]
impl RustpotterDetection {
    /// Get detection name
    pub fn getName(&self) -> String {
        self.detection.name.clone()
    }
    /// Get detection score
    pub fn getScore(&self) -> f32 {
        self.detection.score
    }
    /// Get detection avg score
    pub fn getAvgScore(&self) -> f32 {
        self.detection.avg_score
    }
    /// Get detection score by file name
    pub fn getScoreByName(&self, name: &str) -> Option<f32> {
        self.detection.scores.get(name).map(|v| *v)
    }
    /// Get score file names as a || separated string
    pub fn getScoreNames(&self) -> String {
        self.detection
            .scores
            .keys()
            .into_iter()
            .map(|v| v.to_string())
            .collect::<Vec<String>>()
            .join("||")
    }
    /// Get detection scores
    pub fn getScores(&self) -> Vec<f32> {
        self.detection
            .scores
            .values()
            .into_iter()
            .map(|v| *v)
            .collect()
    }
    /// Get partial detections counter
    pub fn getCounter(&self) -> usize {
        self.detection.counter
    }
    /// Get gain applied
    pub fn getGain(&self) -> f32 {
        self.detection.gain
    }
}
#[wasm_bindgen]
pub struct RustpotterConfig {
    lib_config: RustpotterConfigImpl,
}
#[wasm_bindgen]
#[allow(non_snake_case)]
impl RustpotterConfig {
    /// Creates a rustpotter config instance.
    pub fn new() -> Self {
        #[cfg(feature = "console_error_panic_hook")]
        utils::set_panic_hook();
        #[cfg(feature = "log")]
        utils::set_logger();
        Self {
            lib_config: RustpotterConfigImpl::default(),
        }
    }
    /// Configures the detector threshold,
    /// is the min score (in range 0. to 1.) that some of
    /// the wakeword templates should obtain to trigger a detection.
    ///
    /// Defaults to 0.5, wakeword defined value takes prevalence if present.
    pub fn setThreshold(&mut self, value: f32) {
        self.lib_config.detector.threshold = value;
    }
    /// Configures the detector averaged threshold,
    ///
    /// If set to 0. this functionality is disabled.
    ///
    /// Wakeword defined value takes prevalence if present.
    pub fn setAveragedThreshold(&mut self, value: f32) {
        self.lib_config.detector.avg_threshold = value;
    }
    /// Configures the required number of partial detections
    /// to consider a partial detection as a real detection.
    ///
    /// Defaults to 5
    pub fn setMinScores(&mut self, value: usize) {
        self.lib_config.detector.min_scores = value;
    }
    /// Configures a basic vad detector to avoid some execution.
    ///
    /// Disabled by default
    pub fn setVADMode(&mut self, value: Option<VADMode>) {
        self.lib_config.detector.vad_mode = value.map(|v| v.into());
    }
    /// Configures the operation used to unify the score against each record when using wakeword references.
    /// Doesn't apply to trained wakewords.
    ///
    /// Defaults to max
    pub fn setScoreMode(&mut self, value: ScoreMode) {
        self.lib_config.detector.score_mode = value.into();
    }
    /// Configures the comparator the band size.
    /// Doesn't apply to trained wakewords.
    ///
    /// Defaults to 5
    pub fn setBandSize(&mut self, value: u16) {
        self.lib_config.detector.band_size = value;
    }
    /// Value used to express the score as a percent in range 0 - 1.
    ///
    /// Defaults to 0.22
    pub fn setScoreRef(&mut self, value: f32) {
        self.lib_config.detector.score_ref = value;
    }
    /// Emit detection on min scores.
    ///
    /// Defaults to false
    pub fn setEager(&mut self, value: bool) {
        self.lib_config.detector.eager = value;
    }
    /// Use a gain-normalization filter to dynamically change the input volume level.
    ///
    /// Defaults to false
    pub fn setGainNormalizerEnabled(&mut self, value: bool) {
        self.lib_config.filters.gain_normalizer.enabled = value;
    }
    /// Set the rms level reference used by the gain-normalizer filter.
    /// If null the approximated wakewords rms level is used.
    ///
    /// Defaults to null
    pub fn setGainRef(&mut self, value: Option<f32>) {
        self.lib_config.filters.gain_normalizer.gain_ref = value;
    }
    /// Sets the min gain applied by the gain-normalizer filter.
    ///
    /// Defaults to 0.1
    pub fn setMinGain(&mut self, value: f32) {
        self.lib_config.filters.gain_normalizer.min_gain = value;
    }
    /// Sets the max gain applied by the gain-normalizer filter.
    ///
    /// Defaults to 1.0
    pub fn setMaxGain(&mut self, value: f32) {
        self.lib_config.filters.gain_normalizer.max_gain = value;
    }
    /// Use a band-pass filter to attenuate frequencies
    /// out of the configured range.
    ///
    /// Defaults to false
    pub fn setBandPassEnabled(&mut self, value: bool) {
        self.lib_config.filters.band_pass.enabled = value;
    }
    /// Configures the low-cutoff frequency for the band-pass
    /// filter.
    ///
    /// Defaults to 80.0
    pub fn setBandPassLowCutoff(&mut self, value: f32) {
        self.lib_config.filters.band_pass.low_cutoff = value;
    }
    /// Configures the high-cutoff frequency for the band-pass
    /// filter.
    ///
    /// Defaults to 400.0
    pub fn setBandPassHighCutoff(&mut self, value: f32) {
        self.lib_config.filters.band_pass.high_cutoff = value;
    }
    /// Configures the detector expected sample rate for the audio chunks to process.
    ///
    /// Defaults to 16000
    pub fn setSampleRate(&mut self, value: usize) {
        self.lib_config.fmt.sample_rate = value;
    }
    /// Configures the detector expected sample format for the audio chunks to process.
    ///
    /// Defaults to F32
    pub fn setSampleFormat(&mut self, value: SampleFormat) {
        self.lib_config.fmt.sample_format = value.into();
    }
    /// Configures the detector expected number of channels for the audio chunks to process.
    /// Rustpotter will only use data from the first channel.
    ///
    /// Defaults to 1
    pub fn setChannels(&mut self, value: u16) {
        self.lib_config.fmt.channels = value;
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
    /// Use percentile 25 of the scores.
    p25,
    /// Use percentile 50 of the scores.
    p50,
    /// Use percentile 75 of the scores.
    p75,
    /// Use percentile 80 of the scores.
    p80,
    /// Use percentile 90 of the scores.
    p90,
    /// Use percentile 95 of the scores.
    p95,
}
impl From<ScoreMode> for RustpotterScoreMode {
    fn from(value: ScoreMode) -> Self {
        match value {
            ScoreMode::max => RustpotterScoreMode::Max,
            ScoreMode::median => RustpotterScoreMode::Median,
            ScoreMode::avg => RustpotterScoreMode::Average,
            ScoreMode::p25 => RustpotterScoreMode::P25,
            ScoreMode::p50 => RustpotterScoreMode::P50,
            ScoreMode::p75 => RustpotterScoreMode::P75,
            ScoreMode::p80 => RustpotterScoreMode::P80,
            ScoreMode::p90 => RustpotterScoreMode::P90,
            ScoreMode::p95 => RustpotterScoreMode::P95,
        }
    }
}
#[wasm_bindgen]
#[allow(non_camel_case_types)]
pub enum VADMode {
    easy,
    medium,
    hard,
}
impl From<VADMode> for RustpotterVADMode {
    fn from(value: VADMode) -> Self {
        match value {
            VADMode::easy => RustpotterVADMode::Easy,
            VADMode::medium => RustpotterVADMode::Medium,
            VADMode::hard => RustpotterVADMode::Hard,
        }
    }
}
#[wasm_bindgen]
#[allow(non_camel_case_types)]
pub enum SampleFormat {
    i8,
    i16,
    i32,
    f32,
}
impl From<SampleFormat> for RustpotterSampleFormat {
    fn from(value: SampleFormat) -> Self {
        match value {
            SampleFormat::i8 => RustpotterSampleFormat::I8,
            SampleFormat::i16 => RustpotterSampleFormat::I16,
            SampleFormat::i32 => RustpotterSampleFormat::I32,
            SampleFormat::f32 => RustpotterSampleFormat::F32,
        }
    }
}
