import { Context, default as init_module } from '../../pkg/gpu_wasm';

var context: Context;
await init();

export async function init() {
  const wasm = await init_module();
  const context = wasm.build();
  console.log("js:context:init ", context);

}

export async function reset() {
  console.log("js:context:reset");
  // drop all gpu resources
  context.free();
  await init()
}

export async function stop() {
  console.log("js:context:stop");
  // drop all gpu resources
  context.free();
}