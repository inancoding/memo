// release 构建使用 windows 子系统：不弹控制台窗口，启动更快
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    memo_lib::run()
}
