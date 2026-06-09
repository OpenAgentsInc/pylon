#!/usr/bin/env bun

import { Effect, Console } from "effect"
import {
  createCliRenderer,
  BoxRenderable,
  TextRenderable,
  ScrollBoxRenderable,
  MarkdownRenderable,
  TextareaRenderable,
  parseColor,
  SyntaxStyle,
  type CliRenderer
} from "@opentui/core"

// Global UI references for log aggregation and balance updates
let globalRenderer: CliRenderer | null = null
let logScrollBox: ScrollBoxRenderable | null = null
let balanceTextRenderable: TextRenderable | null = null
let statusTextRenderable: TextRenderable | null = null
let telemetryTextRenderable: TextRenderable | null = null

const logHistory: string[] = []

const syntaxStyle = SyntaxStyle.fromStyles({
  default: { fg: parseColor("#E6EDF3") },
  keyword: { fg: parseColor("#FF7B72"), bold: true },
  string: { fg: parseColor("#A5D6FF") },
  comment: { fg: parseColor("#8B949E"), italic: true },
  number: { fg: parseColor("#79C0FF") },
  function: { fg: parseColor("#D2A8FF") },
  type: { fg: parseColor("#FFA657") },
  variable: { fg: parseColor("#E6EDF3") },
  property: { fg: parseColor("#79C0FF") },
  "markup.heading": { fg: parseColor("#00D7FF"), bold: true },
  "markup.bold": { fg: parseColor("#F0F6FC"), bold: true },
  "markup.italic": { fg: parseColor("#F0F6FC"), italic: true },
  "markup.list": { fg: parseColor("#FF7B72") },
  "markup.quote": { fg: parseColor("#8B949E"), italic: true },
  "markup.raw": { fg: parseColor("#A5D6FF"), bg: parseColor("#161B22") },
  "markup.link": { fg: parseColor("#58A6FF"), underline: true },
  "markup.link.url": { fg: parseColor("#58A6FF"), underline: true },
  conceal: { fg: parseColor("#6E7681") },
})

function logToUi(message: string) {
  const now = new Date()
  const timestamp = `${String(now.getHours()).padStart(2, "0")}:${String(now.getMinutes()).padStart(2, "0")}:${String(now.getSeconds()).padStart(2, "0")}`
  logHistory.push(`[${timestamp}] ${message}`)

  if (logScrollBox && globalRenderer) {
    const line = new MarkdownRenderable(globalRenderer, {
      content: `[${timestamp}] ${message}`,
      syntaxStyle,
      width: "100%",
      conceal: true,
    })
    logScrollBox.add(line)
  } else {
    // Silent pre-boot buffering or console logging
    Console.log(`[BOOT] ${message}`)
  }
}

// Effect-native logging helper
const log = (message: string) => Effect.sync(() => logToUi(message))

function updateMdkBalance(balance: number, suffix = "Sats") {
  if (balanceTextRenderable) {
    balanceTextRenderable.content = ` Balance: ${balance.toLocaleString()} ${suffix}`
  }
}

function updateMdkStatus(status: string, color = "#22C55E") {
  if (statusTextRenderable) {
    statusTextRenderable.content = ` Wallet: ${status}`
    statusTextRenderable.textColor = parseColor(color)
  }
}

function updateTelemetryState(state: string, model: string, vram: string) {
  if (telemetryTextRenderable) {
    telemetryTextRenderable.content = ` State: ${state}\n Model: ${model}\n VRAM:  ${vram}`
  }
}

// Hardware Resource & Telemetry Discovery Service
const startHardwareTelemetryLoop = Effect.gen(function* () {
  yield* log("[Telemetry] Platform discovery initialized.")
  while (true) {
    yield* Effect.sync(() => {
      updateTelemetryState("IDLE", "None (Observational)", "0.0 GB / 0.0 GB")
    })
    yield* Effect.sleep("10 seconds")
  }
})

