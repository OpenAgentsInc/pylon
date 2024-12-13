# Project Hierarchy

```
pylon/
├── .gitignore                  # Git ignore file
├── .vscode/                    # VS Code configuration
├── LICENSE                     # Project license
├── README.md                   # Project overview and documentation
├── docs/                       # Documentation files
├── index.html                  # Main HTML entry point for the web interface
├── package.json               # Node.js dependencies and scripts
├── public/                    # Public static assets
├── src-tauri/                 # Rust backend code
│   ├── Cargo.lock            # Rust dependency lock file
│   ├── Cargo.toml            # Rust project configuration
│   ├── build.rs              # Tauri build script
│   ├── capabilities/         # Tauri capability configurations
│   ├── icons/               # Application icons
│   └── src/                 # Rust source code
│       ├── lib.rs           # Library entry point
│       ├── main.rs          # Main application entry point
│       ├── mcp/             # Model Context Protocol implementation
│       │   ├── capabilities.rs  # MCP capability handling
│       │   ├── protocol.rs      # MCP protocol implementation
│       │   ├── server.rs        # WebSocket server implementation
│       │   └── types.rs         # MCP type definitions
│       └── tests/           # Test modules
│           └── mcp/         # MCP-related tests
├── src/                     # Frontend TypeScript/React code
│   ├── App.css             # Main application styles
│   ├── App.tsx             # Main React component
│   ├── assets/             # Frontend assets
│   ├── components/         # React components
│   ├── main.tsx           # Frontend entry point
│   ├── styles/            # Additional styles
│   └── vite-env.d.ts      # TypeScript environment declarations
├── tsconfig.json          # TypeScript configuration
├── tsconfig.node.json     # Node-specific TypeScript configuration
├── vite.config.ts         # Vite build configuration
└── yarn.lock              # Yarn dependency lock file
```

## Key Components

### Backend (Rust)
- `src-tauri/src/mcp/`: Implementation of the Model Context Protocol
- `src-tauri/src/mcp/protocol.rs`: Core MCP message handling and protocol logic
- `src-tauri/src/mcp/server.rs`: WebSocket server for MCP communication
- `src-tauri/src/mcp/types.rs`: Type definitions for MCP messages and structures
- `src-tauri/src/mcp/capabilities.rs`: MCP capability management

### Frontend (TypeScript/React)
- `src/App.tsx`: Main React application component
- `src/main.tsx`: Frontend entry point
- `src/components/`: React component library

### Configuration
- `src-tauri/Cargo.toml`: Rust dependencies and project configuration
- `package.json`: Node.js dependencies and scripts
- `vite.config.ts`: Vite build system configuration
- `tsconfig.json`: TypeScript compiler configuration

### Documentation
- `README.md`: Project overview and getting started guide
- `docs/`: Detailed documentation