import fs from "node:fs";
import path from "node:path";
import { assert } from "./helpers.mjs";

const root = process.env.FLAGS2ENV_UPSTREAM_ROOT ?? "upstream";
const contract = fs.readFileSync(path.join(root, "sidecar", ".cli-flags.toml"), "utf8");

for (const command of ["preflight", "probe", "probe-readyz"]) {
  assert(contract.includes(`[commands.${command}]`), `missing sidecar command ${command}`);
}
assert(contract.includes('aliases = ["probe-healthz"]'), "health probe alias remains canonical");
assert(contract.includes('default = "127.0.0.1:9090"'), "sidecar stays loopback by default");
assert(contract.includes('env = "FLAGS_2_ENV_SIDECAR_ALLOW_NON_LOOPBACK"'), "non-loopback opt-in is explicit");

console.log("flags-2-env sidecar command/probe transport contract ok");
