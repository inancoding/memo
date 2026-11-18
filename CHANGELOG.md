# 更新日志

> 本文件是 Release 发布说明的唯一来源：`.github/workflows/build.yml` 的 `release` 作业会抽取最上面一节（第一个 `## ` 到下一个 `## ` 之前）作为 GitHub Release 正文。
> 发版时请先在这里写好「本次更新升级内容」和「最低支持系统」，再打 `v*` 标签。

## v1.1.3 · 2026-11-18

### ✨ 本次更新升级内容

- 🐛 **修复已删除便签的窗口重启后永远转圈**：开机自启恢复窗口时，如果便签在删除后未通过托盘正常退出（如直接关机/重启），恢复列表仍包含已删除便签的 ID，编辑窗口打开后找不到数据，加载转圈永远不消失且无法关闭。现在加载时检测到便签不存在会自动销毁窗口。

### 💻 最低支持系统

| 安装包 | 适用系统 | 最低版本 | 依赖与说明 |
|--------|----------|----------|-----------|
| `Memo_1.1.3_x64-setup.exe` | Windows | **Windows 10 build 1809 及以上**（含 Windows 11） | 需 WebView2 Runtime（Win10 1803+ 随系统更新预装，Win11 自带）；安装器为 perMachine，需要管理员权限，界面语言简体中文 / English |
| `Memo_1.1.3_amd64.deb` | Debian / Ubuntu 系 | **Ubuntu 22.04、Debian 12 及以上** | 依赖 `libwebkit2gtk-4.1-0`、`libgtk-3-0`、`libayatana-appindicator3-1`；已在 Ubuntu 24.04 与 26.04 核对依赖可解析（`libgtk-3-0` 由 `libgtk-3-0t64` 提供）。Deepin / UOS / Kylin 若只带 WebKitGTK 4.0 则装不上 |
| `Memo-1.1.3-1.x86_64.rpm` | Fedora / RHEL / openSUSE 系 | **Fedora 38 及以上**（RHEL 9 系需系统自带 `webkit2gtk4.1`） | 依赖 `libwebkit2gtk-4.1.so.0`、`libgtk-3.so.0`、`libayatana-appindicator-gtk3`；已在 Fedora 45 核对由 `webkit2gtk4.1`、`gtk3`、`libayatana-appindicator-gtk3` 提供 |

---

## v1.1.2 · 2026-11-18

### ✨ 本次更新升级内容

本版是架构升级与质量收尾，不加新功能。

- 🪟 **独立窗口模型**：每条便签现在打开专属的编辑窗口（`editor-<id>`），可同时编辑多条便签。前端通过 `WebviewWindow` 创建窗口，避开 Rust 侧 `build()` 挂死问题。开机自启时恢复上次所有未关闭的便签窗口。
- 🗑️ **移除批量删除**：多选模式与卡片勾选框已移除，界面更简洁。单条删除仍保留二次确认。
- 🧹 **清理调试日志**：前端 19 处 `console.log` 全部移除，后端 3 处 `println!` 改为 `eprintln!`（仅记录错误）。
- 📝 **PRD v1.5 同步**：文档更新反映独立窗口模型、批量删除移除、质量收尾完成情况。

### 💻 最低支持系统

| 安装包 | 适用系统 | 最低版本 | 依赖与说明 |
|--------|----------|----------|-----------|
| `Memo_1.1.2_x64-setup.exe` | Windows | **Windows 10 build 1809 及以上**（含 Windows 11） | 需 WebView2 Runtime（Win10 1803+ 随系统更新预装，Win11 自带）；安装器为 perMachine，需要管理员权限，界面语言简体中文 / English |
| `Memo_1.1.2_amd64.deb` | Debian / Ubuntu 系 | **Ubuntu 22.04、Debian 12 及以上** | 依赖 `libwebkit2gtk-4.1-0`、`libgtk-3-0`、`libayatana-appindicator3-1`；已在 Ubuntu 24.04 与 26.04 核对依赖可解析（`libgtk-3-0` 由 `libgtk-3-0t64` 提供）。Deepin / UOS / Kylin 若只带 WebKitGTK 4.0 则装不上 |
| `Memo-1.1.2-1.x86_64.rpm` | Fedora / RHEL / openSUSE 系 | **Fedora 38 及以上**（RHEL 9 系需系统自带 `webkit2gtk4.1`） | 依赖 `libwebkit2gtk-4.1.so.0`、`libgtk-3.so.0`、`libayatana-appindicator-gtk3`；已在 Fedora 45 核对由 `webkit2gtk4.1`、`gtk3`、`libayatana-appindicator-gtk3` 提供 |

