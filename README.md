# Echoes of Ash

Echoes of Ash is a dependency-free, command-line adventure game written in Rust. Explore a shattered realm, grow stronger through combat, collect three star-shards, and breach the Ashen Citadel.

## Run

```text
cargo run
```

The game accepts commands at the location prompt:

| Command | Description |
| --- | --- |
| `look` | Describe the current location and exits |
| `map` | Show the realm map and explored locations |
| `travel <haven\|woods\|mine\|temple\|citadel>` | Move between locations |
| `explore` | Trigger the encounter in the current location |
| `quest` | Show the active shard quest and reputation |
| `talk <quartermaster\|rowan>` | Speak with Haven NPCs |
| `status` | Show health, level, XP, gold, weapon, and shards |
| `inventory` | List consumables |
| `rest` | Restore the party at Haven |
| `buy potion` | Purchase a healing potion at Haven |
| `save` / `load` | Persist or restore progress from `savegame.txt` |
| `help` / `quit` | Show commands or exit |

During combat, use `attack`, `fists`, `spell`, `potion`, `inspect`, or `run`.

The interface includes a live health/gold/shard HUD, progress bars, color-coded status
markers, a map panel, and quest panels. Each dangerous region awards its shard only
once; subsequent expeditions provide salvage gold instead. Saves include progression,
reputation, and quest state.

Traveling to a dangerous region can also trigger a 35% roaming-monster ambush. Combat
damage is randomized around each weapon, spell, and enemy's base power (within 25%),
while armor and minimum-damage rules still apply.

## Development

```text
cargo fmt -- --check
cargo test
cargo clippy -- -D warnings
```

The game uses only the Rust standard library, so it can be built offline without downloading dependencies.
