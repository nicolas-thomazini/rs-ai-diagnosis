# 🧠 AI Medical Buddy

AI Medical Buddy is a CLI-based assistant written in Rust. It allows users to manage patients, record symptoms, and generate simulated diagnoses. It also features a free-chat mode powered by an AI language model.

## ✨ Features

- Manage patients by ID
- Add and track symptoms
- Generate mock diagnoses
- Free-form chat with an AI model
- Persistent in-session memory per patient
- Lightweight and runs locally

## 🛠️ Installation

Make sure you have [Rust](https://www.rust-lang.org/tools/install) and `cargo` installed.

### 1. Clone the repository

```bash
git clone https://github.com/your-username/rust-ai-buddy.git
cd rust-ai-buddy
```

## 2. Install dependencies

Install required crates using Cargo:

```bash
cargo build
```

## 3. Add environment variables

Create a .env file at the project root and add your API key (e.g., for the Mistral or OpenAI API):

```bash
MISTRAL_API_KEY=your_api_key_here
```

## 🚀 Usage

Run the assistant using:

```bash
cargo run
```

Once running, you can use the following commands:

- /q — Quit the application

- /patient <id> — Select or create a patient

- /add <symptom> — Add a symptom to the current patient

- /dx — Generate a mock diagnosis

- /save — Print saved data for the current patient

- Any other text — Send a free-form question to the AI

Example:

```bash
🤖 Enter a command or ask a question: /patient 001
📁 Selected patient: 001

🤖 Enter a command or ask a question: /add headache
✅ Symptom "headache" added to patient "001"

🤖 Enter a command or ask a question: /dx
🧠 Diagnosis generated for "001": Simulated diagnosis based on 1 symptom(s).
```

## 📁 Project Structure

```
.
├── Cargo.toml
├── crates/
│   └── ai-buddy/     # Main logic
├── .env              # Contains your API key
└── src/
    ├── main.rs       # CLI entrypoint
    ├── buddy.rs      # Buddy abstraction
    ├── utils.rs
    └── mistral.rs    # Chatbot integration
```

## 🧪 Requirements

**Rust Dependencies**

All Rust dependencies are declared in Cargo.toml, including:

- tokio

- serde

- dotenv

- simple-fs

- reqwest

- derive_more

To install them:

```bash
cargo build
```

## 🚧 TODO

Implement a trained model to predict a patient's diagnosis based on symptoms and test results.

## License

MIT License © [Nicolas Thomazini]
