#![no_std]

//! Router 实现侧（U 态，一枚线程）：外部中断的收与结。
//! 那一圈的文件在 `adapt/{sweep,resident}.rs ＋ adapt/event/{bell,desk,exhaust}.rs`，那一圈的**壳**在
//! `adapt/resident.rs`；树那侧的事实与线集合在 `core/sources.rs`（纯），寄存器面在 `dev/plic.rs`。
//! 判据、裁法与那一张读数表在 `driver/router/mod.rs`（Source c975），本 crate 只做 provider 搬移。
//! 唯一的公开入口是 [`serve`]；`adapt` / `core` / `dev` 都是实现私有。

extern crate alloc;

pub(crate) use system_support::debug;

pub fn serve() -> Result<(), driver_shared::fail::Fail> {
    // 起手：领配给 → 开两图 → 读树 → 建账 → 铸入口 → 上板 ＋ 上树 → 挂组。
    let mut up = adapt::boot::up()?;
    // 常驻：等三源 → 逐客 / 排空 / 登记 / 铃。
    adapt::resident::run(&mut up)
}

mod adapt;
mod core;
mod dev;
