# Cursor BYOK · Maintained by baichuan335
**Use your own model APIs in Cursor, with a local gateway to the providers you choose.**
```powershell
git clone https://github.com/baichuan335/cursor-byok.git
```

[简体中文](./README-CN.md) · [Releases & downloads](https://github.com/baichuan335/cursor-byok/releases) · [Report an issue](https://github.com/baichuan335/cursor-byok/issues) · [Contribute](https://github.com/baichuan335/cursor-byok/pulls)

[![Version](https://img.shields.io/badge/version-1.0.2-blue)](https://github.com/baichuan335/cursor-byok/releases)
[![License: MIT](https://img.shields.io/badge/license-MIT-green)](./LICENSE)

## About

**baichuan335** continues development and maintenance of this derivative of **leookun/cursor-byok**. The original attribution, commit history, and MIT license are preserved. Releases, support, and contributions for this derivative belong in this repository; users are not asked to move to another product.

Cursor BYOK combines a local Rust gateway with a React / Tauri desktop management interface. It adapts Cursor Agent requests, sends selected model calls to your configured OpenAI- or Anthropic-compatible API, and coordinates tool calls and conversation state.

> [!IMPORTANT]
> This project is free and open source, but model providers may charge for usage. It does not provide free model credits, replace a Cursor subscription, or guarantee every Cursor version or official feature. This project is not affiliated with or endorsed by Cursor or its developers.

## Implemented features

| Feature | Current capability |
| --- | --- |
| Model connections | OpenAI Chat Completions, OpenAI Responses, and Anthropic Messages-compatible APIs |
| Model management | Add, edit, duplicate, reorder, group, discover models, and batch-test connectivity |
| Request settings | Base or full URLs, API keys, context windows, output limits, reasoning/thinking options, custom headers, and extra JSON parameters |
| Agent interaction | Multi-turn conversations, tool calls, and tool results in the backend Agent flow; behavior depends on model and client compatibility |
| Call inspection | Call records, token and cache usage, latency, and cost estimates using configured rates; optional detailed recording |
| Runtime detection | Recognize unambiguous local-orchestration protocol mismatches, request safe client rerouting to the backend Agent, and show recovery instructions |
| Commit generation | Choose a configured model and edit prompts for commit messages, or use the official direct connection |
| TAB routing | Public service, official direct connection, or a custom service URL, configured separately from Agent model channels |
| Local external API | When enabled, call configured models at the displayed local address using a separate access key |
| Application settings | Outbound proxy, ports, themes, languages, startup behavior, and statistics storage management |

**Removed:** This application's former Plugin Configuration pages, account authorization, plugin model management and dedicated runtime, plus sidebar advertisements, ad popups, and related services. Cursor's own plugins, Skills, and MCP were not part of these removals; their availability still depends on Cursor, the account, and client configuration.

## Installation and upgrades

### Prerequisites

- An installed Cursor client; model API use requires a reachable provider endpoint and appropriate credentials.
- The current derivative version is **1.0.2**. A Windows 11 x64 NSIS installer has been built locally.
- The Windows desktop app requires WebView2 Runtime; install it through a trusted Microsoft channel if missing.
- Version 1.0.2 now includes GitHub Actions-built macOS Apple Silicon / Intel DMG installers and Linux x64 DEB / AppImage packages. Real-device, Cursor conversation, and automatic-update end-to-end acceptance testing remain incomplete. Mac builds are ad-hoc signed, without Apple Developer signing or notarization, and may trigger system security warnings.

### Get an installer

1. Open [this repository's Releases](https://github.com/baichuan335/cursor-byok/releases) and read the notes for the version you want.
2. Download only an asset actually provided for that version and suitable for your system. The local Windows 1.0.2 installer is named `Cursor BYOK_1.0.2_x64-setup.exe`.
3. Save your work in Cursor, quit the old Cursor BYOK app, then install and launch the new version. Also save your Cursor work before initial setup, upgrades, or enabling takeover: the takeover flow may terminate Cursor processes.
4. If this repository does not yet provide the asset you need, build from source below. A source version number or successful local build does not mean a GitHub Release has been published.

> [!WARNING]
> **Update signing configuration has been migrated; existing installers do not change with source edits.** Current source uses this maintainer's update public key, with both the standard and Windows portable update manifests pointing to this repository. The previously released 1.0.2 installer still contains the old configuration. Install the first build using the new configuration manually before relying on later maintained updates. The supplemental 1.0.2 Mac/Linux packages contain the new configuration and corresponding Tauri update signatures, but no automatic-update manifest was published with these additions. Install manually; end-to-end automatic download, signature verification, and installation remain unverified. Per-platform BUILD-*.txt files record the supplemental source, while the original version tag is unchanged. Tauri update signing is separate from Windows code signing or Apple signing and notarization.

Upgrades do not actively remove old plugin credentials, advertisement caches, or historical call records; the new version no longer uses resources dedicated to removed features. If Commit settings previously selected a plugin model, choose an existing model or **Direct** in **System settings → Commit Settings** and save. An invalid old selection will not silently switch to a potentially billable official service.

## First use

1. Launch Cursor BYOK and open **Models → Cursor** in the sidebar.
2. If the local CA is missing, click **Initialize CA** and follow the instructions to trust the local certificate. It is used for the local HTTPS proxy; protect its private key and never upload or share it.
3. Click **Add model**, enter its type, endpoint, API key, and model ID, save, then click **Test**.
4. After the test passes, save your work in Cursor, enable **Enable Cursor takeover**, and confirm the status changes to **Taken over**. Adding a model alone does not enable takeover.
5. Start or reopen Cursor and manually select your configured model. Do not select **Auto** to use a local model.
6. Send a short message in an existing or new conversation and check **Call history** for the matching request.

**Expected result:** The model replies, and the call record shows its status and usage. Keep Cursor BYOK running while using local models. To stop takeover, disable it on the Cursor page before quitting the app.

### Model fields

| Field | What to enter |
| --- | --- |
| Model type and protocol | Choose Anthropic Messages or OpenAI Responses / Chat Completions according to the provider's actual API; do not infer the protocol from a model name alone |
| Server URL | Use a base URL and let the app append the endpoint, or enable the full-URL option and provide the complete request URL |
| API key | The model provider's key, not the access key for the local external API |
| Model ID | An identifier accepted by the provider; use model discovery to query the list it exposes |
| Display name and notes | Presentation in the model list; these do not change the upstream model ID |
| Custom headers / extra parameters | JSON objects containing only fields supported by the selected provider and protocol |

A successful connectivity test does not establish compatibility with tool calls, long contexts, or every generation parameter. Validate a small task before relying on a model for real work.

### Other settings

- **Commit:** Choose a generation model and prompt in System settings. **Direct** uses Cursor's official service and depends on the account's permissions and billing.
- **TAB:** Choose the public service, direct connection, or a custom URL in System settings. The default public service is an upstream third-party service retained in the code, not a service operated or guaranteed by baichuan335. A custom service must implement compatible TAB endpoints; an ordinary chat API URL is not sufficient.
- **External API:** Disabled by default. Set a separate access key when enabling it and use the base URL shown in the interface. Do not expose the local management service directly to the internet.
- **Tutorial entry:** The application's **User guide** button still opens upstream documentation, which may differ from this maintained version. Use this README and this repository's release notes as the reference.

## Official accounts and compatibility limits

Configured models use local model channels; requests for models not matched to a local configuration can be forwarded to Cursor upstream. **Auto is not a local-model selector.** Official models, direct TAB connections, and other official features remain subject to account access, credits, and Cursor service policies. Connecting your own API does not unlock paid official features.

This project does not implement the complete client-local `RunInference` runtime. Runtime handling is based on the protocol and client behavior in Cursor **3.29.9**; it does not establish compatibility with every version or guarantee transparent recovery of every existing conversation. Keep the original conversation and first retry or resume when prompted.

## Troubleshooting

### Requests stall or report “Connection stalled repeatedly”

1. Confirm that the running application is a build containing the fixes, not just an older installation alongside edited source.
2. Check the model test, CA trust, takeover status, and selected model. Make sure you have not selected **Auto** by mistake.
3. If old logs contain `BidiAppend data_binary is not part of the captured protocol`, version 1.0.2 accepts both binary `data_binary` and hex-encoded `data` payloads. That error alone does not indicate an API key problem.
4. For protobuf decode errors mentioning `file_states` / `summary`, or a runtime incompatibility warning, follow the next section.
5. For other cases, inspect proxy, certificate, upstream response, and error logs from the same time. Do not hide a problem by only increasing timeouts.

### An existing conversation reports runtime incompatibility

The gateway disables local orchestration in feature configuration. For unambiguously identified incompatible requests, it returns a client-recognized denial signal, asking Cursor to safely reroute to the backend Agent using its own state. A matching downstream request still waiting for its first model route is also released when that first append is rejected. While visible, the assistant checks diagnostics every 10 seconds and on focus; new records trigger a notification, with recovery instructions on the Cursor page.

1. If Cursor is responding again, continue the original conversation. A detection record neither proves an ongoing failure nor confirms a successful automatic switch.
2. If Cursor prompts you to continue or resume, do so in the original conversation first.
3. If it still fails, save your work, fully quit and restart Cursor, reopen the original conversation, select the configured model again, and retry.
4. Keep the conversation and sanitized logs. Do not begin by deleting conversations, clearing the database, or clearing caches.

The client controls rerouting. If output has already started without a resumable checkpoint, or the backend cannot preserve tool restrictions, manual recovery may be required or rerouting may be refused. This handling does not migrate history or rewrite conversation databases. Automatic continuation of real existing conversations and the WebSocket path have not completed end-to-end acceptance testing.

### A model is missing or calls fail

Confirm takeover is enabled, the model was saved, and its model ID and protocol are correct. Check whether the provider supports streaming and the parameters in use. Insufficient official credits do not automatically select your local model; choose it manually.

## Data, security, and privacy

```text
Cursor → Local HTTPS proxy and Agent gateway → Your model API
                            ├→ SQLite: models, settings, calls, and checkpoints
                            └→ Cursor upstream / TAB service: routed by request and settings
```

Configuration and application data are stored in `.cursor-byok-v3` under the current user's home directory. Local storage does not mean offline operation: conversations, code, tool results, and other request content are sent to model providers, Cursor, or the selected TAB service as required by the feature. Detailed recording can also put sensitive content in local logs and call details.

- Protect API keys, CA private keys, databases, update signing keys, and detailed logs. Do not commit them or attach them to public issues.
- Keep personal backups secure; do not upload a complete database as a troubleshooting attachment.
- Cost metrics are estimates using your configured rates, not provider invoices.

## Build from source

### Prerequisites

Install Git, Node.js/npm, Rust/Cargo, and the Tauri 2 build dependencies for your platform. On Windows, use the Rust MSVC toolchain and install Visual Studio Build Tools with **Desktop development with C++**, the Windows SDK, and WebView2 Runtime.

Follow the [official Tauri prerequisites](https://v2.tauri.app/start/prerequisites/) for system dependencies. Compilation downloads dependencies and takes time; cloning the repository alone does not produce a runnable application.

### Local Windows installer (PowerShell)

Run this from the directory where you want the checkout. If it is already cloned, enter its existing `apps/desktop` directory instead:

```powershell
git clone https://github.com/baichuan335/cursor-byok.git
Set-Location cursor-byok/apps/desktop
npm.cmd ci
npm.cmd run check
npm.cmd run tauri:build:local
```

Run one line at a time and stop if any step fails. Do not run `npm install` from the repository root; the frontend's `package.json` is in `apps/desktop`.

The output is under `target/release/bundle/nsis/` at the repository root. The 1.0.2 installer is named `Cursor BYOK_1.0.2_x64-setup.exe`. If you explicitly pass `-- --target x86_64-pc-windows-msvc`, the path becomes `target/x86_64-pc-windows-msvc/release/bundle/nsis/`. Use the path printed in the build log and check the new file's timestamp.

`tauri:build:local` needs no GNU Make. An additional configuration disables updater signature artifacts, so no release signing private key is required. It does not change update endpoints or public keys, configure the derivative's automatic updates, or establish Windows code signing for the installer.

### Development and checks

After installing dependencies in `apps/desktop`, run:

```powershell
npm.cmd run tauri:dev
```

This starts the desktop development environment and may use the current user's application data and Cursor configuration; it is not an isolated sandbox. For frontend checks only, use `npm.cmd run check`.

From the repository root, check the Rust code:

```powershell
cargo fmt --all -- --check
cargo clippy -p cursor-server --all-targets -- -D warnings
cargo test -p cursor-server --test runtime_compatibility --test connect_wire --test external_api
cargo test --workspace --all-targets
```

On Windows, Git CRLF checkout can cause the `every_captured_mode_owns_and_renders_its_runtime_template` test in `prefix_stability` to fail an LF-only string assertion. This is a known check issue; do not describe a run with that test filtered out as an unmodified full-suite pass.

The repository retains a `Makefile`, but the Windows instructions above do not require Make. macOS/Linux desktop builds need their own Tauri dependencies and signing setup; the Windows command that only builds NSIS packages does not validate those platforms.

### Add Mac / Linux installers to an existing version

Maintainers can manually run **Supplement desktop installers** in GitHub Actions with an existing release version (without `v`) and a full source commit SHA already contained in `main`. The workflow checks the source version, builds macOS Apple Silicon, macOS Intel, and Linux x64 installers, then adds assets to the existing Release only after all builds and SHA-256 checks succeed.

This does not bump the version, move its tag, or overwrite the original Windows installer, checksum file, or update manifests. Each supplemental build records its source in a `BUILD-*.txt` asset. Existing asset name collisions stop the upload rather than overwriting files. Mac builds are ad-hoc signed, without Apple Developer signing or notarization. A successful build is not real-device or Cursor conversation acceptance testing.

## Project structure

```text
cursor-byok/
├── apps/desktop/     # React interface and Tauri desktop integration
├── server/           # Rust gateway, Agent execution, control API, and SQLite storage
├── crates/           # Shared Rust libraries, including code indexing and search
├── protocols/        # Cursor protocol definitions
├── support/          # Protocol extraction, capture, and benchmark tools
├── images/           # Project interface images
├── Cargo.toml        # Rust workspace
└── Makefile          # Development, checks, and build entry points
```

## Support and contributions

Report problems in [this repository's Issues](https://github.com/baichuan335/cursor-byok/issues) or submit a [Pull Request](https://github.com/baichuan335/cursor-byok/pulls). Include your operating system, Cursor and Cursor BYOK versions, model protocol, minimal reproduction steps, and sanitized error logs. Do not disclose credentials, private code, or conversation contents.

Update endpoints and the public key have been migrated in current source; the tutorial entry and default public TAB service still use upstream settings. Supplemental platform installers are published; actual automatic upgrades still require separate verification. Treat the current source, actual builds, and this repository's release notes as the record of available features, not planned work.

## Authors, origin, and license

[![Maintainer: baichuan335](https://img.shields.io/badge/maintainer-baichuan335-blue)](https://github.com/baichuan335)
[![Original author: leookun](https://img.shields.io/badge/original%20author-leookun-lightgrey)](https://github.com/leookun)

- **Derivative maintainer:** [baichuan335](https://github.com/baichuan335), maintaining [baichuan335/cursor-byok](https://github.com/baichuan335/cursor-byok).
- **Original author and source:** [leookun](https://github.com/leookun) / [leookun/cursor-byok](https://github.com/leookun/cursor-byok). Credit belongs to the original author and contributors; this derivative does not claim their work as an independent original creation.
- **Original copyright notice:** `Copyright (c) 2026 leookun`.
- **License:** [MIT License](./LICENSE). Copies or substantial portions of the software must retain the copyright and permission notices. See LICENSE for the full terms and disclaimer.
