import { describe, test, expect } from "vitest"
import { 
  SubscriptionId,
  Filter,
  ClientEventMessage,
  ClientCloseMessage,
  RelayEventMessage,
  RelayOkMessage,
  RelayEoseMessage,
  RelayClosedMessage,
  RelayNoticeMessage,
  createOkMessage,
  createClosedMessage,
  EventValidationError,
  EventNotFound,
  SubscriptionError,
  RateLimitError,
  AuthenticationError
} from "../src/NostrMessages.js"
import { NostrEvent, EventId, PubKey, UnixTimestamp, EventKind } from "../src/NostrEvent.js"

describe("NostrMessages", () => {
  // Helper function to create test event
  function createTestEvent(): NostrEvent {
    return new NostrEvent({
      id: EventId.make("a".repeat(64)),
      pubkey: PubKey.make("b".repeat(64)),
      created_at: UnixTimestamp.make(1609459200),
      kind: EventKind.make(1),
      tags: [],
      content: "test content",
      sig: "c".repeat(128) as any
    })
  }

  describe("SubscriptionId", () => {
    test("should accept valid subscription IDs", () => {
      expect(() => SubscriptionId.make("sub1")).not.toThrow()
      expect(() => SubscriptionId.make("a".repeat(64))).not.toThrow()
    })

    test("should reject invalid subscription IDs", () => {
      expect(() => SubscriptionId.make("")).toThrow() // empty
      expect(() => SubscriptionId.make("a".repeat(65))).toThrow() // too long
    })
  })

  describe("Filter", () => {
    test("should accept valid filter", () => {
      const filter: Filter = {
        ids: [EventId.make("a".repeat(64))],
        authors: ["b".repeat(64)],
        kinds: [0, 1, 3],
        since: 1609459200,
        until: 1609459300,
        limit: 100,
        "#e": ["event1", "event2"],
        "#p": ["pubkey1"]
      }
      
      expect(filter.ids).toBeDefined()
      expect(filter.authors).toBeDefined()
      expect(filter.kinds).toBeDefined()
      expect(filter["#e"]).toBeDefined()
      expect(filter["#p"]).toBeDefined()
    })

    test("should accept empty filter", () => {
      const filter: Filter = {}
      expect(Object.keys(filter)).toHaveLength(0)
    })

    test("should accept filter with only some fields", () => {
      const filter: Filter = {
        kinds: [1],
        limit: 50
      }
      
      expect(filter.kinds).toEqual([1])
      expect(filter.limit).toBe(50)
    })
  })

  describe("Client Messages", () => {
    test("should create valid EVENT message", () => {
      const event = createTestEvent()
      const message: ClientEventMessage = ["EVENT", event]
      
      expect(message[0]).toBe("EVENT")
      expect(message[1]).toBe(event)
      expect(message).toHaveLength(2)
    })

    test("should create valid CLOSE message", () => {
      const subId = SubscriptionId.make("sub123")
      const message: ClientCloseMessage = ["CLOSE", subId]
      
      expect(message[0]).toBe("CLOSE")
      expect(message[1]).toBe(subId)
      expect(message).toHaveLength(2)
    })
  })

  describe("Relay Messages", () => {
    test("should create valid relay EVENT message", () => {
      const subId = SubscriptionId.make("sub123")
      const event = createTestEvent()
      const message: RelayEventMessage = ["EVENT", subId, event]
      
      expect(message[0]).toBe("EVENT")
      expect(message[1]).toBe(subId)
      expect(message[2]).toBe(event)
      expect(message).toHaveLength(3)
    })

    test("should create valid OK message for accepted event", () => {
      const eventId = EventId.make("a".repeat(64))
      const message: RelayOkMessage = ["OK", eventId, true, ""]
      
      expect(message[0]).toBe("OK")
      expect(message[1]).toBe(eventId)
      expect(message[2]).toBe(true)
      expect(message[3]).toBe("")
    })

    test("should create valid OK message for rejected event", () => {
      const eventId = EventId.make("a".repeat(64))
      const message: RelayOkMessage = ["OK", eventId, false, "invalid: malformed event"]
      
      expect(message[0]).toBe("OK")
      expect(message[1]).toBe(eventId)
      expect(message[2]).toBe(false)
      expect(message[3]).toBe("invalid: malformed event")
    })

    test("should create valid EOSE message", () => {
      const subId = SubscriptionId.make("sub123")
      const message: RelayEoseMessage = ["EOSE", subId]
      
      expect(message[0]).toBe("EOSE")
      expect(message[1]).toBe(subId)
      expect(message).toHaveLength(2)
    })

    test("should create valid CLOSED message", () => {
      const subId = SubscriptionId.make("sub123")
      const message: RelayClosedMessage = ["CLOSED", subId, "error: database connection failed"]
      
      expect(message[0]).toBe("CLOSED")
      expect(message[1]).toBe(subId)
      expect(message[2]).toBe("error: database connection failed")
      expect(message).toHaveLength(3)
    })

    test("should create valid NOTICE message", () => {
      const message: RelayNoticeMessage = ["NOTICE", "Relay is shutting down for maintenance"]
      
      expect(message[0]).toBe("NOTICE")
      expect(message[1]).toBe("Relay is shutting down for maintenance")
      expect(message).toHaveLength(2)
    })
  })

  describe("Message Helper Functions", () => {
    test("should create OK message with prefix", () => {
      const eventId = EventId.make("a".repeat(64))
      const message = createOkMessage(eventId, false, "rate-limited", "slow down there chief")
      
      expect(message).toEqual([
        "OK",
        eventId,
        false,
        "rate-limited: slow down there chief"
      ])
    })

    test("should create OK message without prefix", () => {
      const eventId = EventId.make("a".repeat(64))
      const message = createOkMessage(eventId, true, undefined, "")
      
      expect(message).toEqual([
        "OK",
        eventId,
        true,
        ""
      ])
    })

    test("should create CLOSED message with prefix", () => {
      const subId = SubscriptionId.make("sub123")
      const message = createClosedMessage(subId, "unsupported", "filter contains unknown elements")
      
      expect(message).toEqual([
        "CLOSED",
        subId,
        "unsupported: filter contains unknown elements"
      ])
    })

    test("should create CLOSED message without prefix", () => {
      const subId = SubscriptionId.make("sub123")
      const message = createClosedMessage(subId, undefined, "shutting down idle subscription")
      
      expect(message).toEqual([
        "CLOSED",
        subId,
        "shutting down idle subscription"
      ])
    })
  })

  describe("Error Types", () => {
    test("should create EventValidationError", () => {
      const eventId = EventId.make("a".repeat(64))
      const error = new EventValidationError({
        eventId,
        reason: "Invalid signature"
      })
      
      expect(error.eventId).toBe(eventId)
      expect(error.reason).toBe("Invalid signature")
      expect(error._tag).toBe("EventValidationError")
    })

    test("should create EventValidationError without eventId", () => {
      const error = new EventValidationError({
        reason: "Malformed JSON"
      })
      
      expect(error.eventId).toBeUndefined()
      expect(error.reason).toBe("Malformed JSON")
    })

    test("should create EventNotFound", () => {
      const eventId = EventId.make("a".repeat(64))
      const error = new EventNotFound({ eventId })
      
      expect(error.eventId).toBe(eventId)
      expect(error._tag).toBe("EventNotFound")
    })

    test("should create SubscriptionError", () => {
      const subId = SubscriptionId.make("sub123")
      const error = new SubscriptionError({
        subscriptionId: subId,
        reason: "Too many filters"
      })
      
      expect(error.subscriptionId).toBe(subId)
      expect(error.reason).toBe("Too many filters")
      expect(error._tag).toBe("SubscriptionError")
    })

    test("should create RateLimitError", () => {
      const error = new RateLimitError({
        reason: "Too many requests per minute"
      })
      
      expect(error.reason).toBe("Too many requests per minute")
      expect(error._tag).toBe("RateLimitError")
    })

    test("should create AuthenticationError", () => {
      const error = new AuthenticationError({
        reason: "Invalid challenge response"
      })
      
      expect(error.reason).toBe("Invalid challenge response")
      expect(error._tag).toBe("AuthenticationError")
    })
  })

  describe("Standard OK prefixes", () => {
    test("should support all standard OK prefixes", () => {
      const eventId = EventId.make("a".repeat(64))
      
      const prefixes = [
        "duplicate",
        "pow", 
        "blocked",
        "rate-limited",
        "invalid",
        "restricted",
        "error"
      ]
      
      for (const prefix of prefixes) {
        const message = createOkMessage(eventId, false, prefix as any, "test message")
        expect(message[3]).toBe(`${prefix}: test message`)
      }
    })
  })

  describe("Standard CLOSED prefixes", () => {
    test("should support all standard CLOSED prefixes", () => {
      const subId = SubscriptionId.make("sub123")
      
      const prefixes = [
        "unsupported",
        "error"
      ]
      
      for (const prefix of prefixes) {
        const message = createClosedMessage(subId, prefix as any, "test message")
        expect(message[2]).toBe(`${prefix}: test message`)
      }
    })
  })
})