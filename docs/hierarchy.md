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
│       ├── commands.rs      # Tauri command handlers
│       ├── utils/           # Utility modules
│       │   ├── mod.rs       # Utils module definition
│       │   └── ollama.rs    # Ollama-specific utilities
│       ├── mcp/             # Model Context Protocol implementation
│       │   ├── mod.rs       # MCP module definition
│       │   ├── providers/   # Resource providers
│       │   │   ├── mod.rs   # Provider module definition
│       │   │   ├── filesystem.rs  # Filesystem provider
│       │   │   └── ollama/  # Ollama provider
│       │   │       ├── mod.rs     # Ollama module definition
│       │   │       ├── types.rs    # Ollama types
│       │   │       ├── provider.rs # Ollama provider implementation
│       │   │       └── tests.rs    # Ollama tests
│       │   ├── protocol/   # Protocol implementation
│       │   │   ├── mod.rs  # Protocol module definition
│       │   │   ├── types.rs # Protocol types
│       │   │   ├── handlers.rs # Protocol handlers
│       │   │   └── tests.rs # Protocol tests
│       │   ├── server.rs   # WebSocket server implementation
│       │   ├── clients.rs  # Client management
│       │   └── types.rs    # Common type definitions
│       └── tests/          # Test modules
│           └── mcp/        # MCP-related tests
├── src/                    # Frontend TypeScript/React code
│   ├── App.css            # Main application styles
│   ├── App.tsx            # Main React component
│   ├── assets/            # Frontend assets
│   ├── components/        # React components
│   ├── main.tsx          # Frontend entry point
│   ├── styles/           # Additional styles
│   └── vite-env.d.ts     # TypeScript environment declarations
├── tsconfig.json         # TypeScript configuration
├── tsconfig.node.json    # Node-specific TypeScript configuration
├── vite.config.ts        # Vite build configuration
└── yarn.lock             # Yarn dependency lock file
```

## Key Components

### Backend (Rust)
- `src-tauri/src/lib.rs`: Library crate entry point
- `src-tauri/src/main.rs`: Binary crate entry point
- `src-tauri/src/commands.rs`: Tauri command handlers
- `src-tauri/src/utils/`: Utility modules
  - `ollama.rs`: Ollama-specific utilities (e.g., health checks)
- `src-tauri/src/mcp/`: Model Context Protocol implementation
  - `providers/`: Resource providers
    - `filesystem.rs`: Filesystem provider implementation
    - `ollama/`: Ollama provider implementation
  - `protocol/`: Protocol implementation
    - `types.rs`: Protocol-specific types
    - `handlers.rs`: Message handlers
  - `server.rs`: WebSocket server
  - `clients.rs`: Client management
  - `types.rs`: Common type definitions

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