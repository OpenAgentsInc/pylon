import { describe, test, expect } from "vitest"
import { 
  NostrEvent, 
  EventId, 
  PubKey, 
  Signature,
  UnixTimestamp,
  EventKind,
  UserMetadataEvent,
  TextNoteEvent,
  ContactListEvent,
  DeletionEvent
} from "../src/NostrEvent.js"

describe("NostrEvent", () => {
  describe("Branded Types", () => {
    test("EventId should accept valid 64-char hex", () => {
      const validId = "a".repeat(64)
      expect(() => EventId.make(validId)).not.toThrow()
    })

    test("EventId should reject invalid formats", () => {
      expect(() => EventId.make("invalid")).toThrow()
      expect(() => EventId.make("a".repeat(63))).toThrow()
      expect(() => EventId.make("a".repeat(65))).toThrow()
      expect(() => EventId.make("ABCD" + "a".repeat(60))).toThrow() // uppercase
    })

    test("PubKey should accept valid 64-char hex", () => {
      const validPubkey = "b".repeat(64)
      expect(() => PubKey.make(validPubkey)).not.toThrow()
    })

    test("EventKind should accept valid range", () => {
      expect(() => EventKind.make(0)).not.toThrow()
      expect(() => EventKind.make(1)).not.toThrow()
      expect(() => EventKind.make(65535)).not.toThrow()
    })

    test("EventKind should reject invalid range", () => {
      expect(() => EventKind.make(-1)).toThrow()
      expect(() => EventKind.make(65536)).toThrow()
      expect(() => EventKind.make(1.5)).toThrow()
    })
  })

  describe("Event ID Calculation", () => {
    test("should calculate correct event ID", () => {
      const pubkey = PubKey.make("a".repeat(64))
      const created_at = UnixTimestamp.make(1609459200) // 2021-01-01 00:00:00 UTC
      const kind = EventKind.make(1)
      const tags: string[][] = [["e", "b".repeat(64)]]
      const content = "Hello, Nostr!"

      const eventId = NostrEvent.calculateId(pubkey, created_at, kind, tags, content)
      
      // The ID should be a valid 64-char hex string
      expect(eventId).toMatch(/^[a-f0-9]{64}$/)
      expect(eventId.length).toBe(64)
    })

    test("should produce different IDs for different content", () => {
      const pubkey = PubKey.make("a".repeat(64))
      const created_at = UnixTimestamp.make(1609459200)
      const kind = EventKind.make(1)
      const tags: string[][] = []

      const id1 = NostrEvent.calculateId(pubkey, created_at, kind, tags, "content1")
      const id2 = NostrEvent.calculateId(pubkey, created_at, kind, tags, "content2")
      
      expect(id1).not.toBe(id2)
    })

    test("should produce same ID for identical inputs", () => {
      const pubkey = PubKey.make("a".repeat(64))
      const created_at = UnixTimestamp.make(1609459200)
      const kind = EventKind.make(1)
      const tags: string[][] = [["p", "c".repeat(64)]]
      const content = "Same content"

      const id1 = NostrEvent.calculateId(pubkey, created_at, kind, tags, content)
      const id2 = NostrEvent.calculateId(pubkey, created_at, kind, tags, content)
      
      expect(id1).toBe(id2)
    })
  })

  describe("Kind Classification", () => {
    test("should classify regular events correctly", () => {
      const regularEvent = createTestEvent({ kind: EventKind.make(1) })
      expect(regularEvent.getKindClassification()).toBe("regular")

      const regularEvent2 = createTestEvent({ kind: EventKind.make(1000) })
      expect(regularEvent2.getKindClassification()).toBe("regular")

      const regularEvent3 = createTestEvent({ kind: EventKind.make(2) })
      expect(regularEvent3.getKindClassification()).toBe("regular")
    })

    test("should classify replaceable events correctly", () => {
      const replaceableEvent = createTestEvent({ kind: EventKind.make(0) })
      expect(replaceableEvent.getKindClassification()).toBe("replaceable")

      const replaceableEvent2 = createTestEvent({ kind: EventKind.make(3) })
      expect(replaceableEvent2.getKindClassification()).toBe("replaceable")

      const replaceableEvent3 = createTestEvent({ kind: EventKind.make(10000) })
      expect(replaceableEvent3.getKindClassification()).toBe("replaceable")
    })

    test("should classify ephemeral events correctly", () => {
      const ephemeralEvent = createTestEvent({ kind: EventKind.make(20000) })
      expect(ephemeralEvent.getKindClassification()).toBe("ephemeral")

      const ephemeralEvent2 = createTestEvent({ kind: EventKind.make(25000) })
      expect(ephemeralEvent2.getKindClassification()).toBe("ephemeral")

      const ephemeralEvent3 = createTestEvent({ kind: EventKind.make(29999) })
      expect(ephemeralEvent3.getKindClassification()).toBe("ephemeral")
    })

    test("should classify addressable events correctly", () => {
      const addressableEvent = createTestEvent({ kind: EventKind.make(30000) })
      expect(addressableEvent.getKindClassification()).toBe("addressable")

      const addressableEvent2 = createTestEvent({ kind: EventKind.make(35000) })
      expect(addressableEvent2.getKindClassification()).toBe("addressable")

      const addressableEvent3 = createTestEvent({ kind: EventKind.make(39999) })
      expect(addressableEvent3.getKindClassification()).toBe("addressable")
    })
  })

  describe("Tag Operations", () => {
    test("should extract d tag for addressable events", () => {
      const addressableEvent = createTestEvent({ 
        kind: EventKind.make(30000),
        tags: [["d", "test-identifier"], ["p", "a".repeat(64)]]
      })
      
      expect(addressableEvent.getDTag()).toBe("test-identifier")
    })

    test("should return empty string for addressable events without d tag", () => {
      const addressableEvent = createTestEvent({ 
        kind: EventKind.make(30000),
        tags: [["p", "a".repeat(64)]]
      })
      
      expect(addressableEvent.getDTag()).toBe("")
    })

    test("should return undefined for non-addressable events", () => {
      const regularEvent = createTestEvent({ 
        kind: EventKind.make(1),
        tags: [["d", "test-identifier"]]
      })
      
      expect(regularEvent.getDTag()).toBeUndefined()
    })

    test("should get tag values by name", () => {
      const event = createTestEvent({
        tags: [
          ["e", "event1", "relay1"],
          ["e", "event2"],
          ["p", "pubkey1"],
          ["p", "pubkey2"]
        ]
      })

      expect(event.getTagValues("e")).toEqual(["event1", "event2"])
      expect(event.getTagValues("p")).toEqual(["pubkey1", "pubkey2"])
      expect(event.getTagValues("nonexistent")).toEqual([])
    })
  })

  describe("Specific Event Types", () => {
    test("UserMetadataEvent should parse content as JSON", () => {
      const metadata = {
        name: "Alice",
        about: "Nostr enthusiast",
        picture: "https://example.com/alice.jpg"
      }

      const event = new UserMetadataEvent({
        id: EventId.make("a".repeat(64)),
        pubkey: PubKey.make("b".repeat(64)),
        created_at: UnixTimestamp.make(1609459200),
        kind: 0,
        tags: [],
        content: metadata,
        sig: Signature.make("c".repeat(128))
      })

      expect(event.content).toEqual(metadata)
      expect(event.kind).toBe(0)
    })

    test("TextNoteEvent should have kind 1", () => {
      const event = new TextNoteEvent({
        id: EventId.make("a".repeat(64)),
        pubkey: PubKey.make("b".repeat(64)),
        created_at: UnixTimestamp.make(1609459200),
        kind: 1,
        tags: [],
        content: "Hello, world!",
        sig: Signature.make("c".repeat(128))
      })

      expect(event.kind).toBe(1)
      expect(event.content).toBe("Hello, world!")
    })

    test("ContactListEvent should have kind 3", () => {
      const event = new ContactListEvent({
        id: EventId.make("a".repeat(64)),
        pubkey: PubKey.make("b".repeat(64)),
        created_at: UnixTimestamp.make(1609459200),
        kind: 3,
        tags: [["p", "friend1"], ["p", "friend2"]],
        content: "",
        sig: Signature.make("c".repeat(128))
      })

      expect(event.kind).toBe(3)
    })

    test("DeletionEvent should have kind 5", () => {
      const event = new DeletionEvent({
        id: EventId.make("a".repeat(64)),
        pubkey: PubKey.make("b".repeat(64)),
        created_at: UnixTimestamp.make(1609459200),
        kind: 5,
        tags: [["e", "event-to-delete"]],
        content: "Deleting this event",
        sig: Signature.make("c".repeat(128))
      })

      expect(event.kind).toBe(5)
    })
  })
})

// Helper function to create test events
function createTestEvent(overrides: Partial<{
  id: string
  pubkey: string
  created_at: number
  kind: number
  tags: string[][]
  content: string
  sig: string
}> = {}): NostrEvent {
  return new NostrEvent({
    id: EventId.make(overrides.id || "a".repeat(64)),
    pubkey: PubKey.make(overrides.pubkey || "b".repeat(64)),
    created_at: UnixTimestamp.make(overrides.created_at || 1609459200),
    kind: EventKind.make(overrides.kind || 1),
    tags: overrides.tags || [],
    content: overrides.content || "test content",
    sig: Signature.make(overrides.sig || "c".repeat(128))
  })
}