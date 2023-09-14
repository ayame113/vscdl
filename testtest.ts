import init, {run} from "./wasm/deno_lint.js"

 await init();


const res = await run("test.tsx", `var aaaaa = 1;`)
console.log("aaa", res)