Linux 补充说明：

- 🖥️ 桌面环境：KDE / XFCE 托盘直接可用；**GNOME（Ubuntu 默认桌面）需要安装 AppIndicator 扩展**才能看到托盘图标。
- 🖱️ 托盘交互：Linux 下左键点击图标会弹出菜单（appindicator 的限制），从菜单里选「打开Memo」；Windows 左键直接打开主窗口。
- 🧊 Linux 无毛玻璃主题，颜色栏从第一个实色（经典黄）开始，且不提供边缘自动隐藏，原因见 v1.0.2 说明。
- 🔲 窗口形状：Linux 下不绘制客户端圆角，主窗口与便签为直角矩形，若桌面环境开启全局圆角则由系统切角。
- 🗂️ 数据文件位置：Windows `%APPDATA%\com.memo.app\memo.db`，Linux `~/.local/share/com.memo.app/memo.db`。目前是**明文 SQLite**，同一登录账户下的进程可直接读取。

---

## v1.1.1 · 2026-11-17

### ✨ 本次更新升级内容

本版是问题修复，不加新功能；Windows 与 Linux 各修一处，界面与功能均无变化。

- 🐛 **修复旧便签点开不显示**：编辑窗恢复上次位置时，如果存档坐标正好落在屏幕边缘 24px 内，这次恢复自身触发的「窗口移动」会被当成已经贴边，窗口立刻被收进屏幕外，只剩一条几乎看不见的把手——表现为「点旧便签没反应、新建便签正常」，且坐标存在本机库里，重启和重装都无效。现在打开便签会复位收起状态并抑制这一次自动收起（等你真正拖动标题栏后贴边收起照常生效），存档坐标先夹回当前显示器的可见区域，越出屏幕的坐标也不再被判定为贴边。
- 🟦 **Linux 取消客户端圆角**：deepin（X11 会话）上 WebKitGTK 取不到窗口层的透明通道，圆角外本该透明的一圈像素被涂成黑色方块，打开设置时遮罩还会让它更显眼。Linux 下改为整面矩形铺满窗口，圆角交给桌面合成器处理；Windows / macOS 的圆角与阴影不变。
- 📦 **安装包补上简介**：deb 的 `Description`、rpm 的 `Summary` 与 `%description` 之前是空的，deepin 应用安装器只显示 `(none)`。现在填上「轻量桌面便签应用」与三行功能说明。

### 💻 最低支持系统

| 安装包 | 适用系统 | 最低版本 | 依赖与说明 |
|--------|----------|----------|-----------|
| `Memo_1.1.1_x64-setup.exe` | Windows | **Windows 10 build 1809 及以上**（含 Windows 11） | 需 WebView2 Runtime（Win10 1803+ 随系统更新预装，Win11 自带）；安装器为 perMachine，需要管理员权限，界面语言简体中文 / English |
| `Memo_1.1.1_amd64.deb` | Debian / Ubuntu 系 | **Ubuntu 22.04、Debian 12 及以上** | 依赖 `libwebkit2gtk-4.1-0`、`libgtk-3-0`、`libayatana-appindicator3-1`；已在 Ubuntu 24.04 与 26.04 核对依赖可解析（`libgtk-3-0` 由 `libgtk-3-0t64` 提供）。Deepin / UOS / Kylin 若只带 WebKitGTK 4.0 则装不上 |
| `Memo-1.1.1-1.x86_64.rpm` | Fedora / RHEL / openSUSE 系 | **Fedora 38 及以上**（RHEL 9 系需系统自带 `webkit2gtk4.1`） | 依赖 `libwebkit2gtk-4.1.so.0`、`libgtk-3.so.0`、`libayatana-appindicator-gtk3`；已在 Fedora 45 核对由 `webkit2gtk4.1`、`gtk3`、`libayatana-appindicator-gtk3` 提供 |

Linux 补充说明：

- 🖥️ 桌面环境：KDE / XFCE 托盘直接可用；**GNOME（Ubuntu 默认桌面）需要安装 AppIndicator 扩展**才能看到托盘图标。
- 🖱️ 托盘交互：Linux 下左键点击图标会弹出菜单（appindicator 的限制），从菜单里选「打开Memo」；Windows 左键直接打开主窗口。
- 🧊 Linux 无毛玻璃主题，颜色栏从第一个实色（经典黄）开始，且不提供边缘自动隐藏，原因见 v1.0.2 说明。
- 🔲 窗口形状：Linux 下不绘制客户端圆角，主窗口与便签为直角矩形，若桌面环境开启全局圆角则由系统切角。
- 🗂️ 数据文件位置：Windows `%APPDATA%\com.memo.app\memo.db`，Linux `~/.local/share/com.memo.app/memo.db`。目前是**明文 SQLite**，同一登录账户下的进程可直接读取。

