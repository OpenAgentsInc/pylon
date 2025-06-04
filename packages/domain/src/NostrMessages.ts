import { Schema } from "effect"
import { NostrEvent, EventId } from "./NostrEvent.js"

// Subscription ID type
export const SubscriptionId = Schema.String.pipe(
  Schema.minLength(1),
  Schema.maxLength(64),
  Schema.brand("SubscriptionId")
)
export type SubscriptionId = typeof SubscriptionId.Type

// Filter interface for REQ messages - using a simpler approach
export interface Filter {
  readonly ids?: readonly EventId[]
  readonly authors?: readonly string[]
  readonly kinds?: readonly number[]
  readonly since?: number
  readonly until?: number
  readonly limit?: number
  readonly [key: `#${string}`]: readonly string[] | undefined
}

// Schema for validating Filter objects
export const FilterSchema = Schema.Struct({
  ids: Schema.optional(Schema.Array(EventId)),
  authors: Schema.optional(Schema.Array(Schema.String.pipe(Schema.pattern(/^[a-f0-9]{64}$/)))),
  kinds: Schema.optional(Schema.Array(Schema.Number.pipe(Schema.int(), Schema.between(0, 65535)))),
  since: Schema.optional(Schema.Number.pipe(Schema.int(), Schema.positive())),
  until: Schema.optional(Schema.Number.pipe(Schema.int(), Schema.positive())),
  limit: Schema.optional(Schema.Number.pipe(Schema.int(), Schema.positive()))
})

// Client to Relay messages
export const ClientEventMessage = Schema.Tuple(
  Schema.Literal("EVENT"),
  NostrEvent
)
export type ClientEventMessage = typeof ClientEventMessage.Type

// REQ message: ["REQ", subscription_id, filter1, filter2, ...]
export const ClientReqMessage = Schema.Array(
  Schema.Unknown
).pipe(
  Schema.minItems(3),
  Schema.filter(
    (arr): arr is [string, string, ...unknown[]] => 
      arr.length >= 3 && arr[0] === "REQ" && typeof arr[1] === "string"
  )
)
export type ClientReqMessage = typeof ClientReqMessage.Type

export const ClientCloseMessage = Schema.Tuple(
  Schema.Literal("CLOSE"),
  SubscriptionId
)
export type ClientCloseMessage = typeof ClientCloseMessage.Type

export const ClientMessage = Schema.Union(
  ClientEventMessage,
  ClientReqMessage,
  ClientCloseMessage
)
export type ClientMessage = typeof ClientMessage.Type

// Relay to Client messages
export const RelayEventMessage = Schema.Tuple(
  Schema.Literal("EVENT"),
  SubscriptionId,
  NostrEvent
)
export type RelayEventMessage = typeof RelayEventMessage.Type

export const RelayOkMessage = Schema.Tuple(
  Schema.Literal("OK"),
  EventId,
  Schema.Boolean,
  Schema.String
)
export type RelayOkMessage = typeof RelayOkMessage.Type

export const RelayEoseMessage = Schema.Tuple(
  Schema.Literal("EOSE"),
  SubscriptionId
)
export type RelayEoseMessage = typeof RelayEoseMessage.Type

export const RelayClosedMessage = Schema.Tuple(
  Schema.Literal("CLOSED"),
  SubscriptionId,
  Schema.String
)
export type RelayClosedMessage = typeof RelayClosedMessage.Type

export const RelayNoticeMessage = Schema.Tuple(
  Schema.Literal("NOTICE"),
  Schema.String
)
export type RelayNoticeMessage = typeof RelayNoticeMessage.Type

export const RelayMessage = Schema.Union(
  RelayEventMessage,
  RelayOkMessage,
  RelayEoseMessage,
  RelayClosedMessage,
  RelayNoticeMessage
)
export type RelayMessage = typeof RelayMessage.Type

// Error types for relay operations
export class EventValidationError extends Schema.TaggedError<EventValidationError>()("EventValidationError", {
  eventId: Schema.optional(EventId),
  reason: Schema.String
}) {}

export class EventNotFound extends Schema.TaggedError<EventNotFound>()("EventNotFound", {
  eventId: EventId
}) {}

export class SubscriptionError extends Schema.TaggedError<SubscriptionError>()("SubscriptionError", {
  subscriptionId: SubscriptionId,
  reason: Schema.String
}) {}

export class RateLimitError extends Schema.TaggedError<RateLimitError>()("RateLimitError", {
  reason: Schema.String
}) {}

export class AuthenticationError extends Schema.TaggedError<AuthenticationError>()("AuthenticationError", {
  reason: Schema.String
}) {}

// Standard OK message prefixes according to NIP-01
export type OkPrefix = 
  | "duplicate"
  | "pow" 
  | "blocked"
  | "rate-limited"
  | "invalid"
  | "restricted"
  | "error"

// Standard CLOSED message prefixes
export type ClosedPrefix =
  | "unsupported"
  | "error"

// Helper functions for creating standard responses
export const createOkMessage = (
  eventId: EventId, 
  accepted: boolean, 
  prefix?: OkPrefix, 
  message = ""
): RelayOkMessage => {
  const fullMessage = prefix ? `${prefix}: ${message}` : message
  return [
    "OK",
    eventId,
    accepted,
    fullMessage
  ]
}

export const createClosedMessage = (
  subscriptionId: SubscriptionId,
  prefix?: ClosedPrefix,
  message = ""
): RelayClosedMessage => {
  const fullMessage = prefix ? `${prefix}: ${message}` : message
  return [
    "CLOSED",
    subscriptionId, 
    fullMessage
  ]
}