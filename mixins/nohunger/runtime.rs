// Horizon NoHunger: doesn't let your hunger bar go down, ported from Meteor
// Client (https://github.com/MeteorDevelopment/meteor-client, GPL-3.0). The
// hunger-suppression module of Meteor's Player category (AntiHunger, the
// packet-spoofing form) becomes, on Horizon, a hold on the sim's food tick
// itself: Horizon runs the drain in the process's own sim, so there are no
// move packets to spoof — the food tick is simply held while the mod is on.
//
// Copyright (c) 2026 hakergeniusz. Portions Copyright (c) Meteor Development
// (github.com/MeteorDevelopment/meteor-client), licensed under GPL-3.0.
//
// While it holds, the drain, the regeneration that costs hunger and the
// starvation all wait; the exhaustion a tick's movement, jumps and attacks
// add keeps accruing (capped, as vanilla caps it), and the first food tick
// after the switch turns off processes it at once — Meteor's oldest warning
// ("WARNING: It will u when u disable it") is still true here, and this port
// keeps it rather than inventing a cleaner disable.
//
// This file is both the mod's Cargo library (its tests) and the module the
// bundle adds to the game. The mod has no settings of its own: it is the
// plain toggle its original was, and Meteor's two booleans (the sprint and
// on-ground packet spoofs) have nothing to port to — Horizon's sim has no
// packets between the window and the world to spoof. The shared state is a
// process-wide atomic in the gate module (`gate.rs`), not a thread_local:
// unlike the freecam's freeze (a per-sim state), the hunger switch is the
// user's one toggle, and the drain it stops runs on the integrated server's
// thread, whose own Game never sees the toggle go live.
//
// Settings are kept as Horizon's mods keep them (`horizon-mods/
// .horizon-mod-data/<id>/<key>`); the mod reads and writes none today.

/// The mod's id: its row in Options > Mods and its settings folder.
pub const ID: &str = "nohunger";
pub const VERSION: &str = "0.1.0";
/// The row's description: the original module's promise.
pub const DESCRIPTION: &str = "Doesn't let your hunger bar go down.";

/// Whether the food tick is held this tick: the mod's switch, as the window's
/// Game reports it from Options > Mods. Everything else about hunger — the
/// drain's arithmetic, saturation, regeneration, starvation — stays the
/// game's; the mod only decides whether the tick runs at all.
pub fn gates(switch_on: bool) -> bool {
    switch_on
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The mod is its switch: on holds the food tick, off releases it.
    #[test]
    fn the_gate_is_the_switch() {
        assert!(gates(true), "an enabled mod holds the food tick");
        assert!(!gates(false), "a disabled mod leaves the drain alone");
    }

    /// The mod's identity, as the mods list and the settings folder name it.
    #[test]
    fn the_mods_identity_is_a_plain_toggle() {
        assert_eq!(ID, "nohunger");
        assert_eq!(VERSION, "0.1.0");
        assert!(
            ID.chars()
                .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_'),
            "the registry takes ids of lowercase letters, digits and _ only"
        );
        assert_eq!(
            DESCRIPTION, "Doesn't let your hunger bar go down.",
            "the description is the original module's promise"
        );
    }
}
