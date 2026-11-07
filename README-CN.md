# Cursor BYOK · baichuan335 维护版
**在 Cursor 中使用自己的模型 API，由本地网关连接你选择的服务商。**
```powershell
git clone https://github.com/baichuan335/cursor-byok.git
```

[English](./README.md) · [版本与下载](https://github.com/baichuan335/cursor-byok/releases) · [问题反馈](https://github.com/baichuan335/cursor-byok/issues) · [提交贡献](https://github.com/baichuan335/cursor-byok/pulls)

[![Version](https://img.shields.io/badge/version-1.0.2-blue)](https://github.com/baichuan335/cursor-byok/releases)
[![License: MIT](https://img.shields.io/badge/license-MIT-green)](./LICENSE)

## 项目介绍

本仓库由 **baichuan335** 基于 **leookun/cursor-byok** 继续二次开发和维护。项目保留原作者署名、提交历史和 MIT 许可证；本仓库的版本、问题反馈和贡献以本仓库为准，不要求用户迁移到其他产品。

Cursor BYOK 包含本地 Rust 网关和 React / Tauri 桌面管理界面。它适配 Cursor 的 Agent 请求，将选定模型的调用发送到你配置的 OpenAI 或 Anthropic 兼容 API，并协调工具调用与对话状态。

> [!IMPORTANT]
> 本项目免费且开源，但模型服务商可能按用量收费。它不提供免费模型额度，不代替 Cursor 官方订阅，也不保证所有 Cursor 版本和官方功能均可用。本项目与 Cursor 及其开发者没有隶属或认可关系。

## 已实现的功能

| 功能 | 当前能力 |
| --- | --- |
| 模型接入 | OpenAI Chat Completions、OpenAI Responses 和 Anthropic Messages 兼容接口 |
| 模型管理 | 添加、编辑、复制、排序、分组、获取模型列表和批量连通性测试 |
| 请求配置 | 基础地址或完整 URL、API Key、上下文窗口、输出上限、推理/思考参数、自定义 Headers 与额外 JSON 参数 |
| Agent 交互 | 后端 Agent 流程中的多轮对话、工具调用与结果回传；实际效果取决于模型和客户端兼容性 |
| 调用观察 | 调用记录、Token 与缓存用量、耗时和按配置单价估算的费用；可选择记录详细内容 |
| 运行流程检测 | 识别明确的本地编排协议不匹配，请求客户端安全切回后端 Agent，并显示恢复说明 |
| Commit 生成 | 选择已配置的模型生成提交信息、编辑提示词，或选择官方直连 |
| TAB 连接设置 | 公共服务、官方直连和自定义服务地址；与 Agent 模型通道分开配置 |
| 本机外部 API | 启用后使用独立访问密钥，通过界面显示的本机地址调用已配置模型 |
| 应用设置 | 出站代理、端口、主题、语言、开机启动和统计记录管理 |

**已移除：** 本项目原有的「插件配置」页面、账号授权、插件模型管理及专用运行时，以及侧边栏广告、广告弹窗和相关服务。Cursor 自身的插件、Skills 和 MCP 不属于这次删除范围，但其可用性仍由 Cursor、账号及客户端配置决定。

## 安装与升级

### 环境准备

- 已安装 Cursor；使用模型 API 需要可访问的服务地址和相应凭据。
- 当前二开版本为 **1.0.2**，已在 Windows 11 x64 本机构建 NSIS 安装包。
- Windows 桌面运行需要 WebView2 Runtime；缺少时请通过可信的 Microsoft 渠道安装。
- 1.0.2 已通过 GitHub Actions 补充构建并发布 macOS Apple Silicon / Intel 的 DMG，以及 Linux x64 的 DEB / AppImage。真实设备、Cursor 会话及自动升级尚未完成端到端验收。Mac 包仅临时签名，没有 Apple 开发者签名或公证，系统可能显示安全提示。

### 获取安装包

1. 打开[本仓库 Releases](https://github.com/baichuan335/cursor-byok/releases)，阅读对应版本说明。
2. 只下载该版本实际提供、且适合你系统的附件。Windows 1.0.2 本地安装包名称为 `Cursor BYOK_1.0.2_x64-setup.exe`。
3. 保存 Cursor 中的工作，退出旧版 Cursor BYOK 后再安装并启动新版。首次配置、升级或开启接管前也请保存 Cursor 的工作，因为接管流程可能结束 Cursor 进程。
4. 若本仓库尚无所需附件，按下文从源码构建；源码版本号和本地构建成功不代表 GitHub Release 已发布。

> [!WARNING]
> **更新签名配置已迁移，现有安装包不会随源码自动改变。** 当前源码已使用本维护者的更新公钥，通用更新清单和 Windows 便携更新清单均指向本仓库。此前发布的 1.0.2 安装包仍含旧配置，需要手动安装首个使用新配置构建的版本，才能使用后续维护版自动更新。1.0.2 新增的 Mac/Linux 包已含新配置和对应 Tauri 更新签名，但本次未发布自动更新清单；请手动下载安装，实际自动更新下载、验签和安装的端到端流程仍待验证。各平台 `BUILD-*.txt` 记录补充构建来源，原版本标签不变；Tauri 更新签名不是 Windows 代码签名或 Apple 签名、公证。

升级不会主动清理旧插件凭据、广告缓存或历史调用记录，新版也不再使用已删除功能的专用资源。如果 Commit 设置曾选择插件模型，请在 **系统设置 → Commit 提交代码模型设置** 中改选现有模型或「直连」并保存；无效的旧选择不会自动切换到收费的官方服务。

## 首次使用

1. 启动 Cursor BYOK，进入侧边栏 **模型配置 → Cursor**。
2. 若提示未初始化 CA，点击 **初始化 CA**，按界面提示完成本机证书信任操作。该证书用于本地 HTTPS 代理，请妥善保护其私钥，不要上传或分享。
3. 点击 **添加模型**，填写模型类型、地址、API Key 和模型名称，保存后点击 **测试**。
4. 测试通过后，保存 Cursor 中的工作，再打开 **开启接管Cursor** 开关；确认状态显示 **已接管**。仅添加模型并不等于已开启接管。
5. 启动或重新打开 Cursor，在模型列表中手动选择已配置的模型，不要选择 **Auto** 来调用本地模型。
6. 在原会话或新会话发送一条简短消息，同时查看 **调用详细** 是否出现对应请求。

**预期结果：** 模型返回回复，调用记录显示请求状态和用量。使用本地模型时应保持 Cursor BYOK 运行。要停止接管，先在 Cursor 页面关闭接管开关，再退出应用。

### 模型字段怎么填

| 字段 | 填写方式 |
| --- | --- |
| 模型类型与协议 | 按服务商实际支持的接口选择 Anthropic Messages，或 OpenAI 的 Responses / Chat Completions；不要仅凭模型名称推断协议 |
| 服务器地址 | 填写基础地址并由应用追加端点，或启用完整 URL 选项后填写完整请求地址 |
| API Key | 填写模型服务商的密钥，不是本机外部 API 的访问密钥 |
| 模型名称 | 服务商接受的模型 ID；可使用「获取模型」查询对方提供的列表 |
| 显示名称与备注 | 用于模型列表展示，不会改变上游模型 ID |
| 自定义 Headers / 额外参数 | 使用 JSON 对象，仅填写当前服务商和协议支持的字段 |

连通性测试通过不代表模型的工具调用、长上下文或所有生成参数均兼容，正式使用前请用小任务验证。

### 其他设置

- **Commit：** 在系统设置中选择生成模型和提示词。「直连」使用 Cursor 官方服务，权限与费用由官方账号决定。
- **TAB：** 在系统设置中选择公共服务、直连或自定义地址。默认公共服务是代码中沿用的上游第三方服务，不是 baichuan335 自建或承诺持续可用的服务；自定义模式要求兼容 TAB 接口，不是普通聊天 API 地址。
- **外部 API：** 默认关闭；启用时设置独立访问密钥，并使用界面显示的基础地址。不要将本机管理服务直接暴露到公网。
- **教程入口：** 当前应用中的「使用教程」仍打开上游文档，内容可能与本维护版不同。请以本 README 和本仓库版本说明为准。

## 官方账号与兼容性边界

你配置的模型走本地模型通道；未匹配到本地配置的模型请求可以转发到 Cursor 上游。**Auto 不是本地模型自动选择器**，官方模型、TAB 直连和其他官方功能仍受账号、额度及 Cursor 服务策略限制。接入自己的 API 不会自动获得官方付费功能。

本项目没有实现完整的客户端本地编排 `RunInference` 运行时。运行流程修复依据 Cursor **3.29.9** 的协议和客户端行为，不能据此宣称兼容所有版本，也不能保证所有旧会话透明恢复。请保留原会话，优先重试或按客户端提示继续。

## 常见问题

### 一直等待或提示「Connection stalled repeatedly」

1. 确认安装并运行的是包含修复的构建，而不是只修改了源码。
2. 检查模型测试、CA 信任、接管状态和所选模型；确认没有误选 **Auto**。
3. 若旧日志出现 `BidiAppend data_binary is not part of the captured protocol`，1.0.2 已支持二进制 `data_binary` 和十六进制 `data` 两种上行载荷。该错误本身不说明 API Key 有问题。
4. 若出现 `file_states` / `summary` 的 protobuf 解码错误或运行流程不兼容提示，按下一节操作。
5. 其他情况请结合相同时间段的代理、证书、上游响应与错误日志排查，不要仅通过增加超时掩盖问题。

### 旧会话出现运行流程不兼容

网关会在功能配置中关闭本地编排；对明确识别出的不兼容请求，返回客户端能识别的拒绝信号，请求客户端带着自身状态安全切回后端 Agent。对应首包被拒绝、尚未建立模型路由的下行请求也会结束等待。助手可见时每 10 秒检查诊断状态，并在获得焦点时检查；新记录会触发提示，恢复说明位于 Cursor 页面。

1. 若已正常回复，直接继续原会话；检测记录不等于仍在故障中，也不等于自动切换已成功。
2. 若 Cursor 提示继续或恢复，先在原会话中操作。
3. 若仍失败，保存工作，完全退出并重启 Cursor，打开原会话，重新选择已配置的模型再试。
4. 保留会话和脱敏日志，不要先删除会话、清空数据库或缓存。

切换由客户端决定；已有输出但缺少可恢复检查点，或后端不能保留工具限制时，可能需要手动恢复或拒绝切换。该处理不迁移历史，也不改写会话数据库。真实旧会话的自动续聊与 WebSocket 链路尚未完成端到端验收。

### 找不到模型或模型调用失败

确认接管已开启、模型已保存、模型 ID 和请求协议正确。检查服务商是否支持流式输出及所用参数。官方额度不足不会自动改用你的本地模型，请手动选择。

## 数据、安全与隐私

```text
Cursor → 本机 HTTPS 代理与 Agent 网关 → 你配置的模型 API
                        ├→ SQLite：模型、设置、调用与检查点数据
                        └→ Cursor 上游 / TAB 服务：按请求与设置路由
```

配置和应用数据存放在当前用户目录下的 `.cursor-byok-v3`。本地存储不等于离线：对话、代码、工具结果及其他请求内容会按功能发送给模型服务商、Cursor 或所选 TAB 服务。启用详细记录时，本机日志和调用详情也可能包含敏感内容。

- 保护 API Key、CA 私钥、数据库、更新签名私钥和详细日志；不要提交到仓库或附在公开 Issue 中。
- 备份个人数据时妥善保管，不要将完整数据库作为排障附件公开。
- 费用指标按你配置的单价估算，不是服务商账单。

## 从源码构建

### 前提

需要 Git、Node.js/npm、Rust/Cargo，以及对应平台的 Tauri 2 构建依赖。Windows 使用 Rust MSVC 工具链，安装 Visual Studio Build Tools 的 **使用 C++ 的桌面开发**、Windows SDK 和 WebView2 Runtime。

请根据 [Tauri 官方前置要求](https://v2.tauri.app/start/prerequisites/) 准备系统依赖。编译需要下载依赖并花费一定时间，并非克隆后即可直接运行。

### Windows 本地安装包（PowerShell）

以下示例从准备存放项目的目录开始；如已克隆，直接进入现有的 `apps/desktop`：

```powershell
git clone https://github.com/baichuan335/cursor-byok.git
Set-Location cursor-byok/apps/desktop
npm.cmd ci
npm.cmd run check
npm.cmd run tauri:build:local
```

逐行执行，某一步失败就先停止处理。不要在仓库根目录运行 `npm install`，前端的 `package.json` 位于 `apps/desktop`。

构建输出位于仓库根目录的 `target/release/bundle/nsis/`，1.0.2 文件名为 `Cursor BYOK_1.0.2_x64-setup.exe`。如显式传入 `-- --target x86_64-pc-windows-msvc`，路径会变成 `target/x86_64-pc-windows-msvc/release/bundle/nsis/`；以构建日志给出的路径和新文件时间为准。

`tauri:build:local` 不需要 GNU Make，通过附加配置关闭 updater 签名产物，不要求发布签名私钥，也不会更换更新源或公钥。它不等于配置好了二开自动更新，也不代表安装包具有 Windows 代码签名。

### 开发与检查

在 `apps/desktop` 安装依赖后，可使用：

```powershell
npm.cmd run tauri:dev
```

它启动桌面开发环境，可能使用当前用户的应用数据和 Cursor 配置，不是隔离沙箱。仅做前端检查可运行 `npm.cmd run check`。

从仓库根目录检查 Rust 代码：

```powershell
cargo fmt --all -- --check
cargo clippy -p cursor-server --all-targets -- -D warnings
cargo test -p cursor-server --test runtime_compatibility --test connect_wire --test external_api
cargo test --workspace --all-targets
```

Windows 的 Git CRLF 检出可能使 `prefix_stability` 中的 `every_captured_mode_owns_and_renders_its_runtime_template` 测试在 LF 字符串断言处失败。这是已知检查问题，不应把过滤后的测试结果描述成原样全量通过。

仓库保留 `Makefile`，但 Windows 上述步骤不依赖 Make。macOS/Linux 桌面构建需额外满足各自 Tauri 依赖及签名要求，不能使用这个仅打 NSIS 包的 Windows 命令代替平台构建验证。

### 为已有版本补充 Mac / Linux 安装包

维护者可在 GitHub Actions 中手动运行 **Supplement desktop installers**，填写已发布版本号（不带 `v`）和已包含在 `main` 中的完整源码提交 SHA。工作流会验证源码版本，构建 macOS Apple Silicon、macOS Intel 和 Linux x64 安装包，在全部成功并检查 SHA-256 后向现有 Release 新增附件。

该流程不提升版本号、不移动已有标签，不覆盖原 Windows 安装包、校验文件或更新清单。同一版本的补充构建来源通过 `BUILD-*.txt` 附件记录；如果附件同名已存在，上传会停止而不是覆盖。Mac 仅临时签名，不含 Apple 开发者签名或公证。构建成功不等于已完成真实设备和 Cursor 会话验收。

## 项目结构

```text
cursor-byok/
├── apps/desktop/     # React 界面与 Tauri 桌面集成
├── server/           # Rust 网关、Agent 运行、管理 API 与 SQLite 存储
├── crates/           # 共享 Rust 库，包括代码索引与搜索核心
├── protocols/        # Cursor 协议定义
├── support/          # 协议提取、抓包与基准测试工具
├── images/           # 项目界面图片
├── Cargo.toml        # Rust 工作区
└── Makefile          # 开发、检查与构建入口
```

## 反馈与贡献

请在[本仓库 Issues](https://github.com/baichuan335/cursor-byok/issues) 反馈问题，或提交 [Pull Request](https://github.com/baichuan335/cursor-byok/pulls)。报告时提供操作系统、Cursor 和 Cursor BYOK 版本、模型协议、最小复现步骤及脱敏错误日志，不要公开凭据、私人代码或对话正文。

更新源和公钥已在当前源码中迁移；教程入口和默认 TAB 公共服务仍沿用上游配置。补充平台安装包已发布，实际自动升级仍需单独验证。功能是否已经提供，以当前源码、实际构建和本仓库发布说明为准，不将计划作为已有能力。

## 作者、来源与许可证

[![Maintainer: baichuan335](https://img.shields.io/badge/maintainer-baichuan335-blue)](https://github.com/baichuan335)
[![Original author: leookun](https://img.shields.io/badge/original%20author-leookun-lightgrey)](https://github.com/leookun)

- **二开维护者：** [baichuan335](https://github.com/baichuan335)，维护仓库为 [baichuan335/cursor-byok](https://github.com/baichuan335/cursor-byok)。
- **原作者与项目来源：** [leookun](https://github.com/leookun) / [leookun/cursor-byok](https://github.com/leookun/cursor-byok)。感谢原作者及原项目贡献者，本维护版不将原项目成果宣称为独立原创。
- **原始版权声明：** `Copyright (c) 2026 leookun`。
- **许可证：** [MIT License](./LICENSE)。分发本软件或其实质部分时，须保留版权声明与许可声明；完整授权条件和免责声明以 LICENSE 为准。
