// Linux 上 WebView 是 WebKitGTK，与 Windows/macOS 的 WebView2/WebKit 有两处能力差异需要前端降级：
// 透明窗口上的 backdrop-filter 合成不正确；程序化 setPosition 无法把窗口移到屏幕外（被合成器夹回屏内）。
// wry 的 UA 在 Linux 上含 "Linux"，Windows 含 "Windows NT"，macOS 含 "Mac OS X"。
export const IS_LINUX = /Linux/i.test(navigator.userAgent)
