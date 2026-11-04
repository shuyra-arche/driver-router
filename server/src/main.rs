#![no_std]
#![no_main]

//! 外部中断的收与结（U 态，一枚线程）。
//! 唯一入口是 `router_server::serve`；本 bin 只做公共 `mold`/`execution` 胶水。

#[mold::entry]
fn main() -> Result<(), driver_shared::fail::Fail> {
    router_server::serve()
}
