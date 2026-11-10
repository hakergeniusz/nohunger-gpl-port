// Nohunger's row in Options > Mods: a plain toggle, so no keys.
// Copyright (c) 2026 hakergeniusz. Portions Copyright (c) Meteor Development
// (github.com/MeteorDevelopment/meteor-client), licensed under GPL-3.0.
fn nohunger_listed(mods: &mut Vec<crate::game::native_hooks::NativeMod>) {
    use crate::game::nohunger as nohunger;
    mods.push(crate::game::native_hooks::NativeMod {
        id: nohunger::ID.into(),
        title: format!("NoHunger {}", nohunger::VERSION),
        description: nohunger::DESCRIPTION.into(),
        keys: vec![],
    });
}
