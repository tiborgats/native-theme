//! The inputs the showcase's `SpeechState` runs on: a synthetic level
//! envelope and a recognizer that hears nothing. No microphone, no network.
//! gpui-component documents this route ("implement this trait to feed audio
//! from elsewhere", speech/recognizer.rs, AudioInput).

use std::{cell::Cell, rc::Rc, time::Duration};

use gpui::{App, Subscription};
use gpui_component::speech::{
    AudioFormat, AudioInput, AudioSink, RecognitionSession, SpeechError, SpeechRecognizer,
    SpeechSink,
};

/// How often the synthetic input delivers a block: on a timer, as a device
/// hands over its buffers, because the waveform paces its scroll by when
/// levels arrive (speech/level.rs, the playhead).
const BLOCK: Duration = Duration::from_millis(40);

/// The block amplitudes, cycled: demonstration data for the waveform, not a
/// style value.
const ENVELOPE: [f32; 12] = [
    0.05, 0.2, 0.45, 0.7, 0.5, 0.3, 0.6, 0.85, 0.4, 0.15, 0.3, 0.1,
];

/// Feeds a `SpeechState` the envelope. `blocks` counts the blocks delivered,
/// for the test that the timer stops with the session.
#[derive(Clone, Default)]
pub(crate) struct SyntheticAudio {
    pub(crate) blocks: Rc<Cell<usize>>,
}

impl AudioInput for SyntheticAudio {
    fn start(
        &self,
        format: AudioFormat,
        sink: AudioSink,
        cx: &mut App,
    ) -> Result<Subscription, SpeechError> {
        // Saturating: nothing here may overflow, whatever format a
        // recognizer asks for.
        let per_block = u64::from(format.sample_rate())
            .saturating_mul(u64::from(format.channels()))
            .saturating_mul(u64::try_from(BLOCK.as_millis()).unwrap_or(u64::MAX))
            / 1000;
        let per_block = usize::try_from(per_block).unwrap_or(usize::MAX);
        let blocks = self.blocks.clone();
        let task = cx.spawn(async move |cx| {
            for amplitude in ENVELOPE.iter().copied().cycle() {
                cx.background_executor().timer(BLOCK).await;
                let peak = (f32::from(i16::MAX) * amplitude) as i16;
                let samples: Vec<i16> = (0..per_block)
                    .map(|i| {
                        if i.is_multiple_of(2) {
                            peak
                        } else {
                            peak.saturating_neg()
                        }
                    })
                    .collect();
                blocks.set(blocks.get().saturating_add(1));
                cx.update(|cx| sink.push(samples, cx));
            }
        });
        Ok(Subscription::new(move || drop(task)))
    }
}

/// Opens a session that reports itself ready, discards the audio and
/// recognises nothing.
pub(crate) struct SilentRecognizer;

struct SilentSession(SpeechSink);

impl SpeechRecognizer for SilentRecognizer {
    fn start(
        &self,
        sink: SpeechSink,
        cx: &mut App,
    ) -> Result<Box<dyn RecognitionSession>, SpeechError> {
        sink.ready(cx);
        Ok(Box::new(SilentSession(sink)))
    }
}

impl RecognitionSession for SilentSession {
    fn push_audio(&mut self, _: &[i16], _: &mut App) {}

    fn finish(&mut self, cx: &mut App) {
        self.0.finish(cx);
    }
}
