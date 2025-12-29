use nih_plug::prelude::*;
use std::sync::Arc;

// Modules
mod dsp;
mod editor;
mod cloud;

use dsp::Saturation;
use editor::AntigravityEditor;

pub struct AntigravityHybrid {
    params: Arc<AntigravityParams>,
    saturation: Saturation,
    cloud_client: cloud::CloudClient,
}

#[derive(Params)]
pub struct AntigravityParams {
    #[id = "drive"]
    pub drive: FloatParam,

    #[id = "output"]
    pub output_gain: FloatParam,

    #[id = "oversample"]
    pub oversample_mode: EnumParam<OversampleMode>,

    #[id = "gpu_enabled"]
    pub gpu_enabled: BoolParam,
}

#[derive(Enum, PartialEq, Clone, Copy)]
pub enum OversampleMode {
    #[name = "Off"]
    Off,
    #[name = "2x"]
    X2,
    #[name = "4x"]
    X4,
}

impl Default for AntigravityHybrid {
    fn default() -> Self {
        Self {
            params: Arc::new(AntigravityParams::default()),
            saturation: Saturation::default(),
            cloud_client: cloud::CloudClient::new(),
        }
    }
}

impl Default for AntigravityParams {
    fn default() -> Self {
        Self {
            drive: FloatParam::new(
                "Drive",
                0.0,
                FloatRange::Linear { min: 0.0, max: 100.0 },
            ),
            output_gain: FloatParam::new(
                "Output Gain",
                0.0,
                FloatRange::Linear { min: -12.0, max: 12.0 },
            ).with_unit("dB"),
            oversample_mode: EnumParam::new("Oversampling", OversampleMode::Off),
            gpu_enabled: BoolParam::new("GPU Acceleration", true),
        }
    }
}

impl Plugin for AntigravityHybrid {
    const NAME: &'static str = "Antigravity Hybrid";
    const VENDOR: &'static str = "Antigravity AI";
    const URL: &'static str = "https://antigravity.ai";
    const EMAIL: &'static str = "support@antigravity.ai";
    const VERSION: &'static str = "0.1.0";
    
    // IO Layout: Stereo In, Stereo Out
    const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[
        AudioIOLayout {
            main_input_channels: NonZeroU32::new(2),
            main_output_channels: NonZeroU32::new(2),
            ..AudioIOLayout::const_default()
        }
    ];

    const SAMPLE_ACCURATE_AUTOMATION: bool = true;

    type SysExMessage = ();
    type BackgroundTask = ();

    fn params(&self) -> Arc<dyn Params> {
        self.params.clone()
    }

    fn initialize(
        &mut self,
        _audio_io_layout: &AudioIOLayout,
        buffer_config: &BufferConfig,
        _context: &mut impl InitContext<Self>,
    ) -> bool {
        self.saturation.set_sample_rate(buffer_config.sample_rate);
        // Initialize cloud/verification in background?
        true
    }

    fn process(
        &mut self,
        buffer: &mut Buffer,
        _aux: &mut AuxiliaryBuffers,
        _context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        // Collect parameter values
        let drive = self.params.drive.value();
        let output_gain_db = self.params.output_gain.value();
        // let oversample = self.params.oversample_mode.value(); // Needed for oversampling logic container

        // Process audio blocks using the DSP module
        self.saturation.process_block(buffer, drive, output_gain_db);

        ProcessStatus::Normal
    }

    fn editor(&mut self, _async_executor: AsyncExecutor<Self>) -> Option<Box<dyn Editor>> {
        editor::create_antigravity_editor(self.params.clone())
    }
}

impl Vst3Plugin for AntigravityHybrid {
    const VST3_CLASS_ID: [u8; 16] = *b"AntigrvtHybridPx"; // Random UUID-like
    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] = &[
        Vst3SubCategory::Fx,
        Vst3SubCategory::Distortion,
    ];
}

impl ClapPlugin for AntigravityHybrid {
    const CLAP_ID: &'static str = "com.antigravity.hybrid";
    const CLAP_DESCRIPTION: Option<&'static str> = Some("Hybrid AI Plugin");
}

nih_export_clap!(AntigravityHybrid);
nih_export_vst3!(AntigravityHybrid);
pub mod ui;
