use env::PieToken;

pub struct Endpoint {
    pub(crate) tx: PieToken,
    pub(crate) seed: PieToken,
}
impl Endpoint {
    pub fn tx(&self) -> Option<PieToken> {
        Some(self.tx)
    }
    pub fn seed(&self) -> PieToken {
        self.seed
    }
}
