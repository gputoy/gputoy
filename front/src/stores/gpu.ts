import { readable, type Subscriber } from "svelte/store"

export const gpu_avail = readable<boolean>(false, (set: Subscriber<boolean>) => {
  // console.log("is set: ", ('gpu' in navigator))
  // set(('gpu' in navigator))
})

// async function _init() {
//   if (!('gpu' in navigator)) {
//     return;
//   }

//   const wasm = await init();

//   let context;
//   try {
//     context = wasm.build();
//   } catch (e) {
//     console.error('context build ended with error: ', e);
//     return;
//   }
//   console.log('context created successfully, ', context);

//   wasm.print_context(context);
// }