## v1.1.0 · 2026-11-16

### ✨ 本次更新升级内容

本轮把 v1.0 之后规划的功能一次补齐，并修掉两个会直接影响使用的问题。

- 🔍 **便签内容搜索**：主窗口按标题与正文纯文本即时过滤，实时显示命中条数，`Esc` 收起。列表里的富文本会先剥掉标签与内嵌图片的 data URI，只比对看得见的文字。
- 🗑️ **批量删除**：卡片左上角勾选即进入多选，支持全选与取消，确认后一次删除多条。删除仍是彻底删除，不进回收站。
- 📌 **托盘菜单扩充**：新建便签、打开主窗口、开机自启动（勾选项）、退出应用。「开机自启动」勾选与设置里的开关读同一个系统状态，两边改动互相同步。
- 🌐 **中英文界面**：「设置 - 通用」新增语言下拉（跟随系统 / 简体中文 / English），切换后托盘菜单与欢迎便签同步换语言；NSIS 安装界面在简体中文之外增加 English。
- ⌨️ **编辑窗快捷键**：新增 `Ctrl+S` 立即保存（等待写入结果）、`Ctrl+W` 关闭当前编辑窗，并新增「设置 - 快捷键」页签列出自定义快捷键。置顶不做快捷键——`Ctrl+Shift+P` 与系统打印窗口重叠，只保留标题栏按钮。
- 🎨 **图标全量更换**：桌面快捷方式、安装包与托盘图标统一换成新图标。
- 🐛 **修复多选勾选框的钩画成小方块**：编辑窗正文里仿 Naive UI 的复选框样式没有加作用域，类名与 Naive UI 真组件相同，泄漏进主窗口后把勾选图标压成 8×8 并偏到左上角，现在只在正文作用域内生效。
- 🐛 **修复欢迎便签不随语言更新**：用于识别旧文案的历史常量取的是更早一版文本，与实际发布出去的并不一致，导致语言切换与文案升级对老数据一条都没生效。
- 🧪 **新增单元测试**：`cargo test` 共 5 项，覆盖 `update_note` 单行读取、兜底文件路径、欢迎便签原地刷新只改未被用户编辑过的那条。

### 💻 最低支持系统

| 安装包 | 适用系统 | 最低版本 | 依赖与说明 |
|--------|----------|----------|-----------|
| `Memo_1.1.0_x64-setup.exe` | Windows | **Windows 10 build 1809 及以上**（含 Windows 11） | 需 WebView2 Runtime（Win10 1803+ 随系统更新预装，Win11 自带）；安装器为 perMachine，需要管理员权限，界面语言简体中文 / English |
| `Memo_1.1.0_amd64.deb` | Debian / Ubuntu 系 | **Ubuntu 22.04、Debian 12 及以上** | 依赖 `libwebkit2gtk-4.1-0`、`libgtk-3-0`、`libayatana-appindicator3-1`；已在 Ubuntu 24.04 与 26.04 核对依赖可解析（`libgtk-3-0` 由 `libgtk-3-0t64` 提供）。Deepin / UOS / Kylin 若只带 WebKitGTK 4.0 则装不上 |
| `Memo-1.1.0-1.x86_64.rpm` | Fedora / RHEL / openSUSE 系 | **Fedora 38 及以上**（RHEL 9 系需系统自带 `webkit2gtk4.1`） | 依赖 `libwebkit2gtk-4.1.so.0`、`libgtk-3.so.0`、`libayatana-appindicator-gtk3`；已在 Fedora 45 核对由 `webkit2gtk4.1`、`gtk3`、`libayatana-appindicator-gtk3` 提供 |

Linux 补充说明：

- 🖥️ 桌面环境：KDE / XFCE 托盘直接可用；**GNOME（Ubuntu 默认桌面）需要安装 AppIndicator 扩展**才能看到托盘图标。
- 🖱️ 托盘交互：Linux 下左键点击图标会弹出菜单（appindicator 的限制），从菜单里选「打开Memo」；Windows 左键直接打开主窗口。
- 🧊 Linux 无毛玻璃主题，颜色栏从第一个实色（经典黄）开始，且不提供边缘自动隐藏，原因见 v1.0.2 说明。
- 🗂️ 数据文件位置：Windows `%APPDATA%\com.memo.app\memo.db`，Linux `~/.local/share/com.memo.app/memo.db`。目前是**明文 SQLite**，同一登录账户下的进程可直接读取。

