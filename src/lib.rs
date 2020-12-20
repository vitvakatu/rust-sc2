/*!
# Introduction

## Installing

Install Rust >= 1.42.0

Warning: Compilation is broken in rustc 1.45.0 - 1.46.0, you'll get following error:
```text
thread 'rustc' has overflowed its stack
error: could not compile `rust-sc2`.
```

Add to dependencies in Cargo.toml:
```toml
[dependencies]
rust-sc2 = "1"
```
Or if you want developer version directly from github:
```toml
[dependencies]
rust-sc2 = { git = "https://github.com/UltraMachine/rust-sc2" }
```

## Making a bot

Making bots with `rust-sc2` is pretty easy:
```
use::rust_sc2::prelude::*;

#[bot]
#[derive(Default)]
struct MyBot;
impl Player for MyBot {
    // This settings are used to connect bot to the game.
    fn get_player_settings(&self) -> PlayerSettings {
        PlayerSettings::new(Race::Random, Some("BotName"))
    }
    // This method will be called automatically each game step.
    // Main bot's logic should be here.
    // Bot's observation updates before each step.
    fn on_step(&mut self, iteration: usize) -> SC2Result<()> {
        /* Your code here */
        Ok(())
    }
}

fn main() -> SC2Result<()> {
    run_vs_computer(
        // Pass mutable referece to your bot here.
        &mut MyBot::default(),
        // Opponent configuration.
        Computer::new(Race::Random, Difficulty::VeryEasy, None),
        // Map name. Panics if map doesn't exists in "StarCraft II/Maps" folder.
        "EternalEmpireLE",
        // Additional settings:
        // LaunchOptions {
        //     sc2_version: Option<&str>, // Default: None - Latest available patch.
        //     save_replay_as: Option<&str>, // Default: None - Doesn't save replay.
        //     realtime: bool, // Default: false
        // }
        LaunchOptions::default(),
    )
}
```

Add some cool stuff and watch how it destroys the opponent.

If you are careful guy who don't trust random macros that can destroy your PC,
see [`#[bot]`][b] macro documentation to understand how it's implemented.
You always can do the same thing by hands if needed.

## What bot can see?

### Self information
#### Common
| Field                 | Type       | Description                                           |
|-----------------------|------------|-------------------------------------------------------|
| `self.race`           | [`Race`]   | The actual race your bot plays.                       |
| `self.player_id`      | `u32`      | Bot's in-game id (usually `1` or `2` in 1v1 matches). |
| `self.minerals`       | `u32`      | Amount of minerals bot has.                           |
| `self.vespene`        | `u32`      | Amount of gas bot has.                                |
| `self.supply_army`    | `u32`      | Amount of supply used by army.                        |
| `self.supply_workers` | `u32`      | Amount of supply used by workers.                     |
| `self.supply_cap`     | `u32`      | The supply limit.                                     |
| `self.supply_used`    | `u32`      | Total supply used.                                    |
| `self.supply_left`    | `u32`      | Amount of free supply.                                |
| `self.start_location` | [`Point2`] | Bot's starting location.                              |
| `self.start_center`   | [`Point2`] | Bot's resource center on start location.              |

#### Race values
| Field                             | Type                  | Description                                             |
|-----------------------------------|-----------------------|---------------------------------------------------------|
| `self.race_values.start_townhall` | [`UnitTypeId`]        | Default townhall which can be built by a worker.        |
| `self.race_values.townhalls`      | `Vec`<[`UnitTypeId`]> | All possible forms of townhall for your race.           |
| `self.race_values.gas`            | [`UnitTypeId`]        | Building used to extract gas from vespene geysers.      |
| `self.race_values.rich_gas`       | [`UnitTypeId`]        | Building used to extract gas from rich vespene geysers. |
| `self.race_values.supply`         | [`UnitTypeId`]        | Supply provider for your race.                          |
| `self.race_values.worker`         | [`UnitTypeId`]        | Worker of your race.                                    |

### Common opponent's information
| Field                     | Type       | Description                                              |
|---------------------------|------------|----------------------------------------------------------|
| `self.enemy_race`         | [`Race`]   | Requested race of your opponent.                         |
| `self.enemy_player_id`    | `u32`      | Opponent in-game id (usually `1` or `2` in 1v1 matches). |
| `self.opponent_id`        | `String`   | Opponent id on ladder, filled in `--OpponentId`.         |
| `self.enemy_start`        | [`Point2`] | Opponent's starting location.                            |
| `self.enemy_start_center` | [`Point2`] | Opponents's resource center on start location.           |