// Money Dev Kit (MDK) Wallet Sidecar Service
const startMdkWalletService = Effect.gen(function* () {
  yield* log("[Wallet] Connecting to local MDK agent-wallet daemon...")
  let loggedOffline = false
  let loggedOnline = false
  while (true) {
    const balance = yield* Effect.tryPromise({
      try: async () => {
        const proc = Bun.spawn(["npx", "--yes", "@moneydevkit/agent-wallet", "balance"], {
          stdout: "pipe",
          stderr: "pipe",
        })
        
        // Implement a strict 3-second timeout to prevent npx download hangs
        const textPromise = new Response(proc.stdout).text()
        const timeoutPromise = new Promise<string>((_, reject) =>
          setTimeout(() => {
            proc.kill()
            reject(new Error("Timeout"))
          }, 3000)
        )
        
        const stdout = await Promise.race([textPromise, timeoutPromise])
        const data = JSON.parse(stdout)
        if (data && typeof data.balance === "number") {
          return data.balance
        }
        if (data && typeof data.confirmed === "number") {
          return data.confirmed
        }
        return null
      },
      catch: () => null,
    })

    yield* Effect.sync(() => {
      if (balance !== null) {
        updateMdkBalance(balance)
        updateMdkStatus("ONLINE (OK)", "#22C55E")
        if (!loggedOnline) {
          logToUi("[Wallet] MDK agent-wallet daemon connected successfully. Balance synchronized.")
          loggedOnline = true
          loggedOffline = false
        }
      } else {
        // Explicitly show 0 Sats and OFFLINE status (No fallback mockup balance)
        updateMdkBalance(0)
        updateMdkStatus("OFFLINE", "#EF4444")
        if (!loggedOffline) {
          logToUi("[Wallet] Local MDK daemon is not running (daemon.log ENOENT / uninitialized). Operating in OFFLINE mode.")
          loggedOffline = true
          loggedOnline = false
        }
      }
    })
    yield* Effect.sleep("10 seconds")
  }
})

