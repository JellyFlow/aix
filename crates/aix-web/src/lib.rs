use aix::AixReader;
use anyhow::Result;
use wasm_bindgen::prelude::*;

fn to_value<T: serde::Serialize + ?Sized>(value: &T) -> Result<JsValue, JsValue> {
    value
        .serialize(&serde_wasm_bindgen::Serializer::new().serialize_maps_as_objects(true))
        .map_err(|e| JsValue::from_str(&e.to_string()))
}

#[wasm_bindgen]
pub struct AixReaderWasm {
    inner: AixReader,
}

#[wasm_bindgen]
impl AixReaderWasm {
    #[wasm_bindgen(constructor)]
    pub fn new(data: Vec<u8>) -> Result<AixReaderWasm, JsValue> {
        let inner = AixReader::new(data).map_err(|e| JsValue::from_str(&e.to_string()))?;
        Ok(AixReaderWasm { inner })
    }

    pub fn list(&self) -> Result<JsValue, JsValue> {
        to_value(&self.inner.list())
    }

    pub fn read_file(&self, name: &str) -> Result<Vec<u8>, JsValue> {
        self.inner
            .read_file(name)
            .map_err(|e| JsValue::from_str(&e.to_string()))
    }

    pub fn get_version(&self) -> Option<String> {
        self.inner.get_version()
    }

    pub fn get_title(&self) -> Option<String> {
        self.inner.get_title()
    }

    pub fn get_pages(&self) -> Result<JsValue, JsValue> {
        to_value(&self.inner.get_pages())
    }

    pub fn get_tools(&self) -> Result<JsValue, JsValue> {
        to_value(&self.inner.get_tools())
    }
}
