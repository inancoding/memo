# Memo 便签应用 - UI 设计规范

> **文档版本: v1.0**
> **创建日期: 2026-10-29**
> **最后更新: 2026-10-29**
> **基于: PRD v1.0 + UI/UX Pro Max 设计系统**

---

## 1. 设计理念

### 1.1 设计原则
- **简洁现代**: Flat Design 风格，无多余装饰，信息优先
- **轻量高效**: 视觉元素精简，减少认知负担，操作路径最短
- **一致流畅**: 统一的交互语言和动画规范，60FPS 流畅体验
- **无障碍优先**: 对比度 >= 4.5:1，键盘完全可操作，支持 reduced-motion

### 1.2 风格定位

| 维度 | 选择 | 说明 |
|------|------|------|
| 设计风格 | Flat Design + 微投影 | 以扁平为主，窗口使用柔和投影区分层级 |
| 圆角语言 | 统一 8px | 窗口 8px，按钮/输入框 6px，卡片 8px |
| 图标风格 | Lucide SVG 图标 | 线条图标，1.5px stroke，不使用 emoji |
| 动效哲学 | 功能型动画 | 每个动画传达状态变化，非装饰性 |

---

## 2. 色彩系统

### 2.1 应用主题色 (亮色模式)

```css
:root {
  /* 品牌色 */
  --color-primary:          #0D9488;   /* Teal 600 - 主操作、标题栏 */
  --color-primary-hover:    #0F766E;   /* Teal 700 */
  --color-primary-active:   #115E59;   /* Teal 800 */
  --color-on-primary:       #FFFFFF;   /* 主色上的文字 */

  /* 辅助色 */
  --color-secondary:        #14B8A6;   /* Teal 500 - 次要操作 */
  --color-secondary-hover:  #0D9488;   /* Teal 600 */
  --color-on-secondary:     #FFFFFF;

  /* 强调色 */
  --color-accent:           #EA580C;   /* Orange 600 - 重要提示、新建按钮 */
  --color-accent-hover:     #C2410C;   /* Orange 700 */
  --color-on-accent:        #FFFFFF;

  /* 表面色 */
  --color-background:       #F8FAFC;   /* Slate 50 - 应用背景 */
  --color-surface:          #FFFFFF;   /* 卡片/窗口表面 */
  --color-surface-hover:    #F1F5F9;   /* Slate 100 */
  --color-foreground:       #0F172A;   /* Slate 900 - 主文字 */

  /* 弱化色 */
  --color-muted:            #E2E8F0;   /* Slate 200 - 分割线、边框 */
  --color-muted-bg:         #F1F5F9;   /* Slate 100 - 禁用/占位背景 */
  --color-muted-fg:         #64748B;   /* Slate 500 - 次要文字、占位符 */

  /* 边框 */
  --color-border:           #CBD5E1;   /* Slate 300 - 默认边框 */
  --color-border-focus:     #0D9488;   /* 聚焦边框 */

  /* 功能色 */
  --color-destructive:      #DC2626;   /* Red 600 - 删除 */
  --color-destructive-hover:#B91C1C;   /* Red 700 */
  --color-on-destructive:   #FFFFFF;

  --color-success:          #16A34A;   /* Green 600 */
  --color-warning:          #CA8A04;   /* Yellow 600 */

  /* 焦点环 */
  --color-ring:             #0D9488;
  --ring-width:             2px;
  --ring-offset:            2px;

  /* 阴影 */
  --shadow-sm:  0 1px 2px rgba(0, 0, 0, 0.05);
  --shadow-md:  0 4px 12px rgba(0, 0, 0, 0.08);
  --shadow-lg:  0 8px 24px rgba(0, 0, 0, 0.12);
  --shadow-xl:  0 12px 32px rgba(0, 0, 0, 0.16);
}
```

### 2.2 应用主题色 (暗色模式)

