# Pylon

![Pylon](docs/images/pylon.png)

## Tech stack
- [Bun](https://bun.sh)
- [Effect](https://effect.website/)
- [OpenTUI](https://github.com/anomalyco/opentui)

## Launch Package

The v0.3 launch package is `@openagentsinc/pylon` and exposes the `pylon`
binary.

Initial supported operator platforms are macOS and Linux. No other operator
platforms are in scope for the first v0.3 launch path.

## Runtime Backends

Pylon now carries the former Probe runtime as `@openagentsinc/pylon-runtime`.
The public `pylon` binary keeps the OpenTUI node dashboard as the default and
routes backend/runtime commands through the same binary:

```sh
pylon runtime backend gemini smoke
pylon backend gemini complete --prompt "Summarize the current task."
pylon apple-fm status
pylon apple-fm tool-stream-demo
```

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
