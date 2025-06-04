import { describe, test, expect } from "vitest"
import { NostrFilter } from "../src/NostrFilter.js"
import { NostrEvent, EventId, PubKey, UnixTimestamp, EventKind } from "../src/NostrEvent.js"
import { Filter } from "../src/NostrMessages.js"

describe("NostrFilter", () => {
  // Helper function to create test events
  function createTestEvent(overrides: Partial<{
    id: string
    pubkey: string
    created_at: number
    kind: number
    tags: string[][]
    content: string
  }> = {}): NostrEvent {
    return new NostrEvent({
      id: EventId.make(overrides.id || "a".repeat(64)),
      pubkey: PubKey.make(overrides.pubkey || "b".repeat(64)),
      created_at: UnixTimestamp.make(overrides.created_at || 1609459200),
      kind: EventKind.make(overrides.kind || 1),
      tags: overrides.tags || [],
      content: overrides.content || "test content",
      sig: "c".repeat(128) as any
    })
  }

  describe("matchesFilter", () => {
    test("should match when no filters are specified", () => {
      const event = createTestEvent()
      const filter: Filter = {}
      
      expect(NostrFilter.matchesFilter(event, filter)).toBe(true)
    })

    test("should match by event ID", () => {
      const eventId = "a".repeat(64)
      const event = createTestEvent({ id: eventId })
      const filter: Filter = { ids: [EventId.make(eventId)] }
      
      expect(NostrFilter.matchesFilter(event, filter)).toBe(true)
    })

    test("should not match incorrect event ID", () => {
      const event = createTestEvent({ id: "a".repeat(64) })
      const filter: Filter = { ids: [EventId.make("b".repeat(64))] }
      
      expect(NostrFilter.matchesFilter(event, filter)).toBe(false)
    })

    test("should match by author pubkey", () => {
      const pubkey = "b".repeat(64)
      const event = createTestEvent({ pubkey })
      const filter: Filter = { authors: [pubkey] }
      
      expect(NostrFilter.matchesFilter(event, filter)).toBe(true)
    })

    test("should not match incorrect author", () => {
      const event = createTestEvent({ pubkey: "a".repeat(64) })
      const filter: Filter = { authors: ["b".repeat(64)] }
      
      expect(NostrFilter.matchesFilter(event, filter)).toBe(false)
    })

    test("should match by kind", () => {
      const event = createTestEvent({ kind: 1 })
      const filter: Filter = { kinds: [1] }
      
      expect(NostrFilter.matchesFilter(event, filter)).toBe(true)
    })

    test("should match multiple kinds", () => {
      const event = createTestEvent({ kind: 1 })
      const filter: Filter = { kinds: [0, 1, 3] }
      
      expect(NostrFilter.matchesFilter(event, filter)).toBe(true)
    })

    test("should not match incorrect kind", () => {
      const event = createTestEvent({ kind: 1 })
      const filter: Filter = { kinds: [0, 3] }
      
      expect(NostrFilter.matchesFilter(event, filter)).toBe(false)
    })

    test("should match by since timestamp", () => {
      const event = createTestEvent({ created_at: 1609459200 })
      const filter: Filter = { since: 1609459100 }
      
      expect(NostrFilter.matchesFilter(event, filter)).toBe(true)
    })

    test("should not match events before since", () => {
      const event = createTestEvent({ created_at: 1609459200 })
      const filter: Filter = { since: 1609459300 }
      
      expect(NostrFilter.matchesFilter(event, filter)).toBe(false)
    })

    test("should match by until timestamp", () => {
      const event = createTestEvent({ created_at: 1609459200 })
      const filter: Filter = { until: 1609459300 }
      
      expect(NostrFilter.matchesFilter(event, filter)).toBe(true)
    })

    test("should not match events after until", () => {
      const event = createTestEvent({ created_at: 1609459200 })
      const filter: Filter = { until: 1609459100 }
      
      expect(NostrFilter.matchesFilter(event, filter)).toBe(false)
    })

    test("should match time range", () => {
      const event = createTestEvent({ created_at: 1609459200 })
      const filter: Filter = { since: 1609459100, until: 1609459300 }
      
      expect(NostrFilter.matchesFilter(event, filter)).toBe(true)
    })

    test("should match by tag filters", () => {
      const event = createTestEvent({ 
        tags: [["e", "event123"], ["p", "pubkey456"]] 
      })
      const filter: Filter = { "#e": ["event123"] }
      
      expect(NostrFilter.matchesFilter(event, filter)).toBe(true)
    })

    test("should match multiple tag values", () => {
      const event = createTestEvent({ 
        tags: [["p", "pubkey456"]] 
      })
      const filter: Filter = { "#p": ["pubkey123", "pubkey456", "pubkey789"] }
      
      expect(NostrFilter.matchesFilter(event, filter)).toBe(true)
    })

    test("should not match incorrect tag values", () => {
      const event = createTestEvent({ 
        tags: [["p", "pubkey456"]] 
      })
      const filter: Filter = { "#p": ["pubkey123", "pubkey789"] }
      
      expect(NostrFilter.matchesFilter(event, filter)).toBe(false)
    })

    test("should match complex filter with multiple conditions", () => {
      const event = createTestEvent({ 
        kind: 1,
        created_at: 1609459200,
        tags: [["p", "pubkey456"]]
      })
      const filter: Filter = { 
        kinds: [1], 
        since: 1609459100, 
        until: 1609459300,
        "#p": ["pubkey456"]
      }
      
      expect(NostrFilter.matchesFilter(event, filter)).toBe(true)
    })

    test("should fail complex filter if any condition fails", () => {
      const event = createTestEvent({ 
        kind: 1,
        created_at: 1609459200,
        tags: [["p", "pubkey456"]]
      })
      const filter: Filter = { 
        kinds: [1], 
        since: 1609459100, 
        until: 1609459300,
        "#p": ["pubkey999"] // This will fail
      }
      
      expect(NostrFilter.matchesFilter(event, filter)).toBe(false)
    })
  })

  describe("matchesAnyFilter", () => {
    test("should match if any filter matches", () => {
      const event = createTestEvent({ kind: 1 })
      const filters: Filter[] = [
        { kinds: [0] }, // Won't match
        { kinds: [1] }, // Will match
        { kinds: [3] }  // Won't match
      ]
      
      expect(NostrFilter.matchesAnyFilter(event, filters)).toBe(true)
    })

    test("should not match if no filters match", () => {
      const event = createTestEvent({ kind: 1 })
      const filters: Filter[] = [
        { kinds: [0] },
        { kinds: [3] },
        { kinds: [5] }
      ]
      
      expect(NostrFilter.matchesAnyFilter(event, filters)).toBe(false)
    })
  })

  describe("validateFilter", () => {
    test("should validate correct filter", () => {
      const filter: Filter = {
        ids: [EventId.make("a".repeat(64))],
        authors: ["b".repeat(64)],
        kinds: [0, 1, 3],
        since: 1609459200,
        until: 1609459300,
        limit: 100
      }
      
      const errors = NostrFilter.validateFilter(filter)
      expect(errors).toHaveLength(0)
    })

    test("should detect invalid event ID format", () => {
      const filter = {
        ids: ["invalid" as any] // Raw invalid ID to test validation
      } as Filter
      
      const errors = NostrFilter.validateFilter(filter)
      expect(errors.length).toBeGreaterThan(0)
      expect(errors[0]).toContain("Invalid event ID format")
    })

    test("should detect invalid pubkey format", () => {
      const filter = {
        authors: ["invalid"] // Raw invalid pubkey
      } as Filter
      
      const errors = NostrFilter.validateFilter(filter)
      expect(errors.length).toBeGreaterThan(0)
      expect(errors[0]).toContain("Invalid pubkey format")
    })

    test("should detect invalid time range", () => {
      const filter: Filter = {
        since: 1609459300,
        until: 1609459200 // until is before since
      }
      
      const errors = NostrFilter.validateFilter(filter)
      expect(errors.length).toBeGreaterThan(0)
      expect(errors[0]).toContain("Invalid time range")
    })

    test("should validate tag filters", () => {
      const filter: Filter = {
        "#e": ["a".repeat(64)],
        "#p": ["b".repeat(64)]
      }
      
      const errors = NostrFilter.validateFilter(filter)
      expect(errors).toHaveLength(0)
    })

    test("should detect invalid tag values", () => {
      const filter = {
        "#e": ["invalid"],
        "#p": ["also-invalid"]
      } as Filter
      
      const errors = NostrFilter.validateFilter(filter)
      expect(errors.length).toBeGreaterThan(0)
    })
  })

  describe("Kind classification helpers", () => {
    test("should identify replaceable filter", () => {
      const filter: Filter = { kinds: [0, 3, 10000] }
      expect(NostrFilter.isReplaceableFilter(filter)).toBe(true)
    })

    test("should identify non-replaceable filter", () => {
      const filter: Filter = { kinds: [1, 2] }
      expect(NostrFilter.isReplaceableFilter(filter)).toBe(false)
    })

    test("should identify addressable filter", () => {
      const filter: Filter = { kinds: [30000, 35000] }
      expect(NostrFilter.isAddressableFilter(filter)).toBe(true)
    })

    test("should identify non-addressable filter", () => {
      const filter: Filter = { kinds: [1, 10000] }
      expect(NostrFilter.isAddressableFilter(filter)).toBe(false)
    })
  })

  describe("getTagFilters", () => {
    test("should extract tag filters", () => {
      const filter: Filter = {
        kinds: [1],
        "#e": ["event1", "event2"],
        "#p": ["pubkey1"],
        "#t": ["hashtag"]
      }
      
      const tagFilters = NostrFilter.getTagFilters(filter)
      expect(tagFilters).toEqual({
        "e": ["event1", "event2"],
        "p": ["pubkey1"],
        "t": ["hashtag"]
      })
    })

    test("should return empty object for no tag filters", () => {
      const filter: Filter = { kinds: [1], since: 123456 }
      const tagFilters = NostrFilter.getTagFilters(filter)
      expect(tagFilters).toEqual({})
    })
  })

  describe("normalizeFilter", () => {
    test("should normalize filter by sorting arrays", () => {
      const filter: Filter = {
        ids: [EventId.make("b".repeat(64)), EventId.make("a".repeat(64))],
        authors: ["z".repeat(64), "a".repeat(64)],
        kinds: [3, 1, 0],
        "#e": ["event2", "event1"]
      }
      
      const normalized = NostrFilter.normalizeFilter(filter)
      expect(normalized.ids).toEqual([EventId.make("a".repeat(64)), EventId.make("b".repeat(64))])
      expect(normalized.authors).toEqual(["a".repeat(64), "z".repeat(64)])
      expect(normalized.kinds).toEqual([0, 1, 3])
      expect(normalized["#e"]).toEqual(["event1", "event2"])
    })

    test("should remove empty arrays", () => {
      const filter: Filter = {
        ids: [],
        authors: ["a".repeat(64)],
        kinds: [1]
      }
      
      const normalized = NostrFilter.normalizeFilter(filter)
      expect(normalized.ids).toBeUndefined()
      expect(normalized.authors).toEqual(["a".repeat(64)])
      expect(normalized.kinds).toEqual([1])
    })
  })
})