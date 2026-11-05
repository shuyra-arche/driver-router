#![no_std]

//! 线路由者（router）的兼容装配声明与构建合同。
//! **U 态**：它只读 PLIC 的寄存器、claim/complete、铸孔、挂组，全都不需要 S 态；
//! 当前运行策略由仓库根目录 `unit.toml` 持有；此处保留旧调用与迁移对照。

use unit::{Ending, ImageBuild, UnitFile};

pub static UNIT: UnitFile = UnitFile {
    name: "router",
    loader: unit::LoaderConfig::user("router"),
    operator: unit::OperatorConfig {
        namespace: &[unit::operator::Namespace {
            key: system_api::namespace::SERVICE,
            placement: unit::operator::Placement::Shared,
            path: "router",
            scope: unit::operator::Scope::Subtree,
            ops: system_api::operator::Ops::PART
                .union(system_api::operator::Ops::UNPART)
                .union(system_api::operator::Ops::BIND)
                .union(system_api::operator::Ops::UNBIND)
                .union(system_api::operator::Ops::FIND),
            permit: Some(system_api::operator::Permit::Public),
            source_mark: None,
            bindings: &[unit::operator::Binding {
                name: router_api::ENTRY_NAME,
                mark: router_api::ENTRY_MARK,
                view: env::Access::STORE,
                permit: system_api::operator::Permit::Public,
            }],
        }],
    },
    identity: unit::Identity {
        aliases: &[],
        grants: &[],
    },
    control: unit::ControlConfig {
        supervision: None,
        after: Some(&[
            unit::Dependency::Bootstrap(unit::Bootstrap::Operator),
            unit::Dependency::Unit("hub"),
        ]),
        ready: unit::Ready::Signal(unit::READY_MARK),
        ending: Some(Ending::Resident),

        construction: &[],
        retirement: &[],
    },
    supply: unit::SupplyConfig { bindings: &[] },
};

pub const BUILD: ImageBuild = ImageBuild {
    image: "router",
    manifest: "server/Cargo.toml",
    package: "router-server",
    bin: "router",
    features: &[],
    target: "riscv64gc-unknown-none-elf",
    profile: "release",
    default_features: true,
};
