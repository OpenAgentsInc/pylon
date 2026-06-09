import { describe, expect, test } from "bun:test"
import { discoverHostInventory, projectHostInventoryFixture } from "../src/inventory"
import { assertPublicProjectionSafe } from "../src/state"

describe("Pylon host inventory projection", () => {
  test("projects a macOS Apple Silicon fixture without raw private paths", () => {
    const inventory = projectHostInventoryFixture({
      platform: "darwin",
      arch: "arm64",
      cpuCores: 12,
      cpuModel: "Apple M3 Max",
      totalMemoryBytes: 36 * 1024 * 1024 * 1024,
      freeMemoryBytes: 12 * 1024 * 1024 * 1024,
      homeFreeBytes: 100 * 1024 * 1024 * 1024,
      networkInterfaceCount: 6,
      externalNetworkInterfaceCount: 2,
      opencodeInstalled: true,
      geminiConfigured: false,
      appleFmReady: true,
      localModelRefs: ["model.local.probe.retained_fixture_cache"],
      now: "2026-06-09T00:00:00.000Z",
    })

    expect(inventory.freshness).toBe("fresh")
    expect(inventory.platform).toBe("darwin")
    expect(inventory.accelerator.kind).toBe("apple_silicon")
    expect(inventory.backendHealth.find((backend) => backend.backendRef === "backend.apple_fm")?.state).toBe("ready")
    expect(inventory.backendHealth.find((backend) => backend.backendRef === "backend.opencode.cli")?.state).toBe("ready")
    expect(inventory.eligibleInventoryCount).toBe(1)
    expect(JSON.stringify(inventory)).not.toContain("/Users/")
    expect(JSON.stringify(inventory)).not.toContain("GEMINI_API_KEY")
  })

  test("projects a Linux baseline with unavailable accelerator but configured Gemini", () => {
    const inventory = projectHostInventoryFixture({
      platform: "linux",
      arch: "x64",
      cpuCores: 8,
      cpuModel: "AMD EPYC",
      totalMemoryBytes: 16 * 1024 * 1024 * 1024,
      freeMemoryBytes: 8 * 1024 * 1024 * 1024,
      homeFreeBytes: 40 * 1024 * 1024 * 1024,
      networkInterfaceCount: 3,
      externalNetworkInterfaceCount: 1,
      opencodeInstalled: false,
      geminiConfigured: true,
      appleFmReady: false,
      now: "2026-06-09T00:00:00.000Z",
    })

    expect(inventory.freshness).toBe("fresh")
    expect(inventory.platform).toBe("linux")
    expect(inventory.accelerator.blockerRefs).toContain("blocker.inventory.accelerator_unproven")
    expect(inventory.backendHealth.find((backend) => backend.backendRef === "backend.gemini")?.state).toBe("configured")
    expect(inventory.backendHealth.find((backend) => backend.backendRef === "backend.apple_fm")?.state).toBe("unsupported")
  })

  test("public projection rejects private topology, auth, raw cache paths, and environment dumps", () => {
    expect(() => assertPublicProjectionSafe({ privateTopology: "tailnet node map" })).toThrow("not public-safe")
    expect(() => assertPublicProjectionSafe({ providerAuth: "token" })).toThrow("not public-safe")
    expect(() => assertPublicProjectionSafe({ cachePath: "/Users/christopherdavid/.cache/model" })).toThrow("not public-safe")
    expect(() => assertPublicProjectionSafe({ env: { apiKey: "abc" } })).toThrow("not public-safe")
  })

  test("live discovery emits fresh or unavailable public-safe inventory", async () => {
    const inventory = await discoverHostInventory({
      now: new Date("2026-06-09T00:00:00.000Z"),
      env: {},
    })

    expect(inventory.schema).toBe("openagents.pylon.host_inventory.v0.3")
    expect(["fresh", "unavailable"]).toContain(inventory.freshness)
    assertPublicProjectionSafe(inventory)
  })
})
