import fs from "node:fs";
import path from "node:path";
import { assert } from "./helpers.mjs";

const root = process.env.FLAGS2ENV_UPSTREAM_ROOT ?? "upstream";
const read = (...parts) => fs.readFileSync(path.join(root, ...parts), "utf8");

const schema = JSON.parse(read("interfaces", "schema", "v1", "flagcatalog.json"));
const typespec = read("interfaces", "schema", "v1", "flagcatalog.tsp");
const sidecarFlags = read("sidecar", ".cli-flags.toml");
const sidecarDockerfile = read("sidecar", "Dockerfile");
const coreCargo = read("core", "clients", "rust", "Cargo.toml");

assert(schema.$schema === "https://json-schema.org/draft/2020-12/schema", "Draft 2020-12 authority");
assert(schema.additionalProperties === false, "FlagCatalog closes root properties");
assert(schema.required.join(",") === "id,revision,payload", "FlagCatalog required fields");
assert(typespec.includes("namespace Flags2Env.Contracts.V1"), "TypeSpec peer authority namespace");
assert(typespec.includes('extension("additionalProperties", false)'), "TypeSpec closes root properties independently");
assert(coreCargo.includes('name = "flags2env"'), "Rust runtime is the canonical flags2env client");
assert(sidecarFlags.includes("files = []"), "sidecar disables ambient dotenv files");
assert(sidecarFlags.includes("allow_unknown = false"), "sidecar rejects undeclared argv");
assert(sidecarDockerfile.includes('COPY --chown=65532:65532 ".cli-flags.toml" "/.cli-flags.toml"'), "distroless image carries audited flags contract");

console.log("flags-2-env landed cross-repo authorities agree on the runtime boundary");
