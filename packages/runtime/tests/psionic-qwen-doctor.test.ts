import { describe, expect, test } from "bun:test";
import { Effect } from "effect";
import { makePsionicQwenClient, modelRefFromModelId, runProbeCli } from "../src";

describe("Psionic Qwen doctor", () => {
  test("reports ready for a Psionic server with Qwen3.5 models", async () => {
    const client = await Effect.runPromise(makePsionicQwenClient({
      explicitBaseUrl: "http://127.0.0.1:18080",
      fetch: fakePsionicFetch({
        health: {
          ready: true,
          execution_engine: "psionic",
          supported_endpoints: ["/v1/chat/completions", "/v1/responses"],
        },
        models: {
          data: [{ id: "qwen3.5-0.8b" }, { id: "qwen3.5-2b" }],
        },
      }),
      now: new Date("2026-06-09T00:00:00.000Z"),
    }));
    const readiness = await Effect.runPromise(client.doctor());

    expect(readiness.ready).toBe(true);
    expect(readiness.status).toBe("ready");
    expect(readiness.modelRefs).toContain("model.psionic.qwen35.0_8b.q8_0");
    expect(readiness.modelRefs).toContain("model.psionic.qwen35.2b.q8_0");
    expect(readiness.supportedEndpointRefs).toContain("endpoint.psionic.v1.chat_completions");
    expect(readiness.supportedEndpointRefs).toContain("endpoint.psionic.v1.responses");
    expect(readiness.blockerRefs).toEqual([]);
    expect(JSON.stringify(readiness.receipt)).not.toContain("18080?");
  });

  test("reports unreachable without throwing when Psionic is absent", async () => {
    const client = await Effect.runPromise(makePsionicQwenClient({
      fetch: async () => {
        throw new Error("connection refused");
      },
      now: new Date("2026-06-09T00:00:00.000Z"),
    }));
    const readiness = await Effect.runPromise(client.doctor());

    expect(readiness.ready).toBe(false);
    expect(readiness.status).toBe("unreachable");
    expect(readiness.blockerRefs).toContain("blocker.psionic_qwen35.health_unreachable");
  });

  test("reports malformed without throwing for bad health JSON", async () => {
    const client = await Effect.runPromise(makePsionicQwenClient({
      fetch: fakePsionicFetch({
        healthResponse: new Response("not json", { status: 200 }),
        models: { data: [] },
      }),
      now: new Date("2026-06-09T00:00:00.000Z"),
    }));
    const readiness = await Effect.runPromise(client.doctor());

    expect(readiness.ready).toBe(false);
    expect(readiness.status).toBe("malformed");
    expect(readiness.blockerRefs).toContain("blocker.psionic_qwen35.health_unreachable");
  });

  test("blocks non-Psionic execution engines", async () => {
    const client = await Effect.runPromise(makePsionicQwenClient({
      fetch: fakePsionicFetch({
        health: {
          ready: true,
          execution_engine: "llama_cpp",
          supported_endpoints: ["/v1/chat/completions"],
        },
        models: {
          data: [{ id: "qwen3.5-2b" }],
        },
      }),
      now: new Date("2026-06-09T00:00:00.000Z"),
    }));
    const readiness = await Effect.runPromise(client.doctor());

    expect(readiness.ready).toBe(false);
    expect(readiness.status).toBe("configured");
    expect(readiness.blockerRefs).toContain("blocker.psionic_qwen35.execution_engine_not_psionic");
  });

  test("CLI JSON doctor returns a typed failure without process crashes", async () => {
    const result = await Effect.runPromise(runProbeCli(["backend", "psionic", "doctor", "--json"], {
      fetch: fakePsionicFetch({
        health: {
          ready: true,
          execution_engine: "psionic",
        },
        models: {
          data: [],
        },
      }),
      now: new Date("2026-06-09T00:00:00.000Z"),
    }));
    const payload = JSON.parse(result.stdout);

    expect(result.exitCode).toBe(1);
    expect(payload.profile.kind).toBe("psionic_qwen35");
    expect(payload.blockerRefs).toContain("blocker.psionic_qwen35.qwen35_model_missing");
  });

  test("maps Qwen3.5 0.8B and 2B identifiers to public model refs", () => {
    expect(modelRefFromModelId("Qwen3.5-0.8B-Instruct-GGUF")).toEqual(["model.psionic.qwen35.0_8b.q8_0"]);
    expect(modelRefFromModelId("qwen35:2b-q8_0")).toEqual(["model.psionic.qwen35.2b.q8_0"]);
    expect(modelRefFromModelId("/Users/example/qwen3.5-2b.gguf")).toEqual(["model.psionic.qwen35.2b.q8_0"]);
    expect(modelRefFromModelId("gemma-2b")).toEqual([]);
  });
});

function fakePsionicFetch(input: {
  readonly health?: unknown;
  readonly healthResponse?: Response;
  readonly models?: unknown;
  readonly modelsResponse?: Response;
}): typeof fetch {
  return async (url) => {
    const path = new URL(url.toString()).pathname;

    if (path === "/health") {
      return input.healthResponse ?? Response.json(input.health ?? { ready: true, execution_engine: "psionic" });
    }

    if (path === "/v1/models") {
      return input.modelsResponse ?? Response.json(input.models ?? { data: [] });
    }

    return new Response("not found", { status: 404 });
  };
}
