// The food tick, held while the mod asks: the drain, the regeneration that
// costs hunger and the starvation all wait. Exhaustion keeps accruing (the
// sim's movement, jumps and attacks add it without the tick), and the first
// tick after the switch turns off processes it at once — the original's
// WARNING, kept rather than cleaned up.
// Copyright (c) 2026 hakergeniusz. Portions Copyright (c) Meteor Development
// (github.com/MeteorDevelopment/meteor-client), licensed under GPL-3.0.
fn nohunger_hold() -> core::ops::ControlFlow<Result<()>> {
    if crate::hunger::nohunger_gate::active() {
        core::ops::ControlFlow::Break(Ok(()))
    } else {
        core::ops::ControlFlow::Continue(())
    }
}