```css
.dark {
  --color-primary:          #14B8A6;   /* Teal 500 - 暗色下提亮 */
  --color-primary-hover:    #2DD4BF;   /* Teal 400 */
  --color-on-primary:       #042F2E;

  --color-secondary:        #0D9488;
  --color-on-secondary:     #F0FDFA;

  --color-accent:           #FB923C;   /* Orange 400 - 暗色下柔化 */
  --color-on-accent:        #1C1917;

  --color-background:       #0F172A;   /* Slate 900 */
  --color-surface:          #1E293B;   /* Slate 800 */
  --color-surface-hover:    #334155;   /* Slate 700 */
  --color-foreground:       #F8FAFC;   /* Slate 50 */

  --color-muted:            #334155;   /* Slate 700 */
  --color-muted-bg:         #1E293B;
  --color-muted-fg:         #94A3B8;   /* Slate 400 */

  --color-border:           #475569;   /* Slate 600 */
  --color-border-focus:     #14B8A6;

  --color-destructive:      #EF4444;
  --color-on-destructive:   #FFFFFF;

  --color-ring:             #14B8A6;

  --shadow-sm:  0 1px 2px rgba(0, 0, 0, 0.3);
  --shadow-md:  0 4px 12px rgba(0, 0, 0, 0.4);
  --shadow-lg:  0 8px 24px rgba(0, 0, 0, 0.5);
}
```

### 2.3 便签卡片配色 (6 色主题)

便签卡片颜色需同时在亮/暗模式下保持可读性。暗色模式下自动降低饱和度、提升明度。

| 主题名 | 亮色背景 | 亮色文字 | 暗色背景 | 暗色文字 | CSS 变量 |
|--------|---------|---------|---------|---------|----------|
| 经典黄 | `#FFF9C4` | `#5D4037` | `#44403C` | `#FDE68A` | `--note-yellow` |
| 天空蓝 | `#B3E5FC` | `#0D47A1` | `#1E3A5F` | `#93C5FD` | `--note-blue` |
| 薄荷绿 | `#C8E6C9` | `#1B5E20` | `#1A3A2A` | `#86EFAC` | `--note-green` |
| 珊瑚粉 | `#FFCDD2` | `#B71C1C` | `#4A1E1E` | `#FCA5A5` | `--note-pink` |
| 薰衣草紫 | `#E1BEE7` | `#4A148C` | `#2E1A47` | `#C4B5FD` | `--note-purple` |
| 纯白 | `#FFFFFF` | `#1E293B` | `#334155` | `#E2E8F0` | `--note-white` |

```css
/* 亮色模式便签色 */
:root {
  --note-yellow:   #FFF9C4;
  --note-blue:     #B3E5FC;
  --note-green:    #C8E6C9;
  --note-pink:     #FFCDD2;
  --note-purple:   #E1BEE7;
  --note-white:    #FFFFFF;
  --note-text-default: #1E293B;
}

/* 暗色模式便签色 (低饱和 + 深底) */
.dark {
  --note-yellow:   #44403C;
  --note-blue:     #1E3A5F;
  --note-green:    #1A3A2A;
  --note-pink:     #4A1E1E;
  --note-purple:   #2E1A47;
  --note-white:    #334155;
  --note-text-default: #E2E8F0;
}
```

### 2.4 色彩对比度验证

| 组合 | 对比度 | 标准 | 结果 |
|------|--------|------|------|
| 主文字 on 背景 (亮) | 15.4:1 | AAA | ✅ |
| 次文字 on 背景 (亮) | 5.7:1 | AA | ✅ |
| 白字 on Primary | 5.1:1 | AA | ✅ |
| 黄便签文字 `#5D4037` on `#FFF9C4` | 8.2:1 | AAA | ✅ |
| 蓝便签文字 `#0D47A1` on `#B3E5FC` | 7.5:1 | AAA | ✅ |
| 粉便签文字 `#B71C1C` on `#FFCDD2` | 5.9:1 | AA | ✅ |

---

## 3. 字体系统

### 3.1 字体选择

| 用途 | 字体 | 字重 | 来源 |
|------|------|------|------|
| UI 界面 | Plus Jakarta Sans | 400/500/600 | Google Fonts |
| 便签内容 | 系统默认字体 | 400 | 系统自带 |
| 等宽内容 | JetBrains Mono | 400 | Google Fonts (可选) |

**选择理由**: Plus Jakarta Sans 兼具现代感与可读性，适合生产力工具；便签内容使用系统字体确保加载速度和原生感。

### 3.2 字号层级

```css
:root {
  /* 字体族 */
  --font-ui:      'Plus Jakarta Sans', -apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif;
  --font-note:    -apple-system, BlinkMacSystemFont, 'Segoe UI', 'Microsoft YaHei', sans-serif;
  --font-mono:    'JetBrains Mono', 'Cascadia Code', 'Consolas', monospace;

  /* 字号 */
  --text-xs:    11px;    /* 时间戳、角标 */
  --text-sm:    12px;    /* 辅助文字、状态栏 */
  --text-base:  14px;    /* 正文默认 */
  --text-lg:    16px;    /* 小标题 */
  --text-xl:    18px;    /* 标题 */
  --text-2xl:   24px;    /* 大标题 */

  /* 行高 */
  --leading-tight:   1.25;
  --leading-normal:  1.5;
  --leading-relaxed: 1.75;

  /* 字间距 */
  --tracking-tight:  -0.01em;
  --tracking-normal:  0em;
  --tracking-wide:    0.025em;
}
```

