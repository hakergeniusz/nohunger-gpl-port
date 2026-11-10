// Once a tick, on every Game in the process. Only the window's Game writes
// the gate: its mods list is the live one (Options > Mods changes it at
// once), while the integrated server's Game holds the world-open copy of it
// (settings are cloned when the world opens) — a write from there would
// stamp a stale switch over the live one. The server thread's food ticks
// read the bit the window keeps, which is the point.
// Copyright (c) 2026 hakergeniusz. Portions Copyright (c) Meteor Development
// (github.com/MeteorDevelopment/meteor-client), licensed under GPL-3.0.
fn nohunger_tick(game: &mut crate::game::Game<'_>) {
    use crate::game::nohunger as nohunger;
    // The window is the Game that joined a server — the integrated one
    // through memory in singleplayer, or a real one online (where nothing
    // drains in this process, and the gate it writes is simply never read).
    if game.remote.is_none() {
        return;
    }
    let switch_on = game.native_on(nohunger::ID);
    horizon_client::hunger::nohunger_gate::set_active(nohunger::gates(switch_on));
}
