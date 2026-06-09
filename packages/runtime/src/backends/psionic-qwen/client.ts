import { Effect, Schema as S } from "effect";
import { type ResolvedProbeBackendProfile, type ResolveProbeBackendProfileOptions } from "../backend-profile";
import { resolvePsionicQwenBackendProfile, type ProbeBackendRegistryError } from "../registry";
import {
  PSIONIC_QWEN_MODEL_REFS,
  PSIONIC_QWEN_SUPPORTED_ENDPOINT_REFS,
  PsionicQwenHealthResponse,
  PsionicQwenModelListResponse,
  type PsionicQwenHealthResponse,
  type PsionicQwenModelListResponse,
} from "./contract";
import {
  makePsionicQwenAvailabilityReceipt,
  type PsionicQwenAvailabilityReceipt,
} from "./receipts";

export const PsionicQwenHealthStatus = S.Literals(["ready", "configured", "malformed", "unreachable"]);
export type PsionicQwenHealthStatus = typeof PsionicQwenHealthStatus.Type;

export interface PsionicQwenClientOptions extends ResolveProbeBackendProfileOptions {
  readonly fetch?: typeof fetch;
  readonly now?: Date;
}

export interface PsionicQwenReadiness {
  readonly profile: ResolvedProbeBackendProfile;
  readonly status: PsionicQwenHealthStatus;
  readonly ready: boolean;
  readonly health?: PsionicQwenHealthResponse;
  readonly modelIds: ReadonlyArray<string>;
  readonly modelRefs: ReadonlyArray<string>;
  readonly supportedEndpointRefs: ReadonlyArray<string>;
  readonly blockerRefs: ReadonlyArray<string>;
  readonly message?: string;
  readonly receipt: PsionicQwenAvailabilityReceipt;
}

export interface PsionicQwenClient {
  readonly profile: ResolvedProbeBackendProfile;
  readonly doctor: () => Effect.Effect<PsionicQwenReadiness, never>;
}

export function makePsionicQwenClient(
  options: PsionicQwenClientOptions = {},
): Effect.Effect<PsionicQwenClient, ProbeBackendRegistryError> {
  return Effect.gen(function* () {
    const profile = yield* resolvePsionicQwenBackendProfile(options);
    const fetchImpl = options.fetch ?? fetch;
    const now = () => (options.now ?? new Date()).toISOString();

    return {
      profile,
      doctor: () => checkPsionicQwenHealth(profile, fetchImpl, now()),
    };
  });
}

