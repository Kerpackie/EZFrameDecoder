# EZ Frame Decoder

A desktop application for decoding 23-byte ASCII frames with a user-friendly interface. Built with Tauri for cross-platform compatibility and Vue 3 for a modern frontend experience.

## Features

- **Frame Decoding**: Decode 23-byte ASCII frames in the format `<command_data>` with 1-3 `>` terminators
- **Batch Processing**: Upload and decode multiple frames from `.txt`, `.log`, or `.csv` files
- **Interactive UI**: Click frames in the sidebar to decode them instantly
- **Field Descriptions**: Hover over decoded fields to see detailed descriptions
- **Hot-Reloadable Specs**: Modify the JSON specification file without restarting the app
- **Cross-Platform**: Native desktop app for Windows, macOS, and Linux

## Getting Started

### Prerequisites

- [Node.js](https://nodejs.org/) (v16 or later)
- [pnpm](https://pnpm.io/) package manager
- [Rust](https://rustup.rs/) toolchain
- [Tauri prerequisites](https://tauri.app/v1/guides/getting-started/prerequisites)

### Installation

1. Clone the repository:
```bash
git clone <repository-url>
cd EZFrameDecoder
```

2. Install dependencies:
```bash
pnpm install
```

3. Run in development mode:
```bash
pnpm tauri dev
```

4. Build for production:
```bash
pnpm tauri build
```

## Usage

### Single Frame Decoding

1. Paste a 23-byte frame into the input field (e.g., `<A20481 0           >>>`)
2. Click the **Decode** button or press **Enter**
3. View the decoded fields in the main panel

### Batch File Processing

1. Drag and drop a `.txt`, `.log`, or `.csv` file onto the sidebar
2. Click any frame in the list to decode it
3. Navigate through multiple frames using the sidebar

### Supported Frame Format

Frames must be exactly 23 characters in the format:
- Start with `<`
- Command letter (A, B, C, etc.)
- Data payload
- End with 1-3 `>` characters for padding

Example: `<A20481 0           >>>`

## Configuration

The application uses a JSON specification file to define how frames are decoded. The spec file is automatically copied to your user config directory on first run:

- **Windows**: `%APPDATA%/EZFrameDecoder/spec_override.json`
- **macOS**: `~/Library/Application Support/EZFrameDecoder/spec_override.json`
- **Linux**: `~/.config/EZFrameDecoder/spec_override.json`

### Specification Format

The spec file defines commands, fields, and their decoding rules:

```json
{
  "framing": { "start": "<" },
  "commands": [
    {
      "letter": "A",
      "description": "Test command",
      "items": [
        {
          "name": "Header",
          "fields": [
            {
              "name": "Opcode",
              "len": 4,
              "base": 10,
              "type": "u16",
              "description": "Operation selector"
            }
          ]
        }
      ]
    }
  ]
}
```

## Keyboard Shortcuts

- **Enter** (in input field) — Decode current frame

## Architecture

### Frontend (Vue 3 + TypeScript)
- [`src/components/DecoderInput.vue`](src/components/DecoderInput.vue) - Frame input component
- [`src/components/CommandList.vue`](src/components/FrameList.vue) - File upload and frame list
- [`src/composables/useSharedDecode`](src/composables/) - Shared decoding logic
- [`src/stores/frameStore`](src/stores/) - Frame state management

### Backend (Rust + Tauri)
- [`src-tauri/src/decoder.rs`](src-tauri/src/decoder.rs) - Core frame decoding logic
- [`src-tauri/src/main.rs`](src-tauri/src/main.rs) - Tauri commands and app setup

### Key Tauri Commands
- [`decode_frame`](src-tauri/src/main.rs) - Decode a single frame
- [`batch_decode`](src-tauri/src/main.rs) - Decode multiple frames from text
- [`reload_spec`](src-tauri/src/main.rs) - Reload specification from disk

## Development

### Project Structure

```
├── src/                    # Vue frontend
│   ├── components/         # Vue components
│   ├── composables/        # Vue composables
│   ├── pages/             # Application pages
│   ├── stores/            # State management
│   └── main.ts            # Frontend entry point
├── src-tauri/             # Rust backend
│   ├── src/
│   │   ├── decoder.rs     # Frame decoding logic
│   │   └── main.rs        # Tauri app setup
│   └── resources/         # Bundled resources
└── dist/                  # Built application
```

### Testing

Run Rust unit tests:
```bash
cd src-tauri
cargo test
```

### Building

The application can be built for multiple platforms:
```bash
pnpm tauri build --target x86_64-pc-windows-msvc    # Windows
pnpm tauri build --target x86_64-apple-darwin       # macOS Intel
pnpm tauri build --target aarch64-apple-darwin      # macOS Apple Silicon
pnpm tauri build --target x86_64-unknown-linux-gnu  # Linux
```

## License

This project is licensed under the **Apache License 2.0**. You are free to:

* **Use** the software for commercial or non-commercial purposes.
* **Modify** the code to suit your needs.
* **Distribute** original or modified versions, provided you retain the license and attribution.

#### Why Apache 2.0?

The Apache 2.0 license was chosen to encourage adoption — including by commercial and industrial users — while providing legal clarity around:

* **Patent use**: Users are granted a license to any relevant patents held by contributors.
* **Attribution**: Credit must be given to the original author(s).
* **Modification notice**: If you distribute a modified version, you must document the changes.

For full license details, see the [LICENSE](./LICENSE) file.

## Contributing

TBD.
