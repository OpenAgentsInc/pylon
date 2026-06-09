# Pylon

![Pylon](docs/images/pylon.png)

## Tech stack
- [Bun](https://bun.sh)
- [Effect](https://effect.website/)
- [OpenTUI](https://github.com/anomalyco/opentui)

## Launch Package

The v0.3 release-candidate package is `@openagentsinc/pylon@0.3.0-rc1` and
exposes the `pylon` binary. Do not treat `0.3.0` as stable until the launch
gates pass.

Initial supported operator platforms are macOS and Linux. No other operator
platforms are in scope for the first v0.3 launch path.

## Runtime Backends

Pylon now carries the former Probe runtime as `@openagentsinc/pylon-runtime`.
The public `pylon` binary bundles that runtime source, keeps the OpenTUI node
dashboard as the default, and routes backend/runtime commands through the same
binary:

```sh
pylon runtime backend gemini smoke
pylon backend gemini complete --prompt "Summarize the current task."
pylon apple-fm status
pylon apple-fm tool-stream-demo
```

## Bootstrap And Status

```sh
pylon bootstrap --json
pylon bootstrap --register-openagents --setup-mdk-wallet --pylon-ref <ref> --display-name <name> --resource-mode background_20 --capability-ref <ref> --json
pylon status --json
```

`bootstrap` creates the local v0.3 home/cache/release layout and writes a
minimal public-safe config summary. Live registration and MDK mutation are
tracked by later launch gates.

The runtime includes:

- Apple Foundation Models bridge support, readiness receipts, streaming tool
  callbacks, and Program Run evidence.
- Gemini direct API and Omega-brokered Gemini materialization.
- Provider-neutral LLM message/request/tool/usage contracts.
- Blueprint signature lookup, tool-menu planning, Action Submission boundaries,
  and contribution release gates.
- Retained OpenTUI Markdown rendering helpers and markdown/code streaming
  fixtures.
- GEPA/Terminal-Bench candidate execution, closeout bundles, token telemetry,
  runner identity, and Omega grant/account contracts.
