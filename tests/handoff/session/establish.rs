use super::Endpoint;
use crate::ACCEPTED;
use std::sync::atomic::Ordering;

use env::PieToken;

pub fn accept(token: PieToken) -> Result<super::Endpoint, ()> {
    ACCEPTED.store(token.get(), Ordering::SeqCst);
    Ok(Endpoint {
        tx: token,
        seed: PieToken::mint(token.get() + 1000),
    })
}
