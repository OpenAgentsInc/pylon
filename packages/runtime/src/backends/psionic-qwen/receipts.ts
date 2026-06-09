import { Schema as S } from "effect";
import { PSIONIC_QWEN_BACKEND_KIND } from "./contract";

export const PsionicQwenAvailabilityReceipt = S.Struct({
  kind: S.Literal("probe_backend_availability"),
  backendKind: S.Literal(PSIONIC_QWEN_BACKEND_KIND),
  profileId: S.String,
  model: S.String,
  baseUrl: S.String,
  ready: S.Boolean,
  status: S.String,
  modelRefs: S.Array(S.String),
  supportedEndpointRefs: S.Array(S.String),
  blockerRefs: S.Array(S.String),
  message: S.optional(S.String),
  observedAt: S.String,
  contentRedacted: S.Literal(true),
});
export type PsionicQwenAvailabilityReceipt = typeof PsionicQwenAvailabilityReceipt.Type;

export function makePsionicQwenAvailabilityReceipt(input: {
  readonly profileId: string;
  readonly model: string;
  readonly baseUrl: string;
  readonly ready: boolean;
  readonly status: string;
  readonly modelRefs: ReadonlyArray<string>;
  readonly supportedEndpointRefs: ReadonlyArray<string>;
  readonly blockerRefs: ReadonlyArray<string>;
  readonly message?: string;
  readonly observedAt?: string;
}): PsionicQwenAvailabilityReceipt {
  return {
    kind: "probe_backend_availability",
    backendKind: PSIONIC_QWEN_BACKEND_KIND,
    profileId: input.profileId,
    model: input.model,
    baseUrl: redactUrl(input.baseUrl),
    ready: input.ready,
    status: input.status,
    modelRefs: [...input.modelRefs],
    supportedEndpointRefs: [...input.supportedEndpointRefs],
    blockerRefs: [...input.blockerRefs],
    message: input.message,
    observedAt: input.observedAt ?? new Date().toISOString(),
    contentRedacted: true,
  };
}

export function redactUrl(value: string): string {
  try {
    const url = new URL(value);
    url.username = "";
    url.password = "";
    url.search = "";
    url.hash = "";
    return url.toString().replace(/\/$/, "");
  } catch {
    return "[redacted-invalid-url]";
  }
}