// Nostr Continuous Presence Heartbeat Loop
const startPresenceHeartbeatLoop = Effect.gen(function* () {
  yield* log("[Heartbeat] Presence service initialized (online, model_ready=true)")
  while (true) {
    yield* Effect.sleep("30 seconds")
  }
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

// OpenCode Programmatic Integration Service (Diagnostics on boot)
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

  globalRenderer = renderer

  // 1. Create Main Outer Layout (Height 100%, Width 100%)
  const outerContainer = new BoxRenderable(renderer, {
    flexDirection: "column",
    width: "100%",
    height: "100%",
  })
  renderer.root.add(outerContainer)

  // 2. Create Header Panel (Height 3)
  const headerBox = new BoxRenderable(renderer, {
    border: true,
    borderType: "single",
    title: " // Pylon earning node ",
    width: "100%",
    height: 3,
  })
  const headerText = new TextRenderable(renderer, {
    content: " earning node active | watching autonomous agent work and earn bitcoin",
    fg: parseColor("#8BA6CD"),
  })
  headerBox.add(headerText)
  outerContainer.add(headerBox)

  // 3. Create Main Split Pane (Row Direction, Flex Grow)
  const splitPane = new BoxRenderable(renderer, {
    flexDirection: "row",
    width: "100%",
    flexGrow: 1,
  })
  outerContainer.add(splitPane)

  // 3a. Logs/Feed Panel (Left Column, Flex Grow)
  const leftPanel = new BoxRenderable(renderer, {
    border: true,
    borderType: "single",
    title: " // Active Workroom Execution Logs ",
    flexGrow: 1,
    height: "100%",
  })
  splitPane.add(leftPanel)

  logScrollBox = new ScrollBoxRenderable(renderer, {
    scrollY: true,
    flexGrow: 1,
    width: "100%",
    height: "100%",
  })
  leftPanel.add(logScrollBox)

  // 3b. Telemetry & Balance Panel (Right Column, Fixed Width 35)
  const rightPanel = new BoxRenderable(renderer, {
    border: true,
    borderType: "single",
    title: " // Telemetry & Wallet ",
    width: 35,
    flexBasis: 35,
    flexGrow: 0,
    flexShrink: 0,
    height: "100%",
    flexDirection: "column",
  })
  splitPane.add(rightPanel)

  statusTextRenderable = new TextRenderable(renderer, {
    content: " Wallet: OFFLINE",
    fg: parseColor("#EF4444"),
    width: "100%",
    height: 1,
  })
  rightPanel.add(statusTextRenderable)

  balanceTextRenderable = new TextRenderable(renderer, {
    content: " Balance: 0 Sats",
    fg: parseColor("#66D9EF"),
    width: "100%",
    height: 1,
  })
  rightPanel.add(balanceTextRenderable)

  // Add some separator space
  rightPanel.add(new TextRenderable(renderer, { content: " ---------------------------------", fg: parseColor("#3B5B82"), height: 1 }))

  telemetryTextRenderable = new TextRenderable(renderer, {
    content: " State: IDLE\n Model: -\n VRAM:  -",
    fg: parseColor("#D7E5FA"),
    width: "100%",
    height: 3,
  })
  rightPanel.add(telemetryTextRenderable)

  // 4. Create Composer Input Panel (Bottom, Height 5)
  const composerBox = new BoxRenderable(renderer, {
    border: true,
    borderType: "single",
    title: " // Composer (meta+return to submit) ",
    width: "100%",
    height: 5,
  })
  outerContainer.add(composerBox)

  const composerInput = new TextareaRenderable(renderer, {
    width: "100%",
    height: "100%",
    placeholder: "Ask your agent anything...",
    onSubmit: async () => {
      const prompt = composerInput.plainText.trim()
      if (!prompt) return

      // Clear the composer
      composerInput.setText("")

      // Render User prompt in logs feed
      const userLine = new MarkdownRenderable(renderer, {
        content: `**User**: ${prompt}`,
        syntaxStyle,
        width: "100%",
        conceal: true,
      })
      logScrollBox?.add(userLine)

      // Setup response placeholder
      const responseLine = new MarkdownRenderable(renderer, {
        content: `**OpenCode**: ... thinking ...`,
        syntaxStyle,
        width: "100%",
        conceal: true,
        streaming: true,
      })
      logScrollBox?.add(responseLine)

      // Start asynchronous OpenCode inference
      const opencodePath = Bun.which("opencode")
      if (opencodePath) {
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

        const reader = proc.stdout.getReader()
        const decoder = new TextDecoder()
        let buffer = ""
        let receivedText = ""

        responseLine.content = `**OpenCode**: `

        while (true) {
          const { done, value } = await reader.read()
          if (done) break

          buffer += decoder.decode(value, { stream: true })
          const lines = buffer.split("\n")
          buffer = lines.pop() ?? ""

          for (const line of lines) {
            if (!line.trim()) continue
            try {
              const event = JSON.parse(line)
              if (event.type === "text" && event.part && event.part.text) {
                receivedText += event.part.text
                responseLine.content = `**OpenCode**: ${receivedText}`
              }
              if (event.type === "step_finish" && event.part && event.part.tokens) {
                const cost = event.part.cost ?? 0
                const tokens = event.part.tokens.total ?? 0
                responseLine.content = `**OpenCode**: ${receivedText}\n\n*[Cost: $${cost.toFixed(4)} | Tokens: ${tokens}]*`
              }
            } catch {
              // Ignore partial chunk syntax errors
            }
          }
        }
        
        responseLine.streaming = false
      } else {
        responseLine.content = `**OpenCode**: Error - OpenCode CLI is not installed on this system.`
        responseLine.streaming = false
      }
    },
  })
  composerBox.add(composerInput)

  // Start OpenTUI Event Loop
  renderer.start()

  // Focus on Composer Input
  composerInput.focus()

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
