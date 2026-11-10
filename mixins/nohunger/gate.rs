// The nohunger gate: whether this process's food tick is held.
// Copyright (c) 2026 hakergeniusz. Portions Copyright (c) Meteor Development
// (github.com/MeteorDevelopment/meteor-client), licensed under GPL-3.0.
//
// One bit, process-wide — deliberately not per-thread, and deliberately not
// the runtime's own state. The freecam's freeze was per-thread because a
// freeze is a per-sim state: the window's camera flew, the integrated
// server's never did, and a shared bit would have made the freeze a coin
// flip on thread scheduling. The hunger switch is the opposite: it is the
// user's one toggle, and the drain it stops runs on the integrated server's
// thread — whose own Game holds a world-open-time copy of the mods list
// (settings are cloned when the world opens) and would never see the toggle
// go live. So the window thread's tick — whose Game reads the switch live —
// writes this bit every tick, and every food tick on every thread reads it:
// the drain the player experiences stops, whoever the server thread sims
// (its own guests are the players of the world, the window's player among
// them).
//
// An AtomicBool, because the writer (the window's tick) and the readers
// (the food ticks, mostly on the Server thread) are different threads. A
// relaxed ordering is enough: the bit is a one-tick-accurate mirror of a
// user toggle, and no reader derives a second state from it.
use std::sync::atomic::{AtomicBool, Ordering};

static ACTIVE: AtomicBool = AtomicBool::new(false);

/// Whether the food tick is held (its drain, regeneration and starvation
/// skipped) for this process.
pub fn active() -> bool {
    ACTIVE.load(Ordering::Relaxed)
}

/// The window's tick says whether the mod holds the food tick. The
/// integrated server's thread never writes: its mods list is the
/// world-open copy, and a stale write would clobber the live one.
pub fn set_active(on: bool) {
    ACTIVE.store(on, Ordering::Relaxed);
}

#[cfg(test)]
mod tests {
    use super::*;
    // The staged crate's sim types: the injected module sees hunger's own
    // items through its paths, not through the parent's imports.
    use crate::assets::Assets;
    use crate::motion::{MotionResources, PlayerMotion};

    /// The gate starts closed, and a write from this thread reaches a reader
    /// on another thread: the window writes, the Server thread reads.
    #[test]
    fn the_gate_starts_closed_and_reaches_across_threads() {
        assert!(!active(), "a fresh process's food tick runs");
        set_active(true);
        std::thread::scope(|s| {
            let held = s.spawn(|| active()).join().unwrap();
            assert!(held, "another thread's food tick reads the held gate");
        });
        set_active(false);
        assert!(!active());
    }

    /// The prepared resources carry the health and movement state the food
    /// tick reads; the lan bot tests skip without them, and so does this.
    fn resources() -> Option<MotionResources> {
        let root = std::env::var_os("HORIZON_TEST_RESOURCES")
            .map(std::path::PathBuf::from)
            .unwrap_or_else(|| {
                std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                    .join("../../.local/horizon-client")
                    .join(horizon_core::VANILLA_VERSION)
            });
        if !root.join("movement-properties.nbt").exists() {
            eprintln!("skipped the gate's food-tick test: prepared resources unavailable");
            return None;
        }
        let assets = Assets::load(&root).unwrap();
        Some(MotionResources::load(&root, &assets).unwrap())
    }

    /// The gate's whole meaning, through the game's own food tick. This test
    /// holds the gate for its body and clears it at the end; the staged run
    /// selects it by name (`hunger::nohunger_gate`), so the game's own
    /// food-tick tests never race it.
    #[test]
    fn the_gate_holds_the_drain_and_releases_it_where_it_was() {
        let Some(resources) = resources() else {
            return;
        };

        // The game's own arithmetic, gate off: an exhausted, unsaturated
        // player's food bar drains a point a tick (vanilla, Normal rules).
        let mut draining = PlayerMotion::new(&resources, horizon_core::geometry::Vec3::ZERO)
            .unwrap();
        draining.health.food.saturation = 0.0;
        draining.health.food.exhaustion = 5.0;
        draining
            .food_tick(&resources.health, Default::default())
            .unwrap();
        assert_eq!(draining.health.food.food, 19, "vanilla drains a point");
        assert_eq!(
            draining.health.food.exhaustion, 1.0,
            "four exhaustion spent for the point"
        );

        // The same player, gate on: the tick is held whole — no drain, no
        // regeneration, no starvation — while the drain's inputs keep
        // accruing: movement, jumps and attacks add exhaustion without the
        // tick (the port holds only the tick, as the original held only
        // some of the sources).
        let mut held = PlayerMotion::new(&resources, horizon_core::geometry::Vec3::ZERO).unwrap();
        held.health.food.exhaustion = 5.0;
        set_active(true);
        for _ in 0..200 {
            held.food_tick(&resources.health, Default::default())
                .unwrap();
            held.cause_food_exhaustion(0.2);
        }
        assert_eq!(held.health.food.food, 20, "the held tick drains nothing");
        assert_eq!(held.health.food.saturation, 5.0);
        assert_eq!(
            held.health.food.exhaustion, 40.0,
            "the accrual caps where vanilla caps it"
        );

        // The original's WARNING, still true: the first released ticks
        // process the accrual at once — five points off the saturation and
        // four off the bar, a point a tick. A point's worth stays queued
        // (the tick spends exhaustion in fours, past a strict `> 4.0`), to
        // go with the next accrual.
        set_active(false);
        for _ in 0..10 {
            held.food_tick(&resources.health, Default::default())
                .unwrap();
        }
        assert_eq!(held.health.food.food, 16, "the accrued exhaustion dumps");
        assert_eq!(
            held.health.food.exhaustion, 4.0,
            "the last point's worth stays queued for the next accrual"
        );
        assert_eq!(held.health.food.saturation, 0.0);

        // A well-fed player under the gate just stays well-fed, and
        // releasing them — nothing accrued — changes nothing.
        let mut fed = PlayerMotion::new(&resources, horizon_core::geometry::Vec3::ZERO).unwrap();
        set_active(true);
        for _ in 0..20 {
            fed.food_tick(&resources.health, Default::default()).unwrap();
        }
        assert_eq!(fed.health.food.food, 20);
        assert_eq!(fed.health.food.saturation, 5.0);
        set_active(false);
        for _ in 0..20 {
            fed.food_tick(&resources.health, Default::default()).unwrap();
        }
        assert_eq!(fed.health.food.food, 20, "nothing accrued, nothing dumps");
        assert_eq!(fed.health.food.saturation, 5.0);
    }
}
