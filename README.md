# NoHunger for Horizon

Doesn't let your hunger bar go down. A [Horizon](https://github.com/horizon-rust/horizon)
native mod ported from [Meteor Client](https://github.com/MeteorDevelopment/meteor-client)'s
hunger-suppression module, licensed [GPL-3.0](LICENSE) as a derivative of it.

While the mod is on (Options > Mods > NoHunger), the game's food tick is held:
the hunger bar, its saturation and the starvation timer all freeze where they
were. Movement, sprinting, jumping, mining and attacks still feel exactly like
vanilla — only the drain is gone. Turn it off and the hunger system resumes
exactly where it left off.

## What it does, precisely

Horizon runs the player's hunger in the same process as the game: the food
tick (`FoodData::tick`, reached through `PlayerMotion::food_tick`) is what
spends saturation and food, heals from saturation, and starves an empty bar.
The mod cancels that tick at its head while enabled — nothing else about
hunger is touched:

- **Exhaustion still accrues.** Movement, jumps and attacks add it exactly as
  vanilla does (capped, as vanilla caps it). The held tick simply never
  processes it.
- **Regeneration stops while held.** In vanilla, regen heals *because of*
  food and costs food (each heal adds 6.0 exhaustion) — a held tick cannot
  heal without un-holding the bar. Peaceful difficulty's free regen and food
  refill run on their own path and are untouched.
- **Disabling processes the accrual at once.** The first ticks after you turn
  the mod off spend everything it queued: up to ten points of food bar. This
  is the original module's behavior — Meteor Client's oldest version warned
  about it in the module description itself ("WARNING: It will u when u
  disable it") — and this port keeps it rather than inventing a cleaner one.
- **Single-player and LAN hosts.** The drain runs on the world's server
  thread, so the gate applies there: every player of that world (the host and
  any LAN guests) keeps their bar while it is on. On an online server the mod
  does nothing — the drain lives in the server's process, not yours.

## Settings

None. The mod is the plain toggle it was in Meteor Client: Meteor's hunger
module ("AntiHunger", the Player category's hunger suppression) worked by
spoofing sprint and on-ground movement packets, and its two booleans configure
those spoofs. Horizon's hunger drains inside this process, so there are no
packets to spoof — the port replaces the mechanism with a hold on the food
tick itself and has nothing left for the two settings to configure.

## Install

The `.hnmod` is source-only: Horizon's launcher compiles it against the
pinned Horizon revision on install.

```sh
python3 tools/package.py   # writes target/nohunger-0.1.0.hnmod
```

## Development

The mod's logic lives in `mixins/nohunger/runtime.rs` — pure standard
library, no game types — which doubles as this repository's tested library
and as the module Horizon injects into the game. The other files in
`mixins/nohunger/` are small helpers that bridge the game to it; they are
injected into the game's sources by [horizon-mixins](https://github.com/horizon-rust/horizon-mixins)
and compile only there:

- `gate.rs` — the process-wide "food tick held" bit, injected beside the food
  tick, with the staged tests that hold and release a real player's food
  tick through it.
- `tick.rs` — once a tick on every Game in the process; the window's Game
  (whose mods list is live) writes the gate, the integrated server's (whose
  copy is from world-open time) never does.
- `hold.rs` — the cancellable head of `PlayerMotion::food_tick` itself.
- `list.rs` — the row in Options > Mods.

```sh
python3 tools/test.py                                # fmt, clippy, the runtime's tests
HORIZON_MIXINS=…/horizon-mixins python3 tools/horizon.py stage --horizon …/horizon
HORIZON_MIXINS=…/horizon-mixins python3 tools/horizon.py check --horizon …/horizon
HORIZON_MIXINS=…/horizon-mixins python3 tools/horizon.py test  --horizon …/horizon
```

`compatibility.json` pins the Horizon revision and the hashes of the three
patched sources (`game.rs`, `game/native_hooks.rs`, `hunger.rs`); a Horizon
change there needs the pins re-checked.

## Ported from Meteor Client

Meteor Client has no module named "NoHunger" — its Player category's hunger
suppression module is **AntiHunger**, present under that name from the first
release to today. This port is of that module's spirit and behavior, under
the name and promise the port was asked for:

- Meteor's AntiHunger ("Reduces (does NOT remove) hunger consumption")
  spoofs packets to lower the exhaustion the server credits you with. The
  port targets Horizon's actual drain point and holds it whole, which
  eliminates the drain instead of reducing it — the behavior this mod's name
  promises.
- The v0.1.5 disable behavior (the accrued-exhaustion dump) is kept, and the
  old description's WARNING is documented here in its place.
- The module's on/off spirit (a plain toggle, no settings) is kept: Meteor's
  two booleans configure packet spoofs, which have no Horizon equivalent.

## License

Copyright (c) 2026 hakergeniusz

Portions Copyright (c) Meteor Development (github.com/MeteorDevelopment/meteor-client),
licensed under GPL-3.0.

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU General Public License as published by the Free Software
Foundation, version 3 of the License. See [LICENSE](LICENSE) and
[CREDITS.md](CREDITS.md).
