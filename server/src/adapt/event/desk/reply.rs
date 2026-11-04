use env::TaskId;
use router_api::frame as lcall;

/// Reply storage stays at a stable address for the duration of the push.
pub struct Reply {
    /// 哪一位客人（回信孔的主人）
    pub who: TaskId,
    /// Response bytes remain at a stable address for the duration of the push.
    pub bytes: [u8; lcall::OccupyReply::LEN],
}