export function checkPsionicQwenHealth(
  profile: ResolvedProbeBackendProfile,
  fetchImpl: typeof fetch = fetch,
  observedAt = new Date().toISOString(),
): Effect.Effect<PsionicQwenReadiness, never> {
  return Effect.gen(function* () {
    const healthEndpoint = new URL(profile.readinessPath, withTrailingSlash(profile.baseUrl));
    const healthResponse = yield* Effect.tryPromise({
      try: () => fetchImpl(healthEndpoint, { method: "GET" }),
      catch: (error) =>
        psionicReadiness(profile, {
          status: "unreachable",
          blockerRefs: ["blocker.psionic_qwen35.health_unreachable"],
          message: `Psionic health endpoint is unreachable: ${String(error)}`,
          observedAt,
        }),
    });

    if (isReadiness(healthResponse)) {
      return healthResponse;
    }

    if (!healthResponse.ok) {
      return psionicReadiness(profile, {
        status: "unreachable",
        blockerRefs: ["blocker.psionic_qwen35.health_unreachable"],
        message: `Psionic health endpoint returned HTTP ${healthResponse.status}`,
        observedAt,
      });
    }

    const healthRaw = yield* Effect.tryPromise({
      try: () => healthResponse.json(),
      catch: (error) =>
        psionicReadiness(profile, {
          status: "malformed",
          blockerRefs: ["blocker.psionic_qwen35.health_unreachable"],
          message: `Psionic health response was not JSON: ${String(error)}`,
          observedAt,
        }),
    });

    if (isReadiness(healthRaw)) {
      return healthRaw;
    }

    const health = yield* S.decodeUnknownEffect(PsionicQwenHealthResponse)(normalizeHealthResponse(healthRaw)).pipe(
      Effect.mapError((error) =>
        psionicReadiness(profile, {
          status: "malformed",
          blockerRefs: ["blocker.psionic_qwen35.health_unreachable"],
          message: `Psionic health response was malformed: ${String(error)}`,
          observedAt,
        }),
      ),
    );

    if (isReadiness(health)) {
      return health;
    }

    const modelsResponse = yield* Effect.tryPromise({
      try: () => fetchImpl(new URL("/v1/models", withTrailingSlash(profile.baseUrl)), { method: "GET" }),
      catch: (error) =>
        psionicReadiness(profile, {
          status: "unreachable",
          health,
          blockerRefs: ["blocker.psionic_qwen35.qwen35_model_missing"],
          supportedEndpointRefs: endpointRefsFromHealth(health),
          message: `Psionic model endpoint is unreachable: ${String(error)}`,
          observedAt,
        }),
    });

    if (isReadiness(modelsResponse)) {
      return modelsResponse;
    }

    if (!modelsResponse.ok) {
      return psionicReadiness(profile, {
        status: "configured",
        health,
        blockerRefs: ["blocker.psionic_qwen35.qwen35_model_missing"],
        supportedEndpointRefs: endpointRefsFromHealth(health),
        message: `Psionic model endpoint returned HTTP ${modelsResponse.status}`,
        observedAt,
      });
    }

    const modelsRaw = yield* Effect.tryPromise({
      try: () => modelsResponse.json(),
      catch: (error) =>
        psionicReadiness(profile, {
          status: "malformed",
          health,
          blockerRefs: ["blocker.psionic_qwen35.qwen35_model_missing"],
          supportedEndpointRefs: endpointRefsFromHealth(health),
          message: `Psionic model response was not JSON: ${String(error)}`,
          observedAt,
        }),
    });

    if (isReadiness(modelsRaw)) {
      return modelsRaw;
    }

    const models = yield* S.decodeUnknownEffect(PsionicQwenModelListResponse)(normalizeModelListResponse(modelsRaw)).pipe(
      Effect.mapError((error) =>
        psionicReadiness(profile, {
          status: "malformed",
          health,
          blockerRefs: ["blocker.psionic_qwen35.qwen35_model_missing"],
          supportedEndpointRefs: endpointRefsFromHealth(health),
          message: `Psionic model response was malformed: ${String(error)}`,
          observedAt,
        }),
      ),
    );

    if (isReadiness(models)) {
      return models;
    }

    const modelIds = uniqueStrings([...modelIdsFromHealth(health), ...modelIdsFromModelList(models)]);
    const modelRefs = modelIds.flatMap(modelRefFromModelId);
    const supportedEndpointRefs = endpointRefsFromHealth(health);
    const blockerRefs = psionicBlockers(health, modelRefs);

    return psionicReadiness(profile, {
      status: blockerRefs.length === 0 ? "ready" : "configured",
      health,
      modelIds,
      modelRefs,
      supportedEndpointRefs,
      blockerRefs,
      message: blockerRefs.length === 0 ? health.message : blockerRefs.join(", "),
      observedAt,
    });
  }).pipe(Effect.catch((readiness: PsionicQwenReadiness) => Effect.succeed(readiness)));
}

function psionicReadiness(
  profile: ResolvedProbeBackendProfile,
  input: {
    readonly status: PsionicQwenHealthStatus;
    readonly health?: PsionicQwenHealthResponse;
    readonly modelIds?: ReadonlyArray<string>;
    readonly modelRefs?: ReadonlyArray<string>;
    readonly supportedEndpointRefs?: ReadonlyArray<string>;
    readonly blockerRefs?: ReadonlyArray<string>;
    readonly message?: string;
    readonly observedAt: string;
  },
): PsionicQwenReadiness {
  const modelIds = input.modelIds ?? [];
  const modelRefs = input.modelRefs ?? [];
  const supportedEndpointRefs = input.supportedEndpointRefs ?? [];
  const blockerRefs = input.blockerRefs ?? [];
  const ready = input.status === "ready" && blockerRefs.length === 0;

  return {
    profile,
    status: input.status,
    ready,
    health: input.health,
    modelIds,
    modelRefs,
    supportedEndpointRefs,
    blockerRefs,
    message: input.message,
    receipt: makePsionicQwenAvailabilityReceipt({
      profileId: profile.id,
      model: profile.model,
      baseUrl: profile.baseUrl,
      ready,
      status: input.status,
      modelRefs,
      supportedEndpointRefs,
      blockerRefs,
      message: input.message,
      observedAt: input.observedAt,
    }),
  };
}

