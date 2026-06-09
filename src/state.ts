import { mkdir, readFile, writeFile } from "node:fs/promises"
import { existsSync } from "node:fs"
import { createHash, generateKeyPairSync, randomUUID } from "node:crypto"
import { hostname } from "node:os"
import { dirname } from "node:path"
import type { BootstrapSummary } from "./bootstrap"

export type PylonLifecycleState = "offline" | "online" | "paused" | "degraded" | "assignment-ready"

export type PylonPaths = BootstrapSummary["paths"] & {
  identity: string
  runtimeState: string
  ledger: string
}

export type PylonIdentity = {
  nodeId: string
  pylonRef: string
  nodeLabel: string
  publicKey: string
  npub: string
  createdAt: string
}

export type PylonPrivateIdentityRecord = PylonIdentity & {
  privateKeyPem: string
}

export type PylonRuntimeState = {
  lifecycle: PylonLifecycleState
  displayName: string | null
  resourceMode: string
  capabilityRefs: string[]
  blockerRefs: string[]
  updatedAt: string
}

export type PylonLocalState = {
  schema: "openagents.pylon.local_state.v0.3"
  packageName: "@openagentsinc/pylon"
  version: "0.3.0-rc1"
  paths: PylonPaths
  identity: PylonIdentity
  runtime: PylonRuntimeState
}

export type PublicProjection =
  | {
      kind: "identity"
      identity: PylonIdentity
    }
  | {
      kind: "availability"
      pylonRef: string
      lifecycle: PylonLifecycleState
      resourceMode: string
      capabilityRefs: string[]
      blockerRefs: string[]
    }
  | {
      kind: "status"
      state: PylonLocalState
    }
  | Record<string, unknown>

const forbiddenKeyPattern =
  /(^|[._-])(wallet_seed|seed|mnemonic|private_key|privatekey|preimage|bearer|access_token|api_key|apikey|provider_token|provider_auth|raw_prompt|raw_prompts|private_repo|repo_content|private_topology|capacity_pool_secret|internal_accounting_credential|secret|password|xprv)([._-]|$)/i

const forbiddenExactKeyPattern =
  /^(walletSeed|seed|mnemonic|privateKey|private_key|preimage|bearer|accessToken|apiKey|providerToken|providerAuth|rawPrompt|rawPrompts|privateRepo|repoContent|privateTopology|capacityPoolSecret|internalAccountingCredential|secret|password|xprv)$/i

const forbiddenStringPattern =
  /\b(wallet seed|mnemonic|private key|payment preimage|bearer\s+[a-z0-9._-]+|sk-[a-z0-9_-]+|private-repo:\/\/|private_repo|raw prompt|capacity pool secret|internal accounting credential|xprv)\b/i

function stableHash(input: string, length = 24) {
  return createHash("sha256").update(input).digest("hex").slice(0, length)
}

function sanitizeLabel(value: string) {
  const sanitized = value.toLowerCase().replace(/[^a-z0-9._-]+/g, "-").replace(/^-+|-+$/g, "")
  return sanitized || "pylon-node"
}

export function resolveStatePaths(paths: BootstrapSummary["paths"]): PylonPaths {
  return {
    ...paths,
    identity: `${paths.home}/identity.json`,
    runtimeState: `${paths.home}/runtime-state.json`,
    ledger: `${paths.home}/ledger.jsonl`,
  }
}

export async function ensureStateDirectories(paths: PylonPaths) {
  await mkdir(paths.home, { recursive: true })
  await mkdir(paths.cache, { recursive: true })
  await mkdir(paths.releases, { recursive: true })
  await mkdir(dirname(paths.ledger), { recursive: true })
}

export function createPylonIdentity(input: { nodeLabel?: string; pylonRef?: string; now?: Date } = {}) {
  const keyPair = generateKeyPairSync("ed25519", {
    publicKeyEncoding: { type: "spki", format: "der" },
    privateKeyEncoding: { type: "pkcs8", format: "pem" },
  })
  const publicKey = Buffer.from(keyPair.publicKey).toString("base64url")
  const nodeId = `pylon_${stableHash(publicKey)}`
  const nodeLabel = sanitizeLabel(input.nodeLabel ?? hostname())
  const pylonRef = input.pylonRef ?? `pylon.${stableHash(`${nodeLabel}:${publicKey}`, 20)}`
  const npub = `npub1${stableHash(publicKey, 52)}`
  const createdAt = (input.now ?? new Date()).toISOString()

  return {
    nodeId,
    pylonRef,
    nodeLabel,
    publicKey,
    npub,
    createdAt,
    privateKeyPem: keyPair.privateKey,
  } satisfies PylonPrivateIdentityRecord
}

