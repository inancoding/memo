# Memo 便签

一款轻量级桌面便签应用，基于 Tauri 2 + Vue 3 构建，体积小、启动快、数据完全本地存储。

## 功能特性

- 📝 **便签管理** — 主窗口便签列表，卡片颜色与便签颜色实时同步，支持置顶、删除（带确认）与勾选多条的批量删除
- 🔍 **内容搜索** — 按标题与正文纯文本即时过滤，实时显示命中条数，`Esc` 收起
- 🎨 **8 种便签配色** — 毛玻璃效果 + 7 种柔和色彩，浅色/深色模式下自动适配（Linux 取消毛玻璃，从第一个实色开始）
- 🪟 **独立编辑窗口** — 自定义富文本编辑器，支持有序/无序列表、复选框、拖入图片并缩放、网址/邮箱/手机号自动高亮，窗口位置大小自动记忆
- 📌 **固定到桌面** — 编辑窗口可置顶显示；开启边缘自动隐藏后贴屏幕边缘收起为窄条
- 🌗 **主题模式** — 系统 / 浅色 / 深色三档切换，多窗口同步，标题栏按钮一键循环
- 🌐 **中英文界面** — 跟随系统或在「设置 - 通用」指定，系统托盘菜单与欢迎便签同步换语言
- ⌨️ **快捷键** — 全局 `Ctrl+Shift+N` 新建、`Ctrl+Shift+M` 显示/隐藏主窗口；编辑窗 `Ctrl+S` 立即保存、`Ctrl+W` 关闭
- 🔒 **单实例运行** — 系统内只运行一个 Memo，重复启动时唤起已有窗口
- 🚀 **开机自启** — 设置开关与托盘勾选项共用同一状态，自启时自动恢复上次打开的便签
- 💾 **本地存储** — SQLite（rusqlite），数据保存在本机，不上传任何内容；自动保存失败时内容保留并落盘兜底
- 🖥️ **系统托盘** — 新建便签、打开主窗口、开关开机自启、退出应用

## 技术栈

| 层 | 技术 |
| --- | --- |
| 桌面框架 | [Tauri 2](https://tauri.app/) (Rust) |
| 前端 | Vue 3 + TypeScript + Vite 6 + Pinia |
| UI | [Naive UI](https://www.naiveui.com/) + UnoCSS |
| 数据库 | SQLite (rusqlite, bundled) |

## 开发

环境要求：[Node.js 20+](https://nodejs.org/)、[Rust stable](https://rustup.rs/)，以及 Tauri 的[平台依赖](https://tauri.app/start/prerequisites/)（Windows 需 WebView2，Linux 需 webkit2gtk-4.1 等）。

```bash
# 安装依赖
npm install

# 启动开发环境
npm run tauri dev
```

## 构建

```bash
# Windows：NSIS 安装包（中文 / English 安装界面，默认安装到 Program Files）
npm run tauri build -- --bundles nsis

# Linux：deb / rpm
npm run tauri build
```

产物位于 `src-tauri/target/release/bundle/`。

## CI

GitHub Actions 在 push 到 `master`、`v*` 标签及 PR 时自动构建：

- `Build Windows` — windows-latest，产出 NSIS 安装包
- `Build Linux` — ubuntu-22.04，产出 deb / rpm 包

可在 [Actions](https://github.com/inancoding/memo/actions) 运行页面下载构建产物。

## 许可证

[MIT](LICENSE) © 2026 nancoding
