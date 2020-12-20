//! Items representing various player's data.
#![allow(missing_docs)]

use crate::{FromProto, IntoProto};
use num_traits::FromPrimitive;
use sc2_proto::{
	common::Race as ProtoRace,
	sc2api::{
		AIBuild as ProtoAIBuild, Difficulty as ProtoDifficulty, PlayerType as ProtoPlayerType,
		Result as ProtoGameResult,
	},
};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Representation of game races (your gender in SC2).
#[variant_checkers]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FromStr)]
pub enum Race {
	/// Brutal mens, who try to survive in this world.
	Terran,
	/// Ruthless insects of incredibly big size. What a nightmare?
	Zerg,
	/// Kinda high-tech guys, who build cannons and batteries near your base "just for scouting".
	Protoss,
	/// Use when you didn't decide your race yet or just want to play them all.
	Random,
}
impl FromProto<ProtoRace> for Race {
	fn from_proto(race: ProtoRace) -> Self {
		match race {
			ProtoRace::Terran => Race::Terran,
			ProtoRace::Zerg => Race::Zerg,
			ProtoRace::Protoss => Race::Protoss,
			ProtoRace::Random => Race::Random,
			ProtoRace::NoRace => Race::Random,
		}
	}
}
impl IntoProto<ProtoRace> for Race {
	fn into_proto(self) -> ProtoRace {
		match self {
			Race::Terran => ProtoRace::Terran,
			Race::Zerg => ProtoRace::Zerg,
			Race::Protoss => ProtoRace::Protoss,
			Race::Random => ProtoRace::Random,
		}
	}
}
impl Default for Race {
	fn default() -> Self {
		Race::Random
	}
}

impl std::fmt::Display for Race {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Random => write!(f, "Random"),
			Self::Terran => write!(f, "Terran"),
			Self::Protoss => write!(f, "Protoss"),
			Self::Zerg => write!(f, "Zerg"),
		}
	}
}

/// Difficulty of in-game AI.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Copy, Clone, FromPrimitive, FromStr)]
#[enum_from_str(use_primitives)]
pub enum Difficulty {
	VeryEasy,
	Easy,
	Medium,
	MediumHard,
	Hard,
	Harder,
	VeryHard,
	CheatVision,
	CheatMoney,
	CheatInsane,
}
impl FromProto<ProtoDifficulty> for Difficulty {
	fn from_proto(difficulty: ProtoDifficulty) -> Self {
		match difficulty {
			ProtoDifficulty::VeryEasy => Difficulty::VeryEasy,
			ProtoDifficulty::Easy => Difficulty::Easy,
			ProtoDifficulty::Medium => Difficulty::Medium,
			ProtoDifficulty::MediumHard => Difficulty::MediumHard,
			ProtoDifficulty::Hard => Difficulty::Hard,
			ProtoDifficulty::Harder => Difficulty::Harder,
			ProtoDifficulty::VeryHard => Difficulty::VeryHard,
			ProtoDifficulty::CheatVision => Difficulty::CheatVision,
			ProtoDifficulty::CheatMoney => Difficulty::CheatMoney,
			ProtoDifficulty::CheatInsane => Difficulty::CheatInsane,
		}
	}
}
impl IntoProto<ProtoDifficulty> for Difficulty {
	fn into_proto(self) -> ProtoDifficulty {
		match self {
			Difficulty::VeryEasy => ProtoDifficulty::VeryEasy,
			Difficulty::Easy => ProtoDifficulty::Easy,
			Difficulty::Medium => ProtoDifficulty::Medium,
			Difficulty::MediumHard => ProtoDifficulty::MediumHard,
			Difficulty::Hard => ProtoDifficulty::Hard,
			Difficulty::Harder => ProtoDifficulty::Harder,
			Difficulty::VeryHard => ProtoDifficulty::VeryHard,
			Difficulty::CheatVision => ProtoDifficulty::CheatVision,
			Difficulty::CheatMoney => ProtoDifficulty::CheatMoney,
			Difficulty::CheatInsane => ProtoDifficulty::CheatInsane,
		}
	}
}

