//! This module contains types that are used as public API
//! when compiling the crate to WebAssembly with wasm_bindgen.
#![cfg(feature = "wasm")]

use wasm_bindgen::prelude::*;

use std::ops;

#[wasm_bindgen]
pub struct SourceAnalysis {
    imports: Vec<super::Import>,
    exports: Vec<super::Export>,
}

#[wasm_bindgen]
impl SourceAnalysis {
    pub(crate) fn from_imports_and_exports(
        imports: Vec<super::Import>,
        exports: Vec<super::Export>,
    ) -> Self {
        SourceAnalysis { imports, exports }
    }

    #[wasm_bindgen(getter)]
    pub fn imports(&self) -> js_sys::Array {
        self.imports
            .iter()
            .cloned()
            .map(|import| match import {
                super::Import::Static(si) => JsValue::from(StaticImport { inner: si }),
                super::Import::Dynamic(di) => JsValue::from(DynamicImport { inner: di }),
                super::Import::Meta(im) => JsValue::from(ImportMeta { inner: im }),
            })
            .collect()
    }

    #[wasm_bindgen(getter)]
    pub fn exports(&self) -> js_sys::Array {
        self.exports
            .iter()
            .cloned()
            .map(|ex| JsValue::from(Export { inner: ex }))
            .collect()
    }
}

#[wasm_bindgen]
pub struct StaticImport {
    inner: super::StaticImport,
}

#[wasm_bindgen]
impl StaticImport {
    #[wasm_bindgen(js_name = "moduleSpecifierRange")]
    pub fn module_specifier_range(&self) -> Range {
        let ops::Range { start, end } = self.inner.module_specifier_range();
        Range { start, end }
    }

    #[wasm_bindgen(js_name = "statementRange")]
    pub fn statement_range(&self) -> Range {
        let ops::Range { start, end } = self.inner.statement_range();
        Range { start, end }
    }
}

#[wasm_bindgen]
pub struct DynamicImport {
    inner: super::DynamicImport,
}

#[wasm_bindgen]
impl DynamicImport {
    #[wasm_bindgen(js_name = "moduleSpecifierExpressionRange")]
    pub fn module_specifier_expression_range(&self) -> Range {
        let ops::Range { start, end } = self.inner.module_specifier_expression_range();
        Range { start, end }
    }

    #[wasm_bindgen(js_name = "importExpressionRange")]
    pub fn import_expression_range(&self) -> Range {
        let ops::Range { start, end } = self.inner.import_expression_range();
        Range { start, end }
    }
}

#[wasm_bindgen]
pub struct ImportMeta {
    inner: super::ImportMeta,
}

#[wasm_bindgen]
impl ImportMeta {
    #[wasm_bindgen(js_name = "expressionRange")]
    pub fn expression_range(&self) -> Range {
        let ops::Range { start, end } = self.inner.expression_range();
        Range { start, end }
    }
}

#[wasm_bindgen]
pub struct Export {
    inner: super::Export,
}

#[wasm_bindgen]
impl Export {
    #[wasm_bindgen(js_name = "exportSpecifierRange")]
    pub fn export_specifier_range(&self) -> Range {
        let ops::Range { start, end } = self.inner.export_specifier_range();
        Range { start, end }
    }
}

#[wasm_bindgen]
pub struct Range {
    pub start: usize,
    pub end: usize,
}
