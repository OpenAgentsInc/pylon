# Pylon Qwen3.5 Local Inference Roadmap

Date: 2026-06-09

Status: audit and first-pass roadmap for adding Psionic-backed Qwen3.5 local
inference as an optional Pylon coding-agent backend.

## Source Material Read

Pylon:

- `README.md`
- `src/inventory.ts`
- `packages/runtime/src/backends/backend-profile.ts`
- `packages/runtime/src/backends/registry.ts`
- `packages/runtime/src/backends/gemini/client.ts`
- `packages/runtime/src/backends/gemini/protocol.ts`
- `packages/runtime/src/fleet/backend-capability.ts`
- `packages/runtime/src/llm/request.ts`
- `packages/runtime/src/llm/tool.ts`
- `packages/runtime/src/llm/tool-runtime.ts`
- `packages/runtime/src/benchmark/closeout-writer.ts`
- `docs/2026-06-09-pylon-psionic-ml-connection-audit.md`
- `docs/probe-port/probe-llm-core.md`
- `docs/probe-port/2026-06-07-blueprint-signature-lookup-apple-fm-tool-use-audit.md`

Psionic:

- `docs/NON_GPT_OSS_QWEN35_PILOT.md`
- `docs/HERMES_QWEN35_COMPATIBILITY.md`
- `docs/HERMES_BACKEND_BENCHMARK.md`
- `docs/HERMES_QWEN35_REUSE_BENCHMARK.md`
- `docs/QWEN35_RESPONSES_TOOL_LOOP_PILOT.md`
- `docs/PSION_RVLLM_SAMPLING_LOOP.md`
- `docs/PSION_RVLLM_CUDA_GRAPH_POOL.md`
- `crates/psionic-serve/src/bin/psionic-openai-server.rs`
- `crates/psionic-serve/src/openai_http.rs`

## Verdict

The first Pylon pass should support both documented small Qwen3.5 rows:

- `qwen3.5:0.8b` as the lowest-footprint local smoke and fallback row;
- `qwen3.5:2b` as the first coding-agent/tool-loop quality row.

Pylon should expose them through an optional `psionic_qwen35` backend profile
that attaches to a local or remote Psionic OpenAI-compatible server. Pylon
should not bundle model weights, should not download model artifacts on normal
startup, and should not claim training or paid capacity from this work.

The smallest useful first pass is an attach-only backend:

1. operator starts or installs Psionic;
2. operator points Pylon at the Psionic base URL;
3. Pylon checks `/health` and `/v1/models`;
4. Pylon advertises local Qwen capability refs only for admitted model rows;
5. Pylon lowers provider-neutral LLM requests and tool definitions into
   OpenAI-compatible chat/responses calls;
6. Pylon records redacted transcript, tool-call, and capability receipts.

## What Psionic Already Proves

### Qwen3.5 0.8B

Psionic records the first explicit `qwen35` pilot for the Ollama
`qwen3.5:0.8b` GGUF in `docs/NON_GPT_OSS_QWEN35_PILOT.md`.

The documented artifact identity is:

- default path:
  `/home/christopherdavid/models/qwen3.5/qwen3.5-0.8b-q8_0.gguf`
- model digest:
  `afb707b6b8fac6e475acc42bc8380fc0b8d2e0e4190be5a969fbf62fcc897db5`
- chat-template digest:
  `273d8e0e683b885071fb17e08d71e5f2a5ddfb5309756181681de4f5a1822d80`

That row proves:

- real GGUF artifact detection as `qwen35`;
- real tokenizer/template facts;
- Qwen multimodal prompt-projection facts for image/video markers;
- deterministic tiny qwen35 native CUDA execution through Psionic;
- generic server publication and request execution;
- `/v1/chat/completions` and `/v1/responses` surface coverage;
- bounded qwen35 tool-loop continuation evidence through the responses pilot;
- explicit refusal boundaries for unsupported multimodal and structured-output
  paths.