impl std::fmt::Display for Difficulty {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Difficulty::VeryEasy => write!(f, "VeryEasy"),
			Difficulty::Easy => write!(f, "Easy"),
			Difficulty::Medium => write!(f, "Medium"),
			Difficulty::MediumHard => write!(f, "MediumHard"),
			Difficulty::Hard => write!(f, "Hard"),
			Difficulty::Harder => write!(f, "Harder"),
			Difficulty::VeryHard => write!(f, "VeryHard"),
			Difficulty::CheatVision => write!(f, "CheatVision"),
			Difficulty::CheatMoney => write!(f, "CheatMoney"),
			Difficulty::CheatInsane => write!(f, "CheatInsane"),
		}
	}
}

/// Strategy build of in-game AI.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Copy, Clone, FromStr)]
pub enum AIBuild {
	RandomBuild,
	Rush,
	Timing,
	Power,
	Macro,
	Air,
}
impl FromProto<ProtoAIBuild> for AIBuild {
	fn from_proto(ai_build: ProtoAIBuild) -> Self {
		match ai_build {
			ProtoAIBuild::RandomBuild => AIBuild::RandomBuild,
			ProtoAIBuild::Rush => AIBuild::Rush,
			ProtoAIBuild::Timing => AIBuild::Timing,
			ProtoAIBuild::Power => AIBuild::Power,
			ProtoAIBuild::Macro => AIBuild::Macro,
			ProtoAIBuild::Air => AIBuild::Air,
		}
	}
}
impl IntoProto<ProtoAIBuild> for AIBuild {
	fn into_proto(self) -> ProtoAIBuild {
		match self {
			AIBuild::RandomBuild => ProtoAIBuild::RandomBuild,
			AIBuild::Rush => ProtoAIBuild::Rush,
			AIBuild::Timing => ProtoAIBuild::Timing,
			AIBuild::Power => ProtoAIBuild::Power,
			AIBuild::Macro => ProtoAIBuild::Macro,
			AIBuild::Air => ProtoAIBuild::Air,
		}
	}
}
impl Default for AIBuild {
	fn default() -> Self {
		AIBuild::RandomBuild
	}
}

/// Type of the player, used when joining a game.
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum PlayerType {
	/// Bot or Human.
	Participant,
	/// In-game AI (aka Computer).
	Computer,
	/// Player who just watch game or replay.
	Observer,
}
impl FromProto<ProtoPlayerType> for PlayerType {
	fn from_proto(player_type: ProtoPlayerType) -> Self {
		match player_type {
			ProtoPlayerType::Participant => PlayerType::Participant,
			ProtoPlayerType::Computer => PlayerType::Computer,
			ProtoPlayerType::Observer => PlayerType::Observer,
		}
	}
}
impl IntoProto<ProtoPlayerType> for PlayerType {
	fn into_proto(self) -> ProtoPlayerType {
		match self {
			PlayerType::Participant => ProtoPlayerType::Participant,
			PlayerType::Computer => ProtoPlayerType::Computer,
			PlayerType::Observer => ProtoPlayerType::Observer,
		}
	}
}

/// Computer opponent configuration used in [`run_vs_computer`](crate::client::run_vs_computer).
pub struct Computer {
	pub race: Race,
	pub difficulty: Difficulty,
	pub ai_build: Option<AIBuild>,
}
impl Computer {
	pub fn new(race: Race, difficulty: Difficulty, ai_build: Option<AIBuild>) -> Self {
		Self {
			race,
			difficulty,
			ai_build,
		}
	}
}

/// Game result for bot passed to [`on_end`](crate::Player::on_end).
#[variant_checkers]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum GameResult {
	Victory,
	Defeat,
	Tie,
	Undecided,
}
impl FromProto<ProtoGameResult> for GameResult {
	fn from_proto(player_type: ProtoGameResult) -> Self {
		match player_type {
			ProtoGameResult::Victory => GameResult::Victory,
			ProtoGameResult::Defeat => GameResult::Defeat,
			ProtoGameResult::Tie => GameResult::Tie,
			ProtoGameResult::Undecided => GameResult::Undecided,
		}
	}
}
