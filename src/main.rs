use lambda_http::{
    aws_lambda_events::serde_json, run, service_fn, Body, Error, Request, RequestExt, Response,
};
use whisper::transcribe;

use crate::audio::Audio;
mod audio;
mod whisper;

/// If model url specified then uses that model, otherwise uses the binary included in the binary, if
/// the binary is not included then it will error.
async fn get_model(url: Option<&str>) -> Result<Vec<u8>, Error> {
    match url {
        Some(url) => {
            println!("Downloading model from: {}", url);
            let response = reqwest::get(url).await?;
            let bytes = response.bytes().await?;
            Ok(bytes.to_vec())
        }
        None => {
            #[cfg(feature = "base")]
            {
                Ok(include_bytes!("../ggml-base-q5_0.bin").to_vec())
            }
            #[cfg(feature = "tiny")]
            {
                Ok(include_bytes!("../ggml-tiny-q5_0.bin").to_vec())
            }
            #[cfg(not(any(feature = "base", feature = "tiny")))]
            {
                Err("You must specify a model url, if binary not included!")?
            }
        }
    }
}

async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
    let query = event.query_string_parameters();
    let audio = match query.first("audio") {
        Some(audio) => audio,
        None => {
            return Ok(Response::builder()
                .status(400)
                .body(Body::from("No audio url!"))?)
        }
    };
    let model = query.first("model");
    let translate = query
        .first("translate")
        .unwrap_or("false")
        .parse::<bool>()
        .unwrap_or(false);
    let max_len = query
        .first("max_len")
        .unwrap_or("0")
        .parse::<i32>()
        .unwrap_or(0);

    let audio = Audio::new(audio);
    let model = match get_model(model).await {
        Ok(model) => model,
        Err(e) => {
            return Ok(Response::builder()
                .status(400)
                .body(Body::from(format!("Error getting model: {}", e)))?)
        }
    };

    let text = match transcribe(audio, &model[..], translate, max_len).await {
        Ok(text) => text,
        Err(e) => {
            return Ok(Response::builder()
                .status(400)
                .body(Body::from(format!("Error transcribing: {}", e)))?)
        }
    };

    let resp = Response::builder()
        .status(200)
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_string(&text).unwrap()))
        .map_err(Box::new)?;
    Ok(resp)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_target(false)
        .without_time()
        .init();

    run(service_fn(function_handler)).await
}