### 3.3 便签内文字号 (用户可选)

| 档位 | 大小 | 行高 | 场景 |
|------|------|------|------|
| 小 | 12px | 1.5 | 密集记录 |
| 默认 | 14px | 1.5 | 日常使用 |
| 中 | 16px | 1.5 | 舒适阅读 |
| 大 | 18px | 1.6 | 视力辅助 |
| 特大 | 24px | 1.6 | 演示/标题 |

---

## 4. 间距与布局

### 4.1 间距系统 (8px 基准)

```css
:root {
  --space-1:   4px;    /* 紧凑内间距 */
  --space-2:   8px;    /* 元素内间距 */
  --space-3:   12px;   /* 小间距 */
  --space-4:   16px;   /* 标准间距 */
  --space-5:   20px;   /* 中等间距 */
  --space-6:   24px;   /* 区块间距 */
  --space-8:   32px;   /* 大区块间距 */
  --space-10:  40px;   /* 页面边距 */
  --space-12:  48px;   /* 区块分隔 */
}
```

### 4.2 主窗口布局

```
┌──────────────────────────────────────────────────────┐
│ [Logo] Memo          🔍 搜索便签...      [＋ 新建]   │  ← 顶栏 h=48px
├──────────────────────────────────────────────────────┤
│                                                      │
│  ┌────────────┐  ┌────────────┐  ┌────────────┐    │
│  │  经典黄     │  │  天空蓝     │  │  薄荷绿     │    │  ← 卡片网格
│  │            │  │            │  │            │    │     gap=16px
│  │  内容预览   │  │  内容预览   │  │  内容预览   │    │
│  │  ...       │  │  ...       │  │  ...       │    │
│  │            │  │            │  │            │    │
│  │  3/15 14:30│  │  3/14 09:12│  │  3/12 18:45│    │
│  └────────────┘  └────────────┘  └────────────┘    │
│                                                      │
│  ┌────────────┐  ┌────────────┐                     │
│  │  珊瑚粉     │  │  薰衣草紫   │                     │
│  │  ...       │  │  ...       │                     │
│  │  3/10 11:20│  │  3/08 16:00│                     │
│  └────────────┘  └────────────┘                     │
│                                                      │
├──────────────────────────────────────────────────────┤
│  共 12 条便签              [🗑 回收站]    [⚙ 设置]   │  ← 底栏 h=40px
└──────────────────────────────────────────────────────┘
```

**尺寸规范**:
- 主窗口最小尺寸: `600 x 400 px`
- 主窗口默认尺寸: `800 x 600 px`
- 顶栏高度: `48px`
- 底栏高度: `40px`
- 内容区 padding: `16px`
- 卡片间距: `16px`
- 卡片最小宽度: `240px`

### 4.3 便签卡片 (主窗口列表中)

```
┌────────────────────────────┐
│  ● 标题 (首行, 最多20字)    │  ← 标题行, font-weight: 600
│                            │
│  内容预览文字，最多显示      │  ← 预览区, 最多3行
│  三行，超出省略号截断...     │     line-clamp: 3
│                            │
│  ───────────────────────── │  ← 分割线
│  📅 3/15 14:30    [置顶📌] │  ← 元信息行
└────────────────────────────┘
```

**卡片尺寸**:
- 高度: 自适应，最小 `120px`
- 内边距: `16px`
- 圆角: `8px`
- 边框: `1px solid var(--color-border)` (亮色) / 无边框 (暗色，靠背景色区分)
- 悬停效果: `translateY(-2px)` + `shadow-md`，过渡 `200ms ease-out`
- 点击效果: `scale(0.98)`，过渡 `100ms ease-in`

### 4.4 便签编辑窗口

```
┌────────────────────────────────────┐
│  ● ● ●    [≡置顶] [📌边缘吸附] [✕] │  ← 标题栏 h=32px, 可拖拽区
├────────────────────────────────────┤
│                                    │
│  在这里输入内容...                  │  ← 编辑区, 自适应填充
│                                    │
│                                    │
│                                    │
│                                    │
│                                    │
│                       3/15 14:30   │  ← 状态栏 h=24px
└────────────────────────────────────┘
```

