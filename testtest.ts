import {  instantiate} from "./lib/vscode_deno_lint.generated.js"

const { lint}=await instantiate()
const res = lint("file:///C:/Users/ayame/work/deno/vscode-deno-lint/testtest.ts", `
var aaaaa = 1;
`, false)
console.log(res)
