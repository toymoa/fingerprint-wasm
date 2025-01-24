use wasm_bindgen::prelude::*;
use web_sys::window;
use serde::Serialize;
use sha2::{Sha256, Digest};

#[wasm_bindgen]
pub async fn get_browser_fingerprint() -> Result<JsValue, JsValue> {
    let window = window().ok_or_else(|| JsValue::from_str("No global window available"))?;
    let navigator = window.navigator();
    let screen = window.screen().map_err(|_| JsValue::from_str("No screen available"))?;

    #[derive(Serialize)]
    struct Fingerprint {
        user_agent: String,
        platform: String,
        languages: String,
        timezone: String,
        screen_resolution: String,
        color_depth: u32,
        canvas: String,
        webgl: String,
        touch_support: bool,
    }

    let user_agent = navigator.user_agent()?;
    let platform = navigator.platform()?;
    let languages: String = navigator.languages().join(",").into();
    let screen_width = screen.width().map_err(|_| JsValue::from_str("Failed to get screen width"))?;
    let screen_height = screen.height().map_err(|_| JsValue::from_str("Failed to get screen height"))?;
    let screen_resolution = format!("{}x{}", screen_width, screen_height);
    let color_depth = screen.color_depth().map_err(|_| JsValue::from_str("Failed to get color depth"))? as u32;

    let timezone = "UTC".to_string(); // 고정값 예제
    let canvas = "canvas_data".to_string(); // 캔버스 데이터 로직 필요
    let webgl = "webgl_data".to_string(); // WebGL 데이터 로직 필요
    let touch_support = true;

    let fingerprint = Fingerprint {
        user_agent,
        platform,
        languages,
        timezone,
        screen_resolution,
        color_depth,
        canvas,
        webgl,
        touch_support,
    };

    let fingerprint_json =
        serde_json::to_string(&fingerprint).map_err(|e| JsValue::from_str(&e.to_string()))?;
    let mut hasher = Sha256::new();
    hasher.update(fingerprint_json.as_bytes());
    let hash = format!("{:x}", hasher.finalize());

    Ok(JsValue::from_str(&hash))
}