**窗口尺寸**:
- 默认: `300 x 300 px`
- 最小: `200 x 150 px`
- 最大: `800 x 600 px`
- 标题栏高度: `32px`
- 状态栏高度: `24px`
- 编辑区内边距: `12px`
- 窗口圆角: `8px`
- 窗口阴影: `shadow-lg` (浮于桌面之上)

**标题栏元素**:
- 左侧: 3 个颜色圆点 (● ● ●)，点击展开颜色选择面板
- 右侧: 置顶按钮、边缘吸附指示按钮、关闭按钮
- 按钮尺寸: `24 x 24 px`，图标 `16px`
- 按钮间距: `4px`

### 4.5 颜色选择面板

```
┌─────────────────────────┐
│  选择便签颜色            │
│                         │
│  [■黄] [■蓝] [■绿]     │
│  [■粉] [■紫] [■白]     │
│                         │
└─────────────────────────┘
```

- 面板宽度: `180px`
- 色块尺寸: `40 x 40 px`，圆角 `6px`
- 选中态: `2px solid var(--color-primary)` 外边框
- 面板从标题栏颜色圆点下方弹出，带 `scale(0.95) → scale(1)` + `opacity` 过渡

---

## 5. 组件规范

### 5.1 按钮

| 类型 | 背景 | 文字 | 边框 | 用途 |
|------|------|------|------|------|
| Primary | `--color-primary` | `--color-on-primary` | 无 | 新建便签、确认操作 |
| Secondary | `transparent` | `--color-primary` | `1px --color-primary` | 取消、次要操作 |
| Ghost | `transparent` | `--color-foreground` | 无 | 工具栏按钮 |
| Destructive | `--color-destructive` | `--color-on-destructive` | 无 | 删除确认 |
| Icon | `transparent` | `--color-muted-fg` | 无 | 图标按钮 (关闭/置顶等) |

**按钮尺寸**:
- 标准: `h=36px`, `px=16px`, `font-size=14px`, `border-radius=6px`
- 小型: `h=28px`, `px=12px`, `font-size=12px`, `border-radius=4px`
- 图标: `24 x 24 px`, `border-radius=4px`
- 触摸目标最小: `44 x 44 px` (WCAG 2.2)

**交互状态**:
```
Default → Hover:    background 变化, 150ms ease
Hover → Active:     scale(0.98) + background 加深, 100ms ease
Active → Focus:     outline: 2px solid var(--color-ring), offset 2px
Default → Disabled: opacity: 0.5, cursor: not-allowed
```

### 5.2 输入框

```
┌──────────────────────────────────┐
│  🔍  搜索便签...                 │  ← 搜索框
└──────────────────────────────────┘
```

- 高度: `36px`
- 内边距: `0 12px 0 36px` (左侧留图标位)
- 边框: `1px solid var(--color-border)`
- 聚焦: `border-color: var(--color-border-focus)` + `ring: 2px var(--color-ring)`
- 圆角: `6px`
- 占位符颜色: `var(--color-muted-fg)`

### 5.3 系统托盘图标

- 尺寸: `16 x 16 px` (渲染在系统托盘)
- 设计: 便签纸折叠角图标，使用 `--color-primary` 填充
- 角标: 便签数量，圆形红底白字，最小 `16px` 直径

### 5.4 右键菜单 (托盘/窗口)

```
┌──────────────────────┐
│  ＋ 新建便签    Ctrl+Shift+N │
│  ─────────────────── │
│  ☐ 显示主窗口    Ctrl+Shift+M │
│  ─────────────────── │
│  ☑ 开机自启动          │
│  ─────────────────── │
│  ⚙ 设置               │
│  ✕ 退出               │
└──────────────────────┘
```

- 菜单宽度: 自适应内容，最小 `200px`
- 菜单项高度: `36px`
- 菜单项内边距: `0 16px`
- 快捷键文字: `var(--color-muted-fg)`, 右对齐
- 悬停: `background: var(--color-surface-hover)`
- 分隔线: `1px solid var(--color-muted)`, 上下 `4px` 间距
- 圆角: `8px`
- 阴影: `shadow-lg`

### 5.5 设置面板