## v1.0.2 · 2026-11-15

### ✨ 本次更新升级内容

本版是根据 deepin 23/25 实测反馈做的 Linux 修复，Windows 端功能与界面均无改动。

- 🧊 **Linux 取消毛玻璃主题**：颜色栏不再出现玻璃色圆点，新建便签默认使用颜色栏第一个实色（经典黄），已有的玻璃便签在 Linux 上也按第一个实色渲染。原因是 WebKitGTK 在透明窗口里合成 `backdrop-filter` 会采到不透明白底——工具栏与颜色栏浮出近白色块，且每次聚焦 / 失焦重绘后整窗越来越白。
- 🟦 **顺带消失的浮层白块**：编辑窗置顶 / 关闭按钮 hover 时出现的浅色底块，随窗口改为整面不透明而不再出现。
- 📐 **Linux 关闭边缘自动隐藏**：deepin 的合成器会把程序化移动夹回屏幕内，窗口贴边后只露出把手条、本体仍停在原地，因此该功能在 Linux 上停用，设置面板同步隐藏这个开关（Windows 不受影响）。
- 🍎 本版本仍不提供 macOS 安装包，兼容性评估记录在 PRD §2.4。

### 💻 最低支持系统

| 安装包 | 适用系统 | 最低版本 | 依赖与说明 |
|--------|----------|----------|-----------|
| `Memo_1.0.2_x64-setup.exe` | Windows | **Windows 10 build 1809 及以上**（含 Windows 11） | 需 WebView2 Runtime（Win10 1803+ 随系统更新预装，Win11 自带）；安装器为 perMachine，需要管理员权限，界面语言简体中文 |
| `Memo_1.0.2_amd64.deb` | Debian / Ubuntu 系 | **Ubuntu 22.04、Debian 12 及以上** | 依赖 `libwebkit2gtk-4.1-0`、`libgtk-3-0`、`libayatana-appindicator3-1`；已在 Ubuntu 24.04 与 26.04 核对依赖可解析（`libgtk-3-0` 由 `libgtk-3-0t64` 提供）。Deepin / UOS / Kylin 若只带 WebKitGTK 4.0 则装不上 |
| `Memo-1.0.2-1.x86_64.rpm` | Fedora / RHEL / openSUSE 系 | **Fedora 38 及以上**（RHEL 9 系需系统自带 `webkit2gtk4.1`） | 依赖 `libwebkit2gtk-4.1.so.0`、`libgtk-3.so.0`、`libayatana-appindicator3.so.1`；已在 Fedora 45 核对由 `webkit2gtk4.1`、`gtk3`、`libayatana-appindicator-gtk3` 提供 |

Linux 补充说明：

- 🖥️ 桌面环境：KDE / XFCE 托盘直接可用；**GNOME（Ubuntu 默认桌面）需要安装 AppIndicator 扩展**才能看到托盘图标。
- 🖱️ 托盘交互：Linux 下左键点击图标会弹出菜单（appindicator 的限制），从菜单里选「打开Memo」；Windows 左键直接打开主窗口。
- 🗂️ 数据文件位置：Windows `%APPDATA%\com.memo.app\memo.db`，Linux `~/.local/share/com.memo.app/memo.db`。目前是**明文 SQLite**，同一登录账户下的进程可直接读取。

## v1.0.1 · 2026-11-15

### ✨ 本次更新升级内容

本版只调整打包与发布链路，应用功能与 v1.0.0 完全一致（`src/` 与 `src-tauri/src/` 无代码改动）。

- 📦 **新增 Linux 安装包**：一次构建同时产出 `.deb` 与 `.rpm`，v1.0.0 只发布了 Windows 安装包。
- 🧩 **CI 合并为单一工作流**：`build.yml` 一个文件同时构建 Windows 与 Linux，并汇总发布到同一个 Release。
- 🧱 **固定 Linux 构建基线**：改用 Ubuntu 22.04 容器构建，二进制 glibc 下限锁定 2.35，让包能装进 Ubuntu 22.04 / Debian 12。
- 📝 **发布说明改为人工撰写**：Release 正文取自本文件最上面一节，不再使用自动生成。
- 🍎 **macOS 兼容性评估**：结论写入 PRD §2.4，CI 中预留构建位；本版本仍不提供 macOS 安装包。