The 0.8B row is the right first row for low-footprint local smoke, fast
operator validation, and fallback coding-agent tasks such as short grounded
answers, simple routing, and local tool-call plumbing checks. It should not be
treated as the default high-quality coding-agent model until Pylon retains its
own acceptance evidence.

### Qwen3.5 2B

Psionic's strongest retained coding-agent compatibility evidence is currently
on the `qwen3.5:2b` row.

Relevant retained evidence:

- `docs/HERMES_QWEN35_COMPATIBILITY.md` records a full `6/6` Hermes
  compatibility proof on native Psionic `qwen35` for the local 2B row.
- `docs/HERMES_BACKEND_BENCHMARK.md` records the same-host Hermes backend
  benchmark with Psionic model path
  `/home/christopherdavid/models/qwen3.5/qwen3.5-2b-q8_0-registry.gguf`.
- `docs/HERMES_QWEN35_REUSE_BENCHMARK.md` records repeated-loop prefix-cache
  evidence on `qwen3.5-2b-q8_0-registry.gguf`.

The 2B row is the right first row for Pylon's optional coding-agent backend
because it already has retained evidence for:

- required tool turns;
- auto plain-text turns;
- multi-turn tool loops;
- same-turn parallel tool calls;
- invalid-argument truthful refusal;
- streamed tool turns;
- serialized two-city tool loop behavior;
- repeated exact-hit warm-path reuse.

## Psionic Server Shape Pylon Should Attach To

Psionic already ships `psionic-openai-server`:

```sh
psionic-openai-server \
  -m /path/to/qwen3.5-0.8b-q8_0.gguf \
  -m /path/to/qwen3.5-2b-q8_0-registry.gguf \
  --backend cuda \
  --host 127.0.0.1 \
  --port 8080
```

The binary supports multiple `-m` model artifacts, `--backend cpu|cuda|metal`,
`--host`, `--port`, `--reasoning-budget`, and
`--mesh-coordination enabled|disabled`.

The Pylon first pass should assume attach mode:

- default base URL: `http://127.0.0.1:8080`;
- readiness path: `/health`;
- model list path: `/v1/models`;
- chat path: `/v1/chat/completions`;
- responses path: `/v1/responses`;
- management status path, when present:
  `/psionic/management/status`.

Pylon should not start Psionic implicitly until the signed sidecar release and
process-supervision gates exist. For the first pass, `pylon psionic doctor`
should explain how to start the server and verify the configured base URL.

## Pylon Backend Design

Add a third runtime backend family alongside Apple FM and Gemini:

- backend kind: `psionic_qwen35`;
- profile id: `psionic-qwen35-local`;
- capability ref: `probe.backend.psionic_qwen35`;
- default base URL: `http://127.0.0.1:8080`;
- auth mode: `none` for local attach;
- attach mode: `attach_existing`;
- stream mode: OpenAI-compatible SSE when streaming is requested;
- supported endpoints: `/v1/chat/completions`, `/v1/responses`.

The backend should reuse Pylon's provider-neutral LLM core:

- `ProbeLlmRequest`;
- `ProbeLlmMessage`;
- `ProbeLlmToolDefinition`;
- `ProbeLlmToolChoice`;
- `dispatchProbeLlmTool`;
- `ProbeLlmUsage`.

It should not create a separate Qwen-only tool runtime. Tool planning should
continue through Blueprint signature lookup and `ProbeToolMenuPlanner`, then
the Psionic backend should lower that menu into OpenAI-compatible tool schemas.

## Model Row Admission

Pylon should admit these two rows in the first pass:

| Model ref | Role | Required proof before advertisement |
| --- | --- | --- |
| `model.psionic.qwen35.0_8b.q8_0` | lowest-footprint smoke/fallback row | `/health`, `/v1/models`, artifact digest match or model manifest ref, chat completion smoke, one required-tool smoke |
| `model.psionic.qwen35.2b.q8_0` | coding-agent/tool-loop row | all 0.8B checks plus multi-turn tool-loop smoke, same-turn parallel tool-call smoke, and transcript receipt |