### Ramps
| Field             | Type            | Description                   |
|-------------------|-----------------|-------------------------------|
| `self.ramp.my`    | [`Ramp`]        | Your main base ramp.          |
| `self.ramp.enemy` | [`Ramp`]        | Opponent's main base ramp.    |
| `self.ramp.all`   | `Vec`<[`Ramp`]> | All the ramps around the map. |

### Units
#### Common
| Field                        | Type            | Description                                                                |
|------------------------------|-----------------|----------------------------------------------------------------------------|
| `self.units.all`             | [`Units`]       | All the units including owned, enemies and neutral.                        |
| `self.units.my`              | [`PlayerUnits`] | Your's only units.                                                         |
| `self.units.enemy`           | [`PlayerUnits`] | Opponent's units, on current step.                                         |
| `self.units.cached`          | [`PlayerUnits`] | Opponent's units, but also contains some hidden units from previous steps. |
| `self.units.mineral_fields`  | [`Units`]       | All mineral fields on the map.                                             |
| `self.units.vespene_geysers` | [`Units`]       | All vespene geysers on the map.                                            |
| `self.units.resources`       | [`Units`]       | All resources (both minerals and geysers) on the map.                      |
| `self.units.destructables`   | [`Units`]       | Destructable rocks and other trash.                                        |
| `self.units.watchtowers`     | [`Units`]       | Watchtowers reveal area around them if there're any ground units near.     |
| `self.units.inhibitor_zones` | [`Units`]       | Inhubitor zones slow down movement speed of nearby units.                  |

#### What `PlayerUnits` consists of?
All fields are collections of [`Units`]:

| Field            | Description                                                                                              |
|------------------|----------------------------------------------------------------------------------------------------------|
| `.all`           | All player units (includes both units and structures).                                                   |
| `.units`         | Units only, without structures.                                                                          |
| `.structures`    | Structures only.                                                                                         |
| `.townhalls`     | From all structures only townhalls here.                                                                 |
| `.workers`       | Workers only (doesn't include MULEs).                                                                    |
| `.gas_buildings` | The gas buildings on geysers used to gather gas.                                                         |
| `.larvas`        | Most of zerg units are morphed from it (Populated for zergs only).                                       |
| `.placeholders`  | Kind of things that appear when you order worker to build something but construction didn't started yet. |

### Other information
| Field                  | Type                           | Description                                                                    |
|------------------------|--------------------------------|--------------------------------------------------------------------------------|
| `self.time`            | `f32`                          | In-game time in seconds.                                                       |
| `self.expansions`      | `Vec`<([`Point2`],[`Point2`])> | All expansions stored in (location, resource center) pairs.                    |
| `self.vision_blockers` | `Vec`<[`Point2`]>              | Obstacles on map which block vision of ground units, but still pathable.       |
| `self.game_info`       | [`GameInfo`]                   | Information about map: pathing grid, building placement, terrain height.       |
| `self.game_data`       | [`GameData`]                   | Constant information about abilities, unit types, upgrades, buffs and effects. |
| `self.state`           | [`GameState`]                  | Information about current state, updated each step.                            |

## What bot can do?

### Units training
Training as much as possible marines may look like:
```
// Iterating bot's barracks which are completed (ready) and not already training (idle).
for barrack in self.units.my.structures.iter().of_type(UnitTypeId::Barracks).ready().idle() {
    // Checking if we have enough resources and supply.
    if self.can_afford(UnitTypeId::Marine, true) {
        // Ordering barracks to train marine.
        barrack.train(UnitTypeId::Marine, false);
        // Subtracting resources and suply used to train.
        self.subtract_resources(UnitTypeId::Marine, true);
    // Can't afford more marines. Stopping the iterator.
    } else {
        break;
    }
}
```

### Building structures
Building up to 5 barracks might look like:
```
// Building near start location, but a bit closer to map center to not accidentally block mineral line.
let main_base = self.start_location.towards(self.game_info.map_center, 8.0);

// Checking if we have enough resources to afford a barrack.
if self.can_afford(UnitTypeId::Barracks, false)
    // Checking if total (current + ordered) number of barracks less than we want.
    && self.counter().all().count(UnitTypeId::Barracks) < 5
{
    // Finding a perfect location for a building.
    if let Some(location) = self.find_placement(
        UnitTypeId::Barracks,
        main_base,
        PlacementOptions {
            // Step increased here to leave some space between barracks,
            // so units won't stuck when coming out of them.
            step: 4,
            ..Default::default()
        },
    ) {
        if let Some(builder) = self.units
            // Finding workers which are not already building.
            .my.workers.iter().filter(|w| !w.is_constructing())
            // Selecting closest to our build location.
            .closest(location)
        {
            // Ordering scv to build barracks finally.
            builder.build(UnitTypeId::Barracks, location, false);
            // Subtracting resources used to build it.
            self.subtract_resources(UnitTypeId::Barracks, false);
        }
    }
}
```

### Expanding
Building new CCs might look like:
```
// Checking if we have enough minerals for new expand.
if self.can_afford(UnitTypeId::CommandCenter, false)
    // Checking if we not already building new base.
    && self.counter().ordered().count(UnitTypeId::CommandCenter) == 0
{
    // Getting next closest expansion
    if let Some((location, _resource_center)) = self.get_expansion() {
        if let Some(builder) = self.units
            // Finding workers which are not already building.
            .my.workers.iter().filter(|w| !w.is_constructing())
            // Selecting closest to our build location.
            .closest(location)
        {
            // Ordering scv to build new base.
            builder.build(UnitTypeId::CommandCenter, location, false);
            // Subtracting resources used to build CC.
            self.subtract_resources(UnitTypeId::CommandCenter, false);
        }
    }
}
```

### Units micro
Attacking when marines >= 15, defending base before:
```
let main_base = self.start_location.towards(self.game_info.map_center, 8.0);
let marines = self.units.my.units.iter().of_type(UnitTypeId::Marine).idle();

if self.counter().count(UnitTypeId::Marine) >= 15 {
    let targets = &self.units.enemy.all;
    if targets.is_empty() {
        for m in marines {
            m.attack(Target::Pos(self.enemy_start), false);
        }
    } else {
        for m in marines {
            m.attack(Target::Tag(targets.closest(m)?.tag), false);
        }
    }
} else {
    let targets = self.units.enemy.all.closer(25.0, self.start_location);
    if targets.is_empty() {
        for m in marines {
            m.move_to(Target::Pos(self.main_base), false);
        }
    } else {
        for m in marines {
            m.attack(Target::Tag(targets.closest(m)?.tag), false);
        }
    }
}
```

## Prepearing for ladder

There're community organized ladders for bots:
- [SC2AI] - Runs games on windows and latest patch of SC2.
- [AI Arena] - Runs games on linux and patch 4.10.

Both use the same kind of system. In order to get your bot ready for ladder, make it parse following args:
- `--LadderServer` - IP address.
- `--OpponentId` - Id of the opponent on ladder.
- `--GamePort` - Port.
- `--StartPort` - Yet another port.

If you're too lazy to add argparser yourself, see [`examples`] folder,
some examples already have fully functional parser.

Then call [`run_ladder_game`](client::run_ladder_game) this way:
```
run_ladder_game(
    &mut bot,
    ladder_server, // Should be 127.0.0.1 by default.
    game_port,
    start_port,
    opponent_id, // Or `None`.
)
```

The API will do the rest.

Since [SC2AI] and [AI Arena] run the games on different platforms
you'll need to provide suitable binaries for each ladder.

Because of version differences ids are conditionally compiled for windows and linux.

[SC2AI]: https://sc2ai.net
[AI Arena]: https://ai-arena.net
[`examples`]: https://github.com/UltraMachine/rust-sc2/tree/master/examples

[`Race`]: player::Race
[`Point2`]: geometry::Point2
[`UnitTypeId`]: ids::UnitTypeId
[`Ramp`]: ramp::Ramp
[`Units`]: units::Units
[`PlayerUnits`]: units::PlayerUnits
[`GameInfo`]: game_info::GameInfo
[`GameData`]: game_data::GameData
[`GameState`]: game_state::GameState
[b]: macro@bot
*/
// #![warn(missing_docs)]
//#![deny(broken_intra_doc_links)]

#[macro_use]
extern crate num_derive;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate sc2_macro;
#[macro_use]
extern crate itertools;
#[macro_use]
extern crate maplit;
#[macro_use]
extern crate log;

use std::{rc::Rc, cell::RefCell};

/// The most frequent used items and various traits here.
/// Prefered usage: `use::rust_sc2::prelude::*;`.
pub mod prelude {
	#[cfg(feature = "rayon")]
	pub use crate::distance::rayon::{ParCenter, ParDistanceIterator, ParDistanceSlice};
	#[cfg(feature = "rayon")]
	pub use crate::units::rayon::ParUnitsIterator;
	pub use crate::{
		action::Target,
		bot::PlacementOptions,
		client::{
			run_ladder_game, run_vs_computer, run_vs_human, LaunchOptions, RunnerMulti, RunnerSingle,
			SC2Result,
		},
		consts::{ALL_PRODUCERS, PRODUCERS, RESEARCHERS, TECH_REQUIREMENTS},
		distance::{Center, Distance, DistanceIterator, DistanceSlice},
		geometry::Point2,
		ids::*,
		player::{AIBuild, Computer, Difficulty, GameResult, Race},
		unit::Unit,
		units::{iter::UnitsIterator, Units},
		Event, Player, PlayerSettings,
	};
	#[doc(no_inline)]
	pub use sc2_macro::{bot, bot_new};
}

mod paths;

pub mod action;
pub mod api;
pub mod bot;
pub mod client;
pub mod consts;
pub mod debug;
pub mod distance;
pub mod game_data;
pub mod game_info;
pub mod game_state;
pub mod geometry;
pub mod ids;
pub mod pixel_map;
pub mod player;
pub mod ramp;
pub mod score;
pub mod unit;
pub mod units;
pub mod utils;

use player::{GameResult, Race};

/**
`#[bot]` macro implements [`Deref`]`<Target = `[`Bot`]`>` and [`DerefMut`]`<Target = `[`Bot`]`>` for your struct.
Implementing this traits allows you to access [`Bot`] fields and methods on your struct through `self`.

[`Bot`]: bot::Bot
[`Deref`]: std::ops::Deref
[`DerefMut`]: std::ops::DerefMut

Usage:
```
#[bot]
struct MyBot;

impl MyBot {
    fn my_func(&self) {
        println!("my race: {:?}", self.race);
        println!("current \"game_step\": {}", self.game_step());
    }
    fn my_func_mut(&mut self) {
        self.chat("It works!");
        self.set_game_step(8);
    }
}
```

## What this macro does?
It adds hidden field where data of [`Bot`] stored.

Also this adds [`Deref`] and [`DerefMut`] implementations to access [`Bot`] data just through `self.whatever`
instead of `self._bot.whatever`.

## What compiler does?
When you type this:
```
self.whatever_from_bot
```
Since [`Deref`] is implemented, compiler performs auto dereference to access [`Bot`]:
```
(*self).whatever_from_bot
```
The way how [`Deref`] implemented determines behavior of dereference operation, so actually it becomes:
```
self._bot.whatever_from_bot
```

# Macro Inside

This:
```
#[bot]
struct MyBot;
```
Expands to:
```
struct MyBot {
    _bot: Bot,
}
impl Deref for MyBot {
    type Target = Bot;

    fn deref(&self) -> &Self::Target {
        &self._bot
    }
}
impl DerefMut for MyBot {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self._bot
    }
}
```

And this:
```
#[bot]
struct MyBot {
    field: Type,
    field2: Type2,
}
```
Expands to:
```
struct MyBot {
    _bot: Bot,
    field: Type,
    field2: Type2,
}
impl Deref for MyBot {
    type Target = Bot;

    fn deref(&self) -> &Self::Target {
        &self._bot
    }
}
impl DerefMut for MyBot {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self._bot
    }
}
```
*/
#[doc(inline)]
pub use sc2_macro::bot;

/**
`#[bot_new]` macro adds initialization of field added by [`#[bot]`](macro@bot) macro.

Usage:
```
#[bot]
struct MyBot;

impl MyBot {
    #[bot_new]
    fn new() -> MyBot {
        MyBot
    }
}
```
If your bot implements `Default` then you don't need it, since [`Bot`](bot::Bot)
implements `Default` too and will be filled automatically:
```
#[bot]
#[derive(Default)]
struct MyBot;

fn main() {
    let bot = MyBot::default();
}
```
```
#[bot]
#[derive(Default)]
struct MyBot {
    field: Type,
    field2: Type2,
    field_n: TypeN,
}

impl MyBot {
    fn new() -> MyBot {
        MyBot {
            field: Type::init(),
            field2: Type2::init(),
            ..Default::default(),
        }
    }
}

fn main() {
    let bot = MyBot::new();
}
```

# Macro Inside

This:
```
#[bot]
struct MyBot;

impl MyBot {
    #[bot_new]
    fn new() -> MyBot {
        MyBot
    }
}
```
Expands to:
```
struct MyBot {
    _bot: Bot,
}

impl MyBot {
    fn new() -> MyBot {
        MyBot {
            _bot: Default::default(),
        }
    }
}
```

And this:
```
#[bot]
struct MyBot {
    field: Type,
    field2: Type2,
}

impl MyBot {
    #[bot_new]
    fn new() -> MyBot {
        MyBot {
            field: Type::init(),
            field2: Type2::init(),
        }
    }
}
```
Expands to:
```
struct MyBot {
    _bot: Bot,
    field: Type,
    field2: Type2,
}

impl MyBot {
    fn new() -> MyBot {
        MyBot {
            _bot: Default::default(),
            field: Type::init(),
            field2: Type2::init(),
        }
    }
}
```
*/
#[doc(inline)]
pub use sc2_macro::bot_new;

#[doc(inline)]
pub use client::SC2Result;
/**
Request to the SC2 API.

# Usage
```
let mut request = Request::new();

/* modify request through it's methods */

let response = self.api().send(request)?;
```
*/
pub use sc2_proto::sc2api::Request;

/// Settings that must be provided by a player when joining a game.
pub struct PlayerSettings {
	race: Race,
	name: Option<String>,
	raw_affects_selection: bool,
	raw_crop_to_playable_area: bool,
}
impl PlayerSettings {
	/// Constructs new settings with given `Race` and name.
	/// `raw_affects_selection` and `raw_crop_to_playable_area` are `false` by default.
	pub fn new(race: Race, name: Option<&str>) -> Self {
		Self {
			race,
			name: name.map(|n| n.to_string()),
			raw_affects_selection: false,
			raw_crop_to_playable_area: false,
		}
	}
	/// Constructs new settings with more options given.
	///
	/// `raw_affects_selection`: Bot will select units to which it gives orders.
	///
	/// `raw_crop_to_playable_area`: Maps and all coordinates will be crooped to playable area.
	/// That means map will start from (0, 0)
	/// and finsh on (playable area length by `x`, playable area length by `y`).
	pub fn configured(
		race: Race,
		name: Option<&str>,
		raw_affects_selection: bool,
		raw_crop_to_playable_area: bool,
	) -> Self {
		Self {
			race,
			name: name.map(|n| n.to_string()),
			raw_affects_selection,
			raw_crop_to_playable_area,
		}
	}
}

/// Events that happen in game.
/// Passed to [`on_event`](Player::on_event).
pub enum Event {
	/// Unit died or structure destroyed (all units: your, enemy, neutral).
	UnitDestroyed(u64),
	/// Unit finished training (your only).
	UnitCreated(u64),
	/// Worker started to build a structure (your only).
	ConstructionStarted(u64),
	/// Construction of a structure finished (your only).
	ConstructionComplete(u64),
	/// Detected actual race of random opponent.
	RandomRaceDetected(Race),
}

/// Trait that bots must implement.
pub trait Player {
	/// Returns settings used to connect bot to the game.
	fn get_player_settings(&self) -> PlayerSettings;
	/// Called once on first step (i.e on game start).
	fn on_start(&mut self) -> SC2Result<()> {
		Ok(())
	}
	/// Called on every game step. (Main logic of the bot should be here)
	fn on_step(&mut self, _iteration: usize) -> SC2Result<()> {
		Ok(())
	}
	/// Called once on last step with a result for your bot.
	fn on_end(&self, _result: GameResult) -> SC2Result<()> {
		Ok(())
	}
	/// Called when different events happen.
	fn on_event(&mut self, _event: Event) -> SC2Result<()> {
		Ok(())
    }
    
    fn bot(&self) -> Rc<RefCell<crate::bot::Bot>>;
}

trait FromProto<T>
where
	Self: Sized,
{
	fn from_proto(p: T) -> Self;
}

trait IntoSC2<T> {
	fn into_sc2(self) -> T;
}
impl<T, U: FromProto<T>> IntoSC2<U> for T {
	fn into_sc2(self) -> U {
		U::from_proto(self)
	}
}

trait TryFromProto<T>
where
	Self: Sized,
{
	fn try_from_proto(p: T) -> Option<Self>;
}

trait IntoProto<T> {
	fn into_proto(self) -> T;
}

/*trait FromSC2<T> {
	fn from_sc2(s: T) -> Self;
}
impl<T, U: IntoProto<T>> FromSC2<U> for T {
	fn from_sc2(s: U) -> T {
		s.into_proto()
	}
}*/
