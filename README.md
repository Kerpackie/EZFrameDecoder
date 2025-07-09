# EZ Frame Decoder

A high-performance desktop utility designed to effortlessly parse and interpret fixed-length data frames. Built with Rust and Tauri for a native cross-platform experience, and Vue 3 for a modern, reactive user interface.

This tool transforms raw data logs into human-readable information in an instant, making it ideal for engineers and developers debugging hardware protocols.

![Screenshot of EZ Frame Decoder](https://github.com/user-attachments/assets/5eb1b355-cdb0-415e-8a77-8c0d697cb6a5)

---

## ✨ Key Features

* **Dynamic Decoding**: Paste raw frames for instant, real-time decoding based on the active specification.
* **Batch Processing**: Load and parse frames from `.txt`, `.log`, or `.csv` files.
* **Customizable Specs**: Define your own command structures using the powerful, JSON-based `.ezspec` format.
* **Advanced Spec Editor**: A dedicated, feature-rich UI for creating and managing command families and their protocols (enabled via "Advanced Mode").
* **Flexible Spec Management**: Easily load custom spec files from anywhere, export your current spec, or reset to the default configuration.
* **Complex Protocol Support**: Supports conditional logic in your command definitions using `switch` blocks to handle variable packet structures based on header fields.
* **Durable Editing**: All changes made in the Spec Editor are automatically saved back to the loaded spec file, ensuring your work is never lost.
* **Modern Interface**: A clean, themeable UI with both light and dark modes for your comfort.

---

## 🚀 How It Works

The application's logic is driven by a specification file (`.ezspec`) that you control. This file defines how to interpret incoming data frames.

### The Spec File

* On its first launch, the application creates a default specification file named `spec_override.ezspec` in your system's configuration directory.
* **Healing Mechanism**: If this user-specific file ever becomes corrupted or is deleted, the application will automatically restore it from the bundled default, ensuring the app always starts in a valid state.
* You can find the default override file at these locations:
    * **Windows**: `%APPDATA%\EZFrameDecoder\spec_override.ezspec`
    * **macOS**: `~/Library/Application Support/EZFrameDecoder/spec_override.ezspec`
    * **Linux**: `~/.config/EZFrameDecoder/spec_override.ezspec`

### Application Modes

The application has two modes, which can be toggled in the **Settings** page:

1.  **Standard Mode**: The default mode, focused on decoding. It provides a read-only view of the currently loaded commands, perfect for safe, everyday use.
2.  **Advanced Mode**: Unlocks the full **Spec Editor**, giving you complete CRUD (Create, Read, Update, Delete) control over command families and their definitions.

---

## 📖 Usage

### Decoding Frames

1.  Navigate to the **Decoder** page.
2.  **Single Frame**: Paste a raw data frame into the input box and click "Decode".
3.  **Multiple Frames**: Drag and drop a `.txt`, `.log`, or `.csv` file containing frames onto the drop zone on the left. Click any frame in the list to see its decoded output.

### Managing Specs and Settings

1.  Navigate to the **Settings** page.
2.  Here you can toggle **Advanced Mode** or **Dark Mode**.
3.  If Advanced Mode is enabled, you can:
    * **Choose Spec File**: Load a custom `.ezspec` file from your computer.
    * **Export Spec**: Save the currently active specification to a new file.
    * **Reset to Default**: Revert the application to using the default `spec_override.ezspec` file.

### Editing Specifications (Advanced Mode)

1.  Enable **Advanced Mode** in Settings.
2.  Navigate to the **Spec Editor** page.
3.  **Families**:
    * The left sidebar lists all "Command Families" from your spec. A family is a group of commands that share properties like a start character and frame length.
    * Click **New Family** to create a new one.
    * Select a family to view its commands. You can then **Edit** or **Delete** the selected family.
4.  **Commands**:
    * With a family selected, you can view its list of commands. Click **Add New Command** to define a new command.
    * You can **Edit** or **Delete** existing commands directly from the list.
    * The command builder allows you to define fields, groups, and complex `switch` logic for dynamic packet parsing.

---

## 🧬 The `.ezspec` Format

The `.ezspec` file is a JSON object that defines how frames are decoded. The structure is hierarchical:

* **`families[]`**: An array of `Family` objects.
    * **`Family`**: Defines a top-level protocol. It has a `name`, `start` character, `terminator` character, and a fixed `frame_len`. It contains an array of `commands`.
        * **`Command`**: Defined by a unique `letter` within its family. It contains an array of `items` that describe the data structure.
            * **`Item`**: Can be a `Group` or a `Switch`.
                * **`Group`**: A named collection of `Fields`. The first group is typically named `header`.
                * **`Switch`**: Allows for conditional decoding. It keys off a field defined in a preceding group (usually the header) and decodes the rest of the payload based on that field's value.
                * **`Field`**: The smallest unit. It has a `name`, `len` (length in hex characters), `base` (10 or 16), and `type` (`u8`, `u16`, `bool`, etc.).

### Example Snippet

```json
{
  "families": [
    {
      "name": "Demo",
      "description": "A demo family of commands.",
      "start": "!",
      "terminator": ">",
      "frame_len": 23,
      "commands": [
        {
          "letter": "A",
          "description": "A demonstration command, showcasing different aspects!",
          "items": [
            {
              "name": "Header",
              "fields": [
                {
                  "name": "RSAddress",
                  "len": 2,
                  "base": 16,
                  "type": "u8",
                  "description": "Device address on bus"
                }
              ]
            },
            {
              "name": "Payload",
              "fields": [
                {
                  "name": "NumericHexData",
                  "len": 5,
                  "base": 16,
                  "type": "u32",
                  "description": "A numeric hex field with 5 character width."
                },
                {
                  "name": "NumericDecimalData",
                  "len": 4,
                  "base": 10,
                  "type": "u16",
                  "description": "A numeric decimal field with 4 character width."
                },
                {
                  "name": "SingleWideBoolean",
                  "len": 1,
                  "base": 16,
                  "type": "bool",
                  "description": "A single character wide Boolean flag."
                },
                {
                  "name": "DoubleWideBoolean",
                  "len": 2,
                  "base": 16,
                  "type": "bool",
                  "description": "A two character wide Boolean flag."
                },
                {
                  "name": "Padding",
                  "len": 4,
                  "base": 16,
                  "type": "u16",
                  "description": "A 4 character padding."
                }
              ]
            }
          ]
        },
        {
          "letter": "C",
          "description": "A demonstration of a command that switches based on an OPCode",
          "items": [
            {
              "name": "Header",
              "fields": [
                {
                  "name": "RSAddress",
                  "len": 2,
                  "base": 16,
                  "type": "u8",
                  "description": "Device address on bus"
                },
                {
                  "name": "OPCode",
                  "len": 4,
                  "base": 16,
                  "type": "u16",
                  "description": "The OPCode which we will use to switch our commands."
                }
              ]
            },
            {
              "switch": "OPCode",
              "cases": {
                "0x0000": {
                  "description": "OPCode 0000",
                  "groups": [
                    {
                      "name": "Group",
                      "fields": [
                        {
                          "name": "field-1",
                          "len": 6,
                          "base": 16,
                          "type": "u24",
                          "description": ""
                        },
                        {
                          "name": "field-2",
                          "len": 2,
                          "base": 16,
                          "type": "u8",
                          "description": ""
                        },
                        {
                          "name": "field-3",
                          "len": 2,
                          "base": 16,
                          "type": "u8",
                          "description": ""
                        },
                        {
                          "name": "field-4",
                          "len": 2,
                          "base": 16,
                          "type": "u8",
                          "description": ""
                        }
                      ]
                    }
                  ]
                },
                "0x1234": {
                  "description": "Case 1234",
                  "groups": [
                    {
                      "name": "payload",
                      "fields": [
                        {
                          "name": "field-1",
                          "len": 2,
                          "base": 16,
                          "type": "u8",
                          "description": ""
                        },
                        {
                          "name": "field-2",
                          "len": 2,
                          "base": 16,
                          "type": "u8",
                          "description": ""
                        },
                        {
                          "name": "field-3",
                          "len": 2,
                          "base": 16,
                          "type": "u8",
                          "description": ""
                        },
                        {
                          "name": "field-4",
                          "len": 2,
                          "base": 16,
                          "type": "u8",
                          "description": ""
                        },
                        {
                          "name": "field-5",
                          "len": 2,
                          "base": 16,
                          "type": "u8",
                          "description": ""
                        },
                        {
                          "name": "field-6",
                          "len": 2,
                          "base": 16,
                          "type": "u8",
                          "description": ""
                        }
                      ]
                    }
                  ]
                }
              },
              "default": {
                "description": "Default Case",
                "groups": [
                  {
                    "name": "payload",
                    "fields": [
                      {
                        "name": "field-1",
                        "len": 10,
                        "base": 16,
                        "type": "u32",
                        "description": "Padding"
                      },
                      {
                        "name": "field-2",
                        "len": 2,
                        "base": 16,
                        "type": "u8",
                        "description": ""
                      },
                      {
                        "name": "field-3",
                        "len": 2,
                        "base": 16,
                        "type": "u8",
                        "description": ""
                      }
                    ]
                  }
                ]
              }
            }
          ]
        }
      ]
    }
  ]
}
```

---

## 🛠️ Getting Started (from Source)

### Prerequisites

* [Node.js](https://nodejs.org/) (v16 or later)
* [pnpm](https://pnpm.io/) package manager
* [Rust](https://rustup.rs/) toolchain
* [Tauri prerequisites](https://tauri.app/v1/guides/getting-started/prerequisites)

### Installation

1.  Clone the repository:
    ```bash
    git clone <repository-url>
    cd EZFrameDecoder
    ```

2.  Install dependencies:
    ```bash
    pnpm install
    ```

3.  Run in development mode:
    ```bash
    pnpm tauri dev
    ```

4.  Build for production:
    ```bash
    pnpm tauri build
    ```

---

## 🤝 Contributing

Contributions are welcome! Whether you're fixing a bug, adding a new feature, or improving documentation, your help is appreciated.

### How to Contribute

1.  **Fork the repository**: Create your own copy of the project on GitHub.
2.  **Create a new branch**: `git checkout -b feature/your-awesome-feature`
3.  **Make your changes**: Write your code and commit your changes with clear messages.
4.  **Push to your branch**: `git push origin feature/your-awesome-feature`
5.  **Open a Pull Request**: Submit a pull request from your forked repository to the main project.

### Reporting Bugs

If you find a bug, please open an issue on the GitHub repository. Include as much detail as possible:
* Steps to reproduce the bug.
* What you expected to happen.
* What actually happened.
* Screenshots or logs, if applicable.

---

## 📜 License

This project is licensed under the **Apache 2.0 License**.

### Why Apache 2.0?

The Apache 2.0 license was chosen to encourage adoption—including by commercial and industrial users—while providing legal clarity and maintaining open-source principles.

* **Permissive**: You are free to use, modify, and distribute the software for any purpose, commercial or non-commercial, without worrying about royalties.
* **Patent Grant**: It provides an express grant of patent rights from contributors to users, protecting you from patent litigation.
* **Attribution**: You must retain the original copyright and license notices in your distributed versions.
* **Modification Notice**: If you modify the code, you must include a notice of your changes.

For full license details, see the [LICENSE](./LICENSE) file.
