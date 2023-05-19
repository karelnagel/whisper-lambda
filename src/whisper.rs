use crate::audio::Audio;
use lambda_http::aws_lambda_events::serde::Serialize;
use whisper_rs::{FullParams, SamplingStrategy, WhisperContext};

#[derive(Debug, Clone, Serialize)]
pub struct Segment {
    start: i64,
    end: i64,
    text: String,
}

pub async fn transcribe(
    audio: Audio,
    model: &[u8],
    translate: bool,
    max_len: i32,
) -> Result<Vec<Segment>, String> {
    let ctx = WhisperContext::new_from_buffer(model).map_err(|x| x.to_string())?;
    let mut state = ctx.create_state().map_err(|x| x.to_string())?;

    let mut params = FullParams::new(SamplingStrategy::Greedy { best_of: 1 });
    params.set_print_special(false);
    params.set_print_progress(false);
    params.set_print_realtime(false);
    params.set_print_timestamps(false);
    params.set_translate(translate);
    params.set_max_len(max_len);

    let audio_data = audio.read_audio().await?;

    state
        .full(params, &audio_data[..])
        .map_err(|x| x.to_string())?;

    let num_segments = state.full_n_segments().map_err(|x| x.to_string())?;
    let mut text = vec![];
    for i in 0..num_segments {
        let segment = state.full_get_segment_text(i).map_err(|x| x.to_string())?;
        let start = state.full_get_segment_t0(i).map_err(|x| x.to_string())?;
        let end = state.full_get_segment_t1(i).map_err(|x| x.to_string())?;
        text.push(Segment {
            start,
            end,
            text: segment,
        })
    }
    Ok(text)
}
