use ipc::session::Held;

/// 一格：没主，或者有主（**那条路的持有者** ＋ 忙不忙）
pub(super) enum Cell {
    Idle,
    Owned { lane: Held, busy: bool },
}
