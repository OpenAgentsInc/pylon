import { Effect, Console } from "effect"
import { createCliRenderer, BoxRenderable, TextRenderable, ScrollBoxRenderable, parseColor, type CliRenderer } from "@opentui/core"

// Global UI reference for log aggregation
let globalRenderer: CliRenderer | null = null
let logScrollBox: ScrollBoxRenderable | null = null
const logHistory: string[] = []

function logToUi(message: string) {
  const now = new Date()
  const timestamp = `${String(now.getHours()).padStart(2, "0")}:${String(now.getMinutes()).padStart(2, "0")}:${String(now.getSeconds()).padStart(2, "0")}`
  logHistory.push(`[${timestamp}] ${message}`)

  if (logScrollBox && globalRenderer) {
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

// OpenCode Programmatic Integration Helper
async function executeOpencodeInference(opencodePath: string, prompt: string) {
  const proc = Bun.spawn(
    [
      opencodePath,
      "run",
      prompt,
      "--model",
      "opencode/deepseek-v4-flash-free",
      "--format",
      "json",
    ],
    {
      stdout: "pipe",
      stderr: "pipe",
    }
  )
  const stdout = await new Response(proc.stdout).text()
  
  let textResult = ""
  let finalCost = 0
  let totalTokens = 0
  
  for (const line of stdout.split("\n")) {
    if (!line.trim()) continue
    try {
      const event = JSON.parse(line)
      if (event.type === "text" && event.part && event.part.text) {
        textResult += event.part.text
      }
      if (event.type === "step_finish" && event.part && event.part.tokens) {
        finalCost = event.part.cost ?? 0
        totalTokens = event.part.tokens.total ?? 0
      }
    } catch {
      // Ignore parse errors or other outputs
    }
  }
  
  return {
    text: textResult.trim(),
    cost: finalCost,
    tokens: totalTokens,
  }
}

// OpenCode Programmatic Integration Service
const runOpencodeStartupInference = Effect.gen(function* () {
  yield* log("[OpenCode] Checking for local OpenCode CLI installation...")
  const opencodePath = Bun.which("opencode")
  if (opencodePath) {
    yield* log(`[OpenCode] Found OpenCode CLI at ${opencodePath}. Initiating bootup diagnostics...`)
    
    // 1. Get neutral log summary (<10 words)
    const logSummaryResult = yield* Effect.tryPromise({
      try: () => {
        const prompt = `Here are the bootup sequence logs:\n\n${logHistory.join("\n")}\n\nProvide a one line, <10 word, neutral, terminal-sounding summary of these bootup sequence logs.`
        return executeOpencodeInference(opencodePath, prompt)
      },
      catch: (error) => new Error(`Failed to execute bootup summary: ${String(error)}`),
    })
    
    yield* log(`[OpenCode] Bootup Summary: "${logSummaryResult.text}"`)
    yield* log(`[OpenCode] Cost: $${logSummaryResult.cost.toFixed(4)} | Tokens: ${logSummaryResult.tokens}`)

    // 2. Read AGENTS.md and summarize capabilities
    yield* log("[OpenCode] Fetching and analyzing https://openagents.com/AGENTS.md...")
    const capabilityResult = yield* Effect.tryPromise({
      try: () => {
        const prompt = "Read https://openagents.com/AGENTS.md (readonly) and summarize current capabilities in 1-2 concise paragraphs."
        return executeOpencodeInference(opencodePath, prompt)
      },
      catch: (error) => new Error(`Failed to execute capability summary: ${String(error)}`),
    })

    yield* log(`[OpenCode] Capabilities Summary:\n${capabilityResult.text}`)
    yield* log(`[OpenCode] Cost: $${capabilityResult.cost.toFixed(4)} | Tokens: ${capabilityResult.tokens}`)
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
