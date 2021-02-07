/* tslint:disable */
/* eslint-disable */
/**
* @param {string} input
* @returns {SourceAnalysis}
*/
export function parse(input: string): SourceAnalysis;
/**
*/
export class DynamicImport {
  free(): void;
/**
* @returns {Range}
*/
  moduleSpecifierExpressionRange(): Range;
/**
* @returns {Range}
*/
  importExpressionRange(): Range;
}
/**
*/
export class Export {
  free(): void;
/**
* @returns {Range}
*/
  exportSpecifierRange(): Range;
}
/**
*/
export class ImportMeta {
  free(): void;
/**
* @returns {Range}
*/
  expressionRange(): Range;
}
/**
*/
export class Range {
  free(): void;
/**
* @returns {number}
*/
  end: number;
/**
* @returns {number}
*/
  start: number;
}
/**
*/
export class SourceAnalysis {
  free(): void;
/**
* @returns {Array<any>}
*/
  readonly exports: Array<any>;
/**
* @returns {Array<any>}
*/
  readonly imports: Array<any>;
}
/**
*/
export class StaticImport {
  free(): void;
/**
* @returns {Range}
*/
  moduleSpecifierRange(): Range;
/**
* @returns {Range}
*/
  statementRange(): Range;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly parse: (a: number, b: number) => number;
  readonly __wbg_sourceanalysis_free: (a: number) => void;
  readonly sourceanalysis_imports: (a: number) => number;
  readonly sourceanalysis_exports: (a: number) => number;
  readonly __wbg_staticimport_free: (a: number) => void;
  readonly staticimport_statementRange: (a: number) => number;
  readonly __wbg_dynamicimport_free: (a: number) => void;
  readonly dynamicimport_moduleSpecifierExpressionRange: (a: number) => number;
  readonly dynamicimport_importExpressionRange: (a: number) => number;
  readonly __wbg_export_free: (a: number) => void;
  readonly export_exportSpecifierRange: (a: number) => number;
  readonly __wbg_get_range_start: (a: number) => number;
  readonly __wbg_set_range_start: (a: number, b: number) => void;
  readonly __wbg_get_range_end: (a: number) => number;
  readonly __wbg_set_range_end: (a: number, b: number) => void;
  readonly __wbg_importmeta_free: (a: number) => void;
  readonly __wbg_range_free: (a: number) => void;
  readonly staticimport_moduleSpecifierRange: (a: number) => number;
  readonly importmeta_expressionRange: (a: number) => number;
  readonly __wbindgen_malloc: (a: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number) => number;
}

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
        