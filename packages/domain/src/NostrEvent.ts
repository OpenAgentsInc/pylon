import { Schema } from "effect"
import { createHash } from "crypto"
import * as secp256k1 from "@noble/secp256k1"

// Branded types for type safety
export const EventId = Schema.String.pipe(
  Schema.pattern(/^[a-f0-9]{64}$/),
  Schema.brand("EventId")
)
export type EventId = typeof EventId.Type

export const PubKey = Schema.String.pipe(
  Schema.pattern(/^[a-f0-9]{64}$/),
  Schema.brand("PubKey")
)
export type PubKey = typeof PubKey.Type

export const Signature = Schema.String.pipe(
  Schema.pattern(/^[a-f0-9]{128}$/),
  Schema.brand("Signature")
)
export type Signature = typeof Signature.Type

export const UnixTimestamp = Schema.Number.pipe(
  Schema.int(),
  Schema.positive(),
  Schema.brand("UnixTimestamp")
)
export type UnixTimestamp = typeof UnixTimestamp.Type

export const EventKind = Schema.Number.pipe(
  Schema.int(),
  Schema.between(0, 65535),
  Schema.brand("EventKind")
)
export type EventKind = typeof EventKind.Type

// Tag schema - array of strings
export const Tag = Schema.Array(Schema.String).pipe(
  Schema.minItems(1)
)
export type Tag = typeof Tag.Type

// Content must be a string (can be empty)
export const Content = Schema.String
export type Content = typeof Content.Type

// Core Nostr Event schema
export class NostrEvent extends Schema.Class<NostrEvent>("NostrEvent")({
  id: EventId,
  pubkey: PubKey,
  created_at: UnixTimestamp,
  kind: EventKind,
  tags: Schema.Array(Tag),
  content: Content,
  sig: Signature
}) {
  /**
   * Calculate the event ID by serializing and hashing the event data
   * according to NIP-01 specification
   */
  static calculateId(
    pubkey: PubKey,
    created_at: UnixTimestamp,
    kind: EventKind,
    tags: readonly (readonly string[])[],
    content: Content
  ): EventId {
    // Serialize according to NIP-01: [0, pubkey, created_at, kind, tags, content]
    const serialized = JSON.stringify([
      0,
      pubkey,
      created_at,
      kind,
      tags,
      content
    ])
    
    // SHA256 hash
    const hash = createHash('sha256')
      .update(serialized, 'utf8')
      .digest('hex')
    
    return EventId.make(hash)
  }

  /**
   * Verify the event signature using secp256k1 schnorr
   */
  static async verifySignature(event: NostrEvent): Promise<boolean> {
    try {
      // Calculate expected ID
      const expectedId = NostrEvent.calculateId(
        event.pubkey,
        event.created_at,
        event.kind,
        event.tags,
        event.content
      )

      // Verify the ID matches
      if (event.id !== expectedId) {
        return false
      }

      // Verify the schnorr signature
      return secp256k1.schnorr.verify(event.sig, event.id, event.pubkey)
    } catch {
      return false
    }
  }

  /**
   * Get the classification of this event kind according to NIP-01
   */
  getKindClassification(): 'regular' | 'replaceable' | 'ephemeral' | 'addressable' {
    const k = this.kind

    // Ephemeral events: 20000 <= n < 30000
    if (k >= 20000 && k < 30000) {
      return 'ephemeral'
    }

    // Addressable events: 30000 <= n < 40000
    if (k >= 30000 && k < 40000) {
      return 'addressable'
    }

    // Replaceable events: 10000 <= n < 20000 || n == 0 || n == 3
    if ((k >= 10000 && k < 20000) || k === 0 || k === 3) {
      return 'replaceable'
    }

    // Regular events: everything else
    // 1000 <= n < 10000 || 4 <= n < 45 || n == 1 || n == 2
    return 'regular'
  }

  /**
   * Get the 'd' tag value for addressable events
   */
  getDTag(): string | undefined {
    if (this.getKindClassification() !== 'addressable') {
      return undefined
    }

    const dTag = this.tags.find(tag => tag[0] === 'd')
    return dTag?.[1] || ''
  }

  /**
   * Get all tag values for a given tag name
   */
  getTagValues(tagName: string): string[] {
    return this.tags
      .filter(tag => tag[0] === tagName)
      .map(tag => tag[1])
      .filter(Boolean)
  }
}

// User metadata event (kind 0)
export const UserMetadata = Schema.Struct({
  name: Schema.optional(Schema.String),
  about: Schema.optional(Schema.String),
  picture: Schema.optional(Schema.String),
  // Allow additional fields
}).pipe(Schema.extend(Schema.Record({ key: Schema.String, value: Schema.Unknown })))
export type UserMetadata = typeof UserMetadata.Type

// Create specific event types using composition instead of extension
export class UserMetadataEvent extends Schema.Class<UserMetadataEvent>("UserMetadataEvent")({
  id: EventId,
  pubkey: PubKey,
  created_at: UnixTimestamp,
  kind: Schema.Literal(0),
  tags: Schema.Array(Tag),
  content: Schema.transform(
    Schema.String,
    UserMetadata,
    {
      strict: true,
      decode: (s) => JSON.parse(s),
      encode: (obj) => JSON.stringify(obj)
    }
  ),
  sig: Signature
}) {
  getKindClassification(): 'regular' | 'replaceable' | 'ephemeral' | 'addressable' {
    return new NostrEvent({
      id: this.id,
      pubkey: this.pubkey,
      created_at: this.created_at,
      kind: this.kind as any,
      tags: this.tags,
      content: JSON.stringify(this.content),
      sig: this.sig
    }).getKindClassification()
  }

  getDTag(): string | undefined {
    return new NostrEvent({
      id: this.id,
      pubkey: this.pubkey,
      created_at: this.created_at,
      kind: this.kind as any,
      tags: this.tags,
      content: JSON.stringify(this.content),
      sig: this.sig
    }).getDTag()
  }

  getTagValues(tagName: string): string[] {
    return new NostrEvent({
      id: this.id,
      pubkey: this.pubkey,
      created_at: this.created_at,
      kind: this.kind as any,
      tags: this.tags,
      content: JSON.stringify(this.content),
      sig: this.sig
    }).getTagValues(tagName)
  }
}

// Text note event (kind 1)
export class TextNoteEvent extends Schema.Class<TextNoteEvent>("TextNoteEvent")({
  id: EventId,
  pubkey: PubKey,
  created_at: UnixTimestamp,
  kind: Schema.Literal(1),
  tags: Schema.Array(Tag),
  content: Content,
  sig: Signature
}) {}

// Contact list event (kind 3)
export class ContactListEvent extends Schema.Class<ContactListEvent>("ContactListEvent")({
  id: EventId,
  pubkey: PubKey,
  created_at: UnixTimestamp,
  kind: Schema.Literal(3),
  tags: Schema.Array(Tag),
  content: Content,
  sig: Signature
}) {}

// Event deletion event (kind 5)
export class DeletionEvent extends Schema.Class<DeletionEvent>("DeletionEvent")({
  id: EventId,
  pubkey: PubKey,
  created_at: UnixTimestamp,
  kind: Schema.Literal(5),
  tags: Schema.Array(Tag),
  content: Content,
  sig: Signature
}) {}