function publicIdentity(record: PylonPrivateIdentityRecord): PylonIdentity {
  const { privateKeyPem: _privateKeyPem, ...identity } = record
  return identity
}

async function readJsonFile<T>(path: string): Promise<T | null> {
  if (!existsSync(path)) return null
  return JSON.parse(await readFile(path, "utf8")) as T
}

export async function loadOrCreateIdentity(paths: PylonPaths, input: { nodeLabel?: string; pylonRef?: string } = {}) {
  await ensureStateDirectories(paths)
  const existing = await readJsonFile<PylonPrivateIdentityRecord>(paths.identity)
  if (existing) return publicIdentity(existing)

  const identity = createPylonIdentity(input)
  await writeFile(paths.identity, `${JSON.stringify(identity, null, 2)}\n`, { mode: 0o600 })
  return publicIdentity(identity)
}

export async function loadOrCreateRuntimeState(
  paths: PylonPaths,
  input: Partial<Pick<PylonRuntimeState, "displayName" | "resourceMode" | "capabilityRefs">> = {},
) {
  await ensureStateDirectories(paths)
  const existing = await readJsonFile<PylonRuntimeState>(paths.runtimeState)
  const state: PylonRuntimeState = {
    lifecycle: existing?.lifecycle ?? "offline",
    displayName: input.displayName ?? existing?.displayName ?? null,
    resourceMode: input.resourceMode ?? existing?.resourceMode ?? "background_20",
    capabilityRefs: input.capabilityRefs ?? existing?.capabilityRefs ?? [],
    blockerRefs: existing?.blockerRefs ?? [],
    updatedAt: new Date().toISOString(),
  }
  await writeFile(paths.runtimeState, `${JSON.stringify(state, null, 2)}\n`)
  return state
}

export async function ensurePylonLocalState(summary: BootstrapSummary): Promise<PylonLocalState> {
  const paths = resolveStatePaths(summary.paths)
  const identity = await loadOrCreateIdentity(paths, {
    nodeLabel: summary.bootstrap.displayName ?? undefined,
    pylonRef: summary.bootstrap.pylonRef ?? undefined,
  })
  const runtime = await loadOrCreateRuntimeState(paths, {
    displayName: summary.bootstrap.displayName,
    resourceMode: summary.bootstrap.resourceMode,
    capabilityRefs: summary.bootstrap.capabilityRefs,
  })

  return {
    schema: "openagents.pylon.local_state.v0.3",
    packageName: "@openagentsinc/pylon",
    version: "0.3.0-rc1",
    paths,
    identity,
    runtime,
  }
}

export function assertPublicProjectionSafe(value: unknown, path = "projection"): asserts value is PublicProjection {
  if (value === null || value === undefined) return
  if (typeof value === "string") {
    if (forbiddenStringPattern.test(value)) {
      throw new Error(`${path} contains private-data-shaped text`)
    }
    return
  }
  if (typeof value !== "object") return

  for (const [key, child] of Object.entries(value as Record<string, unknown>)) {
    if (forbiddenKeyPattern.test(key) || forbiddenExactKeyPattern.test(key)) {
      throw new Error(`${path}.${key} is not public-safe`)
    }
    assertPublicProjectionSafe(child, `${path}.${key}`)
  }
}

export function projectPublicStatus(state: PylonLocalState) {
  const projection = {
    kind: "status",
    state: {
      schema: state.schema,
      packageName: state.packageName,
      version: state.version,
      paths: {
        home: state.paths.home,
        config: state.paths.config,
        cache: state.paths.cache,
        releases: state.paths.releases,
      },
      identity: state.identity,
      runtime: state.runtime,
    },
  } as const

  assertPublicProjectionSafe(projection)
  return projection
}