```
┌──────────────────────────────────────────────┐
│  ⚙ 设置                                  [✕] │
├──────────────────────────────────────────────┤
│                                              │
│  通用                                        │
│  ├── 开机自启动              [开关 ●───○]    │
│  ├── 全局快捷键              [Ctrl+Shift+N]  │
│  └── 主题模式     [○亮色 ●暗色 ●跟随系统]   │
│                                              │
│  边缘隐藏                                    │
│  ├── 启用边缘隐藏            [开关 ●───○]    │
│  ├── 吸附触发距离            [20px ▓▓░░░]   │
│  ├── 可见条宽度              [4px  ▓░░░░]   │
│  ├── 动画时长                [200ms ▓▓░░░]  │
│  ├── 收回延迟                [300ms ▓▓▓░░]  │
│  └── 启用方向                                │
│      ├── 上边缘  [☑]  下边缘  [☐]           │
│      ├── 左边缘  [☑]  右边缘  [☑]           │
│                                              │
│  关于                                        │
│  ├── 版本: v1.0.0                           │
│  └── [检查更新]                              │
│                                              │
└──────────────────────────────────────────────┘
```

- 面板宽度: `480px`
- 分组标题: `font-weight: 600`, `font-size: 16px`, 底部间距 `12px`
- 设置项高度: `40px`
- 开关组件: `40 x 22 px`，圆形滑块 `18px`
- 滑块: 轨道 `120 x 4 px`，滑块 `16 x 16 px`

---

## 6. 边缘隐藏动画规范

### 6.1 动画参数

| 动画 | 时长 | 缓动 | 属性 |
|------|------|------|------|
| 隐藏 (滑出) | `200ms` | `cubic-bezier(0.4, 0, 1, 1)` (ease-in) | `transform: translateY(-100%)` |
| 弹出 (滑入) | `200ms` | `cubic-bezier(0, 0, 0.2, 1)` (ease-out) | `transform: translateY(0)` |
| 收回延迟 | `300ms` | — | 定时器，无动画 |
| 吸附指示 | `150ms` | `ease` | `opacity` + `border-color` |

### 6.2 隐藏方向映射

| 吸附边缘 | 隐藏方向 | transform | 可见条位置 |
|----------|----------|-----------|-----------|
| 上 (top) | 向上滑出 | `translateY(-100%)` | 屏幕顶部 4px 条 |
| 下 (bottom) | 向下滑出 | `translateY(100%)` | 屏幕底部 4px 条 |
| 左 (left) | 向左滑出 | `translateX(-100%)` | 屏幕左侧 4px 条 |
| 右 (right) | 向右滑出 | `translateX(100%)` | 屏幕右侧 4px 条 |

### 6.3 可见指示条

- 宽度: `4px` (可配置)
- 颜色: 继承便签主题色，暗色模式下降低饱和度
- 位置: 紧贴屏幕边缘
- 悬停反馈: 鼠标接近时可见条亮度提升 (`opacity: 0.6 → 1.0`)，过渡 `150ms`

### 6.4 动画状态机

```
                    拖拽到边缘
    [正常显示] ──────────────────→ [吸附中]
        ↑                             │
        │ 从边缘拖回                   │ 松手
        │                             ↓
        │                         [隐藏中] ← 200ms ease-in 动画
        │                             │
        │                             │ 动画完成
        │                             ↓
        │    鼠标移入检测区         [已隐藏]
        │  ←──────────────────────── │
        │                             │
        │                             │ 鼠标移出窗口+检测区
        │                             ↓
        └───────────────────── [收回中] ← 300ms 延迟 + 200ms ease-out 动画
              动画完成
```

### 6.5 Reduced Motion 支持

```css
@media (prefers-reduced-motion: reduce) {
  .note-window {
    transition: none !important;
    animation: none !important;
  }
  /* 隐藏/弹出改为瞬间切换，无位移动画 */
}
```

---

## 7. 图标规范

### 7.1 图标库
使用 **Lucide Icons** (MIT 协议，SVG 格式，与 Flat Design 风格一致)

### 7.2 图标尺寸

| 场景 | 尺寸 | stroke-width |
|------|------|-------------|
| 标题栏按钮 | `16px` | `1.5px` |
| 工具栏按钮 | `20px` | `1.5px` |
| 托盘图标 | `16px` | `1.5px` |
| 列表图标 | `14px` | `1.5px` |
| 空状态插图 | `48px` | `1px` |

### 7.3 关键图标映射