Pylon should prefer the 2B row for coding-agent tasks when both are ready. It
should use 0.8B for:

- install smoke;
- health and latency probes;
- simple local answer tasks;
- fallback when 2B is absent but local Qwen is still useful.

It should refuse tasks that require the 2B row when only 0.8B is available:

- multi-step coding-agent work;
- paid inference tasks;
- benchmark claims that name Hermes compatibility;
- launch copy that says Pylon has a high-quality local coding model.

## Capability Projection

`src/inventory.ts` currently has a generic `backend.local_model` row with
`blocker.backend.local_model_inventory_unproven`. Replace or refine it with
Psionic-specific rows:

- `backend.psionic.openai_server`;
- `backend.psionic.qwen35`;
- `model.psionic.qwen35.0_8b.q8_0`;
- `model.psionic.qwen35.2b.q8_0`.

Projected health should include only public-safe refs:

- backend ready/configured/missing;
- model refs;
- supported endpoint refs;
- model-cache state;
- release identity ref, if a signed Psionic sidecar is used;
- artifact digest refs, not local paths;
- blocker refs.

Projected health must not expose:

- raw GGUF paths;
- environment dumps;
- bearer tokens;
- provider secrets;
- private network topology;
- local benchmark transcripts.

## Tool-Call Integration

The Psionic Qwen backend should support two tool-call loops.

### Chat Completions Loop

Use `/v1/chat/completions` for the first coding-agent backend because Psionic's
Hermes compatibility proof is retained there.

Required behavior:

- lower `ProbeLlmToolDefinition` to OpenAI-compatible tools;
- map `ProbeLlmToolChoice` to OpenAI-compatible `tool_choice`;
- parse `message.tool_calls` and streaming `delta.tool_calls`;
- dispatch local tools through `dispatchProbeLlmTool`;
- append assistant tool-call turns and tool result turns;
- enforce a max model round-trip count;
- record redacted backend transcript and tool-call receipts.

### Responses Loop

Use `/v1/responses` when Pylon needs response-state replay.

Required behavior:

- preserve `previous_response_id`;
- preserve tool replay as `role = tool` with the tool name;
- surface Psionic response-state refs in private receipts;
- public projection should only carry redacted refs and status.

The responses loop is not required for the first useful `chat.completions`
integration, but it should be part of the roadmap because Psionic already has
a dedicated qwen35 responses tool-loop pilot.

## Acceptance Gates

Before Pylon advertises `psionic_qwen35` as ready:

1. `pylon backend psionic doctor` reaches `/health`.
2. `/health` reports `execution_engine = psionic`.
3. `/health` reports supported endpoints including `/v1/chat/completions`.
4. `/v1/models` includes at least one admitted qwen35 model row.
5. Artifact manifest or model digest matches the admitted row.
6. Plain text chat completion succeeds.
7. Required single-tool call succeeds.
8. Round-trip limit refuses infinite tool loops.
9. Receipts redact content and raw local paths.
10. `pylon inventory --json` projects only safe backend/model refs.

Before Pylon prefers 2B for coding-agent work:

1. 2B row is present in `/v1/models`.
2. Required tool turn passes.
3. Multi-turn tool loop passes.
4. Same-turn parallel tool-call smoke passes.
5. Invalid argument refusal is truthful.
6. Streamed tool-call event parsing is covered.
7. Local transcript receipts are retained and redacted.

Before any paid inference claim:

1. OpenAgents assignment lease supports local inference work class.
2. Pricing, budget, and timeout are in the lease.
3. Wallet/payout readiness is fresh.
4. Settlement path is proven.
5. Psionic artifact and backend receipts are attached to closeout.

Paid inference is not part of the first pass.

## Blocker Refs

Add specific blockers:

- `blocker.psionic_qwen35.connector_unconfigured`
- `blocker.psionic_qwen35.health_unreachable`
- `blocker.psionic_qwen35.execution_engine_not_psionic`
- `blocker.psionic_qwen35.qwen35_model_missing`
- `blocker.psionic_qwen35.model_0_8b_missing`
- `blocker.psionic_qwen35.model_2b_missing`
- `blocker.psionic_qwen35.artifact_digest_unverified`
- `blocker.psionic_qwen35.chat_completion_failed`
- `blocker.psionic_qwen35.tool_call_failed`
- `blocker.psionic_qwen35.parallel_tool_call_unproven`
- `blocker.psionic_qwen35.responses_state_unproven`
- `blocker.psionic_qwen35.paid_inference_not_admitted`

## Implementation Roadmap

### Phase 1: Attach-Only Backend

- Add `psionic_qwen35` backend profile and registry entry.
- Add env/config resolution:
  - `PYLON_PSIONIC_BASE_URL`;
  - `PROBE_PSIONIC_BASE_URL` as a compatibility alias.
- Add `pylon backend psionic doctor`.
- Add health/model-list client.
- Add public-safe availability receipt.
- Update inventory to project Psionic Qwen model refs.

### Phase 2: OpenAI-Compatible Chat Client

- Add request lowering from `ProbeLlmRequest` to OpenAI-compatible
  `/v1/chat/completions`.
- Add SSE and non-stream parser for text deltas, finish events, usage, and
  tool calls.
- Reuse `dispatchProbeLlmTool`.
- Add redacted transcript/tool-call receipts.
- Add fake Psionic server tests for plain text, required tool, malformed
  response, and round-trip limit.

### Phase 3: First-Pass Model Gates

- Add 0.8B smoke case.
- Add 2B tool-loop case.
- Add model preference policy:
  - prefer 2B for coding-agent mode;
  - allow 0.8B for smoke/fallback/simple local tasks;
  - refuse 2B-required assignments when only 0.8B is present.
- Update launch gate copy to allow "optional local Qwen inference backend" only
  after the attach and smoke gates pass.

### Phase 4: Responses State

- Add `/v1/responses` request lowering and parser.
- Preserve response-state refs privately.
- Add tool-result replay test.
- Keep public projection redacted.

### Phase 5: Sidecar And Artifact Installer

- Reuse the Psionic sidecar plan in
  `docs/2026-06-09-pylon-psionic-ml-connection-audit.md`.
- Add signed Psionic release manifest verification.
- Add model artifact manifest verification for 0.8B and 2B.
- Add opt-in artifact download, never startup auto-download.
- Add cache-by-digest layout.

## Copy Rules

Allowed after Phase 1 and Phase 2 pass:

- "Pylon can attach to a local Psionic Qwen3.5 server as an optional backend."
- "Pylon supports Qwen3.5 0.8B and 2B local inference rows when Psionic and
  verified model artifacts are present."
- "Pylon can route bounded local tool-call smokes through Psionic Qwen."

Blocked until later gates:

- "Pylon trains Qwen."
- "Pylon bundles Qwen models."
- "Pylon downloads Qwen on startup."
- "Pylon local Qwen inference is paid capacity."
- "0.8B is equivalent to the 2B coding-agent proof row."
- "Every Pylon can run Qwen."

## First-Pass Definition Of Done

The first pass is done when:

- `pylon backend psionic doctor --json` reports health and model rows;
- `pylon inventory --json` shows safe Psionic backend/model refs;
- a fake Psionic server test covers 0.8B and 2B model-list admission;
- a fake Psionic server test covers required tool calls;
- a fixture or local smoke proves 0.8B plain text generation;
- a fixture or local smoke proves 2B required-tool and multi-turn tool loops;
- docs and launch gates distinguish optional inference from training and paid
  work.

That is the smallest honest path to Qwen in Pylons: attach to Psionic,
support both small rows, prefer 2B for coding-agent work, keep 0.8B as the low
footprint row, and block all training or paid-capacity claims until their own
receipts exist.
