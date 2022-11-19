import { toast } from "@zerodevx/svelte-toast"
import { readable, type Subscriber } from "svelte/store"

export const gpu_avail = readable<boolean>(true, (set: Subscriber<boolean>) => {
  let hasWebgpu = ('gpu' in (navigator ?? {}))
  if (!hasWebgpu)
    toast.push('Your browser is incompatible with gputoy :-(')
  set(hasWebgpu)
})