/* tslint:disable */
/* eslint-disable */
/**
* @param {string} id
* @returns {any}
*/
export function initialize(id: string): any;
/**
* @param {any} scene
* @param {string} color
*/
export function change_background_color(scene: any, color: string): void;
/**
*/
export class GroundOptions {
  free(): void;
/**
*/
  height: number;
/**
*/
  width: number;
}
/**
*/
export class SphereOptions {
  free(): void;
/**
*/
  diameter: number;
/**
*/
  segments: number;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly initialize: (a: number, b: number) => number;
  readonly change_background_color: (a: number, b: number, c: number) => void;
  readonly __wbg_groundoptions_free: (a: number) => void;
  readonly __wbg_get_groundoptions_width: (a: number) => number;
  readonly __wbg_set_groundoptions_width: (a: number, b: number) => void;
  readonly __wbg_get_groundoptions_height: (a: number) => number;
  readonly __wbg_set_groundoptions_height: (a: number, b: number) => void;
  readonly __wbg_sphereoptions_free: (a: number) => void;
  readonly __wbg_set_sphereoptions_segments: (a: number, b: number) => void;
  readonly __wbg_get_sphereoptions_diameter: (a: number) => number;
  readonly __wbg_set_sphereoptions_diameter: (a: number, b: number) => void;
  readonly __wbg_get_sphereoptions_segments: (a: number) => number;
  readonly __wbindgen_export_0: WebAssembly.Table;
  readonly _dyn_core__ops__function__FnMut_____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h53de206a4c2265c7: (a: number, b: number) => void;
  readonly __wbindgen_malloc: (a: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number) => number;
  readonly __wbindgen_free: (a: number, b: number) => void;
  readonly __wbindgen_exn_store: (a: number) => void;
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
