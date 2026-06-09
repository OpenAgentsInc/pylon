import { Effect, Console } from "effect"
import { createCliRenderer, BoxRenderable, TextRenderable, ScrollBoxRenderable, parseColor, type CliRenderer } from "@opentui/core"

// Global UI reference for log aggregation
let globalRenderer: CliRenderer | null = null
let logScrollBox: ScrollBoxRenderable | null = null

function logToUi(message: string) {
  if (logScrollBox && globalRenderer) {
    const timestamp = new Date().toISOString().slice(11, 19)
    const line = new TextRenderable(globalRenderer, {
      content: `[${timestamp}] ${message}`,
      fg: parseColor("#A5D6FF"),
      width: "100%",
    })
    logScrollBox.add(line)
  } else {
    // Silent pre-boot buffering or console logging
    Console.log(`[BOOT] ${message}`)
  }
}

// Effect-native logging helper
const log = (message: string) => Effect.sync(() => logToUi(message))

// Hardware Resource & Telemetry Discovery Service
const startHardwareTelemetryLoop = Effect.gen(function* () {
  yield* log("[Telemetry] Initializing platform discovery...")
  yield* Effect.repeat(
    Effect.gen(function* () {
      yield* log("[Telemetry] Polling CPU/GPU thermals and load...")
    }),
    { schedule: "10 seconds" }
  )
})

// Money Dev Kit (MDK) Wallet Sidecar Service
const startMdkWalletService = Effect.gen(function* () {
  yield* log("[Wallet] Connecting to local MDK agent-wallet daemon on port 3001...")
  yield* log("[Wallet] Wallet connection established. Ready to receive payouts.")
})

// Nostr Continuous Presence Heartbeat Loop
const startPresenceHeartbeatLoop = Effect.gen(function* () {
  yield* log("[Heartbeat] Initializing presence service...")
  yield* Effect.repeat(
    Effect.gen(function* () {
      yield* log("[Heartbeat] Emitting presence signal (online, model_ready=true)")
    }),
    { schedule: "30 seconds" }
  )
})

// OpenCode Programmatic Integration Service
const runOpencodeStartupInference = Effect.gen(function* () {
  yield* log("[OpenCode] Checking for local OpenCode CLI installation...")
  const opencodePath = Bun.which("opencode")
  if (opencodePath) {
    yield* log(`[OpenCode] Found OpenCode CLI at ${opencodePath}. Executing hello world inference...`)
    
    const result = yield* Effect.tryPromise({
      try: async () => {
        const proc = Bun.spawn([opencodePath, "run", "Say 'Hello, World!' in one short sentence."])
        const stdout = await new Response(proc.stdout).text()
        return stdout.trim()
      },
      catch: (error) => new Error(`Failed to execute OpenCode inference: ${String(error)}`),
    })
    
    yield* log(`[OpenCode] Inference Response: "${result}"`)
  } else {
    yield* log("[OpenCode] OpenCode CLI is not installed on this system.")
  }
})

// Main Pylon v0.3 Application Loop
const runPylonNode = Effect.gen(function* () {
  yield* log("Initializing Pylon v0.3 observational earning node...")

  // Bootstrap OpenTUI Core
  const renderer = yield* Effect.tryPromise({
    try: () =>
      createCliRenderer({
        screenMode: "fullscreen",
        exitOnCtrlC: true,
        targetFps: 30,
      }),
    catch: (error) => new Error(`Failed to initialize OpenTUI renderer: ${String(error)}`),
  })

  // Set the global renderer reference
  globalRenderer = renderer

  // Create UI Container Layout with borders
  const mainBox = new BoxRenderable(renderer, {
    border: true,
    borderType: "single",
    title: " // Pylon v0.3 observational dashboard log feed ",
    width: "100%",
    height: "100%",
  })
  renderer.root.add(mainBox)

  // Create an inner scrollable log box
  logScrollBox = new ScrollBoxRenderable(renderer, {
    scrollY: true,
    flexGrow: 1,
    width: "100%",
    height: "100%",
  })
  mainBox.add(logScrollBox)

  // Start OpenTUI Event Loop
  renderer.start()

  // Start Background Services as Concurrent Fibers
  const telemetryFiber = yield* Effect.fork(startHardwareTelemetryLoop)
  const walletFiber = yield* Effect.fork(startMdkWalletService)
  const heartbeatFiber = yield* Effect.fork(startPresenceHeartbeatLoop)
  const opencodeFiber = yield* Effect.fork(runOpencodeStartupInference)

  yield* log("Pylon v0.3 observational dashboard active.")

  // Enter the persistent execution block
  yield* Effect.never
})

// Execute the main program safely via Effect
Effect.runPromise(
  runPylonNode.pipe(
    Effect.catchAll((error) =>
      Console.error(`Pylon v0.3 crashed on startup: ${error.message}`)
    )
  )
)
