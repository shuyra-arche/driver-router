use super::facts::{Facts, facts};
use env::PieToken;

pub fn alive(token: PieToken) -> bool {
    facts()
        .lock()
        .unwrap()
        .get(&token.get())
        .is_some_and(|fact| fact.alive)
}

pub fn reserve(token: PieToken) -> Result<(env::TaskId, env::TaskId, env::Mark), ()> {
    facts()
        .lock()
        .unwrap()
        .get(&token.get())
        .copied()
        .map(
            |Facts {
                 vestor,
                 owner,
                 mark,
                 ..
             }| (vestor, owner, mark),
        )
        .ok_or(())
}
