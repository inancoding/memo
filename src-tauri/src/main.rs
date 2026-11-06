// release 构建使用 windows 子系统：不弹控制台窗口，启动更快
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    // 修复部分 Linux（如 deepin）WebKitGTK 硬件加速导致的窗口白屏：禁用 DMABUF 渲染器
    #[cfg(target_os = "linux")]
    {
        if std::env::var_os("WEBKIT_DISABLE_DMABUF_RENDERER").is_none() {
            std::env::set_var("WEBKIT_DISABLE_DMABUF_RENDERER", "1");
        }
    }

    memo_lib::run()
}