function psionicBlockers(
  health: PsionicQwenHealthResponse,
  modelRefs: ReadonlyArray<string>,
): ReadonlyArray<string> {
  const blockers: string[] = [];
  const engine = health.execution_engine ?? health.executionEngine ?? health.backend;

  if (engine !== undefined && engine.toLowerCase() !== "psionic") {
    blockers.push("blocker.psionic_qwen35.execution_engine_not_psionic");
  }

  if (modelRefs.length === 0) {
    blockers.push("blocker.psionic_qwen35.qwen35_model_missing");
  }

  return blockers;
}

function endpointRefsFromHealth(health: PsionicQwenHealthResponse): ReadonlyArray<string> {
  const endpoints = uniqueStrings([...(health.supported_endpoints ?? []), ...(health.supportedEndpoints ?? [])])
    .map((value) => value.toLowerCase());
  const refs = new Set<string>([PSIONIC_QWEN_SUPPORTED_ENDPOINT_REFS.health, PSIONIC_QWEN_SUPPORTED_ENDPOINT_REFS.models]);

  if (endpoints.some((endpoint) => endpoint.includes("chat/completions") || endpoint.includes("chat_completions"))) {
    refs.add(PSIONIC_QWEN_SUPPORTED_ENDPOINT_REFS.chatCompletions);
  }

  if (endpoints.some((endpoint) => endpoint.includes("responses"))) {
    refs.add(PSIONIC_QWEN_SUPPORTED_ENDPOINT_REFS.responses);
  }

  return [...refs];
}

function modelIdsFromHealth(health: PsionicQwenHealthResponse): ReadonlyArray<string> {
  return uniqueStrings([
    health.default_model,
    health.defaultModel,
    health.model,
    ...(health.models ?? []),
  ].filter(isString));
}

function modelIdsFromModelList(models: PsionicQwenModelListResponse): ReadonlyArray<string> {
  return uniqueStrings((models.data ?? []).map((model) => typeof model === "string" ? model : model.id));
}

export function modelRefFromModelId(modelId: string): ReadonlyArray<string> {
  const normalized = modelId.toLowerCase().replace(/[-/.:]+/g, "_");
  const isQwen35 = normalized.includes("qwen3_5") || normalized.includes("qwen35") || normalized.includes("qwen_3_5");

  if (!isQwen35) {
    return [];
  }

  if (normalized.includes("0_8") || normalized.includes("0.8") || normalized.includes("08b")) {
    return [PSIONIC_QWEN_MODEL_REFS.qwen35_0_8b];
  }

  if (normalized.includes("2b") || normalized.includes("2_b")) {
    return [PSIONIC_QWEN_MODEL_REFS.qwen35_2b];
  }

  return [];
}

function normalizeHealthResponse(value: unknown): unknown {
  if (typeof value !== "object" || value === null) {
    return value;
  }

  return value;
}

function normalizeModelListResponse(value: unknown): unknown {
  if (typeof value !== "object" || value === null) {
    return value;
  }

  const input = value as Record<string, unknown>;
  const data = Array.isArray(input.data) ? input.data : [];

  return {
    object: typeof input.object === "string" ? input.object : undefined,
    data,
  };
}

function uniqueStrings(values: ReadonlyArray<string>): ReadonlyArray<string> {
  return [...new Set(values.filter((value) => value.trim().length > 0))];
}

function isString(value: unknown): value is string {
  return typeof value === "string";
}

function isReadiness(value: unknown): value is PsionicQwenReadiness {
  return typeof value === "object" && value !== null && "profile" in value && "receipt" in value && "status" in value;
}

function withTrailingSlash(value: string): string {
  return value.endsWith("/") ? value : `${value}/`;
}