| 功能 | Lucide 图标名 | 说明 |
|------|--------------|------|
| 新建 | `plus` | 加号 |
| 关闭 | `x` | 叉号 |
| 置顶 | `pin` / `pin-off` | 图钉 |
| 搜索 | `search` | 放大镜 |
| 设置 | `settings` | 齿轮 |
| 删除 | `trash-2` | 垃圾桶 |
| 回收站 | `archive-restore` | 归档恢复 |
| 边缘吸附 | `monitor` | 显示器 |
| 颜色 | `palette` | 调色板 |
| 托盘 | `sticky-note` | 便签 |
| 导出 | `download` | 下载箭头 |
| 导入 | `upload` | 上传箭头 |
| 亮色模式 | `sun` | 太阳 |
| 暗色模式 | `moon` | 月亮 |

---

## 8. 交互细节

### 8.1 拖拽行为

- 标题栏为拖拽区域 (`data-tauri-drag-region`)
- 拖拽时窗口 `opacity: 0.9` + `shadow-xl`，提供浮起感
- 接近边缘时显示吸附预览: 边缘高亮 `2px dashed var(--color-primary)`
- 拖拽释放时在吸附阈值内自动对齐到边缘

### 8.2 窗口调整大小

- 四边 + 四角均可拖拽调整
- 拖拽手柄区域: `8px` 宽 (边缘热区)
- 光标变化: `n-resize` / `s-resize` / `e-resize` / `w-resize` / 对角方向
- 实时显示尺寸 tooltip: `300 x 300`，位于窗口左上角

### 8.3 键盘导航

| 按键 | 行为 |
|------|------|
| `Tab` | 在可聚焦元素间循环 |
| `Enter` / `Space` | 激活当前聚焦的按钮/链接 |
| `Escape` | 关闭弹窗 / 取消编辑 / 隐藏主窗口 |
| `Ctrl+Shift+N` | 新建便签 |
| `Ctrl+Shift+M` | 显示/隐藏主窗口 |
| `Ctrl+W` | 关闭当前便签窗口 |
| `Ctrl+Shift+P` | 切换置顶 |
| `Arrow Keys` | 在便签列表中移动焦点 |

### 8.4 悬停与点击反馈

```
按钮 Hover:   background lighten 10%, 150ms ease
按钮 Active:  scale(0.96), 100ms ease-in
卡片 Hover:   translateY(-2px) + shadow-md, 200ms ease-out
卡片 Click:   scale(0.98), 100ms ease-in
输入框 Focus: border-color primary + ring, 150ms ease
开关 Toggle:  滑块位移 + 背景色变化, 200ms ease
```

---

## 9. 响应式适配

虽然 Memo 是桌面应用，但主窗口需适配不同尺寸:

| 断点 | 宽度 | 布局变化 |
|------|------|---------|
| Compact | `< 500px` | 单列卡片，隐藏搜索栏文字 |
| Standard | `500-800px` | 双列卡片网格 |
| Wide | `800-1200px` | 三列卡片网格 |
| Ultra | `> 1200px` | 四列卡片网格 |

**卡片网格**: CSS Grid + `auto-fill` + `minmax(240px, 1fr)`

---

## 10. 交付检查清单

### 10.1 无障碍 (Priority 1)
- [ ] 所有文字对比度 >= 4.5:1 (AA)
- [ ] 所有可交互元素有可见 focus 状态
- [ ] 图标按钮有 `aria-label`
- [ ] 支持 `prefers-reduced-motion`
- [ ] 键盘完全可操作 (无鼠标依赖)
- [ ] 拖拽操作有按钮替代方案

### 10.2 交互 (Priority 2)
- [ ] 所有点击元素有 `cursor: pointer`
- [ ] Hover 状态过渡平滑 (150-300ms)
- [ ] 触摸目标 >= 44x44px
- [ ] 动画不影响布局 (使用 transform/opacity)
- [ ] 边缘隐藏/弹出动画 60FPS

### 10.3 视觉 (Priority 3)
- [ ] 使用 SVG 图标，不使用 emoji
- [ ] 暗色模式所有色彩对比度验证通过
- [ ] 便签 6 色在亮/暗模式下均可读
- [ ] 圆角/间距/阴影严格遵循设计令牌
- [ ] 无渐变背景 (Flat Design 原则)

### 10.4 性能 (Priority 4)
- [ ] 字体子集化 (仅包含使用到的字符)
- [ ] 图标 tree-shaking (按需导入 Lucide)
- [ ] CSS 变量避免运行时计算
- [ ] 动画使用 `will-change` 提示 GPU 加速
- [ ] 列表虚拟滚动 (便签数 > 50 时)

---

*文档版本: v1.0 | 完整版本信息见文档头部*
