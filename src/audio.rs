use reqwest::Error;

pub struct Audio {
    url: String,
}
impl Audio {
    pub fn new(url: &str) -> Self {
        Self {
            url: url.to_string(),
        }
    }
    pub async fn get_bytes(&self) -> Result<Vec<u8>, Error> {
        let response = reqwest::get(self.url.clone()).await?;
        let bytes = response.bytes().await?;
        Ok(bytes.to_vec())
    }
    pub async fn read_audio(&self) -> Result<Vec<f32>, String> {
        let bytes = self.get_bytes().await.map_err(|x| x.to_string())?;
        let mut reader = hound::WavReader::new(&bytes[..]).map_err(|x| x.to_string())?;
        let spec = reader.spec();
        let channels = spec.channels;
        let sample_rate = spec.sample_rate;

        let mut audio = whisper_rs::convert_integer_to_float_audio(
            &reader
                .samples::<i16>()
                .map(|s| s.expect("invalid sample"))
                .collect::<Vec<_>>(),
        );

        if channels == 2 {
            audio = whisper_rs::convert_stereo_to_mono_audio(&audio)?;
        } else if channels != 1 {
            Err(">2 channels unsupported")?;
        }

        if sample_rate != 16000 {
            Err("sample rate must be 16KHz")?;
        }
        Ok(audio)
    }
}
