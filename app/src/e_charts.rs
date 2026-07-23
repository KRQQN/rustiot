#![cfg_attr(feature = "ssr", allow(unused_variables, unused_imports, dead_code))]
use js_sys::Object;
use wasm_bindgen::prelude::*;
use web_sys::HtmlDivElement;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = createChart)]
    fn create_chart(div: HtmlDivElement, options: &JsValue) -> Object;

    #[wasm_bindgen(js_name = updateChart)]
    fn update_chart(chart: &Object, options: &JsValue);

    #[wasm_bindgen(js_name = resizeChart)]
    fn resize_chart(chart: &Object);

    #[wasm_bindgen(js_name = disposeChart)]
    fn dispose_chart(chart: &Object);
}

pub struct EChart {
    inner: Object,
}

impl EChart {
    pub fn new(div: HtmlDivElement, options: &JsValue) -> Self {
        Self {
            inner: create_chart(div, options),
        }
    }

    pub fn update(&self, options: &JsValue) {
        update_chart(&self.inner, options);
    }

    pub fn resize(&self) {
        resize_chart(&self.inner);
    }
}

impl Drop for EChart {
    fn drop(&mut self) {
        dispose_chart(&self.inner);
    }
}