### 💻 最低支持系统

| 安装包 | 适用系统 | 最低版本 | 依赖与说明 |
|--------|----------|----------|-----------|
| `Memo_1.0.1_x64-setup.exe` | Windows | **Windows 10 build 1809 及以上**（含 Windows 11） | 需 WebView2 Runtime（Win10 1803+ 随系统更新预装，Win11 自带）；安装器为 perMachine，需要管理员权限，界面语言简体中文 |
| `Memo_1.0.1_amd64.deb` | Debian / Ubuntu 系 | **Ubuntu 22.04、Debian 12 及以上** | 依赖 `libwebkit2gtk-4.1-0`、`libgtk-3-0`、`libayatana-appindicator3-1`；已在 Ubuntu 24.04 与 26.04 核对依赖可解析（`libgtk-3-0` 由 `libgtk-3-0t64` 提供）。Deepin / UOS / Kylin 若只带 WebKitGTK 4.0 则装不上 |
| `Memo-1.0.1-1.x86_64.rpm` | Fedora / RHEL / openSUSE 系 | **Fedora 38 及以上**（RHEL 9 系需系统自带 `webkit2gtk4.1`） | 依赖 `libwebkit2gtk-4.1.so.0`、`libgtk-3.so.0`、`libayatana-appindicator3.so.1`；已在 Fedora 45 核对由 `webkit2gtk4.1`、`gtk3`、`libayatana-appindicator-gtk3` 提供 |

Linux 补充说明：

- 🖥️ 桌面环境：KDE / XFCE 托盘直接可用；**GNOME（Ubuntu 默认桌面）需要安装 AppIndicator 扩展**才能看到托盘图标。
- 🖱️ 托盘交互：Linux 下左键点击图标会弹出菜单（appindicator 的限制），从菜单里选「打开Memo」；Windows / macOS 左键直接打开主窗口。
- 🗂️ 数据文件位置：Windows `%APPDATA%\com.memo.app\memo.db`，Linux `~/.local/share/com.memo.app/memo.db`。目前是**明文 SQLite**，同一登录账户下的进程可直接读取。

## v1.0.0 · 2026-11-11

### ✨ 本次更新升级内容

- 🔒 **自动保存不再静默丢内容**：写入改为串行队列，失败时编辑窗顶部常驻红色提示条并给出原因；后台按 2s→30s 指数退避自动重试；连续失败时把正文兜底落盘到恢复文件，重新打开便签自动回填并显示蓝色提示条。关闭窗口、切换便签、隐藏窗口都会等待保存真正完成。
- ⚡ **保存链路提速**：`update_note` 由「更新后重读全表再挑一行」改为按 id 读单行，便签越多保存越快，并补了单元测试。
- 🧹 **删除即彻底删除**（v0.9.4 起）：移除软删除与回收站概念，被删内容不再以明文留在库里。

> 本版本只发布了 Windows 安装包，Linux 包由 v1.0.1 恢复。

## v0.9.4 · 2026-11-11

- 🗑️ 便签改为彻底物理删除，移除回收站与软删除流程。

## v0.9.3 · 2026-11-11

- ↔️ 恢复无边框窗口的边缘/四角拖拽调整大小（改为 JS 热区驱动 `startResizeDragging`）。
- 🔲 修复编辑窗口在 Windows 10 上的圆角与实线边框；边缘隐藏的把手条颜色改为跟随便签配色。

## v0.9.2 · 2026-11-09

- 🔲 修复主窗口在 Windows 10 上的直角边框（开启透明、关闭系统阴影）。
- ⚠️ 构建目标收缩为仅 Windows，移除 Linux CI（该决定已由 v1.0.0 撤销）。

## v0.9.1 · 2026-11-08

- 🎨 修复编辑窗口毛玻璃与纯色配色切换时内容跳动。

## v0.9.0 · 2026-11-07

- 🪟 移除原生标题栏行为，屏蔽右键菜单，新建便签后自动打开编辑窗口。

## v0.8.x · 2026-11-06 ~ 11-07

- 🐧 Linux（deepin）适配：禁用 WebKitGTK DMABUF 渲染器解决窗口白屏，修复 4 处 UI 问题。
- 🚀 启动加载动画、移除阻塞渲染的远程字体；安装包中文化与自定义图标；单实例限制；八色调色板与编辑改动实时同步主列表。
