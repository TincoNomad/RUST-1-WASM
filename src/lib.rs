use wasm_bindgem::prelude::*;

#[wasm_bindgem]
extern {
    pub fn alert(s: &str);
}

#[wasm_bindgem]
pub fn saludar(nombre : &str) {
    alert(&format!("Hola, {}, ¿Cómo estas? ", nombre))
}