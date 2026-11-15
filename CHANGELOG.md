# 更新日志

> 本文件是 Release 发布说明的唯一来源：`.github/workflows/build.yml` 的 `release` 作业会抽取最上面一节（第一个 `## ` 到下一个 `## ` 之前）作为 GitHub Release 正文。
> 发版时请先在这里写好「本次更新升级内容」和「最低支持系统」，再打 `v*` 标签。

## v1.0.0 · 2026-11-15

### ✨ 本次更新升级内容

- 🔒 **自动保存不再静默丢内容**：写入改为串行队列，失败时编辑窗顶部常驻红色提示条并给出原因；后台按 2s→30s 指数退避自动重试；连续失败时把正文兜底落盘到恢复文件，重新打开便签自动回填并显示蓝色提示条。关闭窗口、切换便签、隐藏窗口都会等待保存真正完成。
- ⚡ **保存链路提速**：`update_note` 由「更新后重读全表再挑一行」改为按 id 读单行，便签越多保存越快，并补了单元测试。
- 🧹 **删除即彻底删除**（v0.9.4 起）：移除软删除与回收站概念，被删内容不再以明文留在库里。
- 📦 **恢复 Linux 安装包**：同一份 CI 现在一次产出 Windows `.exe`、Linux `.deb` 与 `.rpm`，并合并发布到同一个 Release。
- 🍎 **macOS 预留构建位**：本版本不提供 macOS 安装包，构建作业已在 CI 中预留注释开关。

### 💻 最低支持系统

| 安装包 | 适用系统 | 最低版本 | 依赖与说明 |
|--------|----------|----------|-----------|
| `Memo_1.0.0_x64-setup.exe` | Windows | **Windows 10 build 1809 及以上**（含 Windows 11） | 需 WebView2 Runtime（Win10 1803+ 随系统更新预装，Win11 自带）；安装器为 perMachine，需要管理员权限，界面语言简体中文 |
| `Memo_1.0.0_x_amd64.deb` | Debian / Ubuntu 系 | **Ubuntu 22.04、Debian 12 及以上** | 依赖 `libwebkit2gtk-4.1`、`libgtk-3`、AppIndicator3；Deepin / UOS / Kylin 若只带 WebKitGTK 4.0 则装不上 |
| `Memo-1.0.0-1.x86_64.rpm` | Fedora / RHEL / openSUSE 系 | **Fedora 38 及以上**（RHEL 9 系需系统自带 `webkit2gtk4.1`） | 依赖同上，包名为 `webkit2gtk4.1`、`gtk3`、`libappindicator-gtk3` |

Linux 补充说明：

- 🖥️ 桌面环境：KDE / XFCE 托盘直接可用；**GNOME（Ubuntu 默认桌面）需要安装 AppIndicator 扩展**才能看到托盘图标。
- 🖱️ 托盘交互：Linux 下左键点击图标会弹出菜单（appindicator 的限制），从菜单里选「打开Memo」；Windows / macOS 左键直接打开主窗口。
- 🗂️ 数据文件位置：Windows `%APPDATA%\com.memo.app\memo.db`，Linux `~/.local/share/com.memo.app/memo.db`。目前是**明文 SQLite**，同一登录账户下的进程可直接读取。

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
