//! Contains pure types, no IO.
//!
//! Most of these are more descriptive wrappers around integers.
use std::result::Result;

/// SCX Format version.
pub type SCXVersion = [u8; 4];

/// Could not parse a diplomatic stance because given number is an unknown stance ID.
#[derive(Debug, Clone, Copy)]
pub struct ParseDiplomaticStanceError(i32);

impl std::fmt::Display for ParseDiplomaticStanceError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "invalid diplomatic stance {} (must be 0/1/3)", self.0)
    }
}

impl std::error::Error for ParseDiplomaticStanceError {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiplomaticStance {
    Ally = 0,
    Neutral = 1,
    Enemy = 3,
}

impl DiplomaticStance {
    pub fn try_from(n: i32) -> Result<Self, ParseDiplomaticStanceError> {
        match n {
            0 => Ok(DiplomaticStance::Ally),
            1 => Ok(DiplomaticStance::Neutral),
            3 => Ok(DiplomaticStance::Enemy),
            n => Err(ParseDiplomaticStanceError(n)),
        }
    }
}

impl From<DiplomaticStance> for i32 {
    fn from(stance: DiplomaticStance) -> i32 {
        match stance {
            DiplomaticStance::Ally => 0,
            DiplomaticStance::Neutral => 1,
            DiplomaticStance::Enemy => 3,
        }
    }
}

/// Could not parse a data set because given number is an unknown data set ID.
#[derive(Debug, Clone, Copy)]
pub struct ParseDataSetError(i32);

impl std::fmt::Display for ParseDataSetError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "invalid data set {} (must be 0/1)", self.0)
    }
}

impl std::error::Error for ParseDataSetError {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataSet {
    BaseGame,
    Expansions,
}

impl DataSet {
    pub fn try_from(n: i32) -> Result<Self, ParseDataSetError> {
        match n {
            0 => Ok(DataSet::BaseGame),
            1 => Ok(DataSet::Expansions),
            n => Err(ParseDataSetError(n)),
        }
    }
}

impl From<DataSet> for i32 {
    fn from(id: DataSet) -> i32 {
        match id {
            DataSet::BaseGame => 0,
            DataSet::Expansions => 1,
        }
    }
}

/// Could not parse a DLC package identifier because given number is an unknown DLC ID.
#[derive(Debug, Clone, Copy)]
pub struct ParseDLCPackageError(i32);

impl std::fmt::Display for ParseDLCPackageError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "unknown dlc package {}", self.0)
    }
}

impl std::error::Error for ParseDLCPackageError {}

/// An HD Edition DLC identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DLCPackage {
    AgeOfKings,
    AgeOfConquerors,
    TheForgotten,
    AfricanKingdoms,
    RiseOfTheRajas,
}

impl DLCPackage {
    pub fn try_from(n: i32) -> Result<Self, ParseDLCPackageError> {
        match n {
            2 => Ok(DLCPackage::AgeOfKings),
            3 => Ok(DLCPackage::AgeOfConquerors),
            4 => Ok(DLCPackage::TheForgotten),
            5 => Ok(DLCPackage::AfricanKingdoms),
            6 => Ok(DLCPackage::RiseOfTheRajas),
            n => Err(ParseDLCPackageError(n)),
        }
    }
}

impl From<DLCPackage> for i32 {
    fn from(dlc_id: DLCPackage) -> i32 {
        match dlc_id {
            DLCPackage::AgeOfKings => 2,
            DLCPackage::AgeOfConquerors => 3,
            DLCPackage::TheForgotten => 4,
            DLCPackage::AfricanKingdoms => 5,
            DLCPackage::RiseOfTheRajas => 6,
        }
    }
}

/// Could not parse a starting age because given number refers to an unknown age.
#[derive(Debug, Clone, Copy)]
pub struct ParseStartingAgeError {
    version: f32,
    found: i32,
}

impl std::fmt::Display for ParseStartingAgeError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let expected = if self.version < 1.25 { "-1-4" } else { "-1-6" };
        write!(
            f,
            "invalid starting age {} (must be {})",
            self.found, expected
        )
    }
}

impl std::error::Error for ParseStartingAgeError {}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum StartingAge {
    /// Use the game default.
    Default = -1,
    /// Start in the Dark Age with Nomad resources.
    Nomad = -2,
    /// Start in the Dark Age.
    DarkAge = 0,
    /// Start in the Feudal Age.
    FeudalAge = 1,
    /// Start in the Castle Age.
    CastleAge = 2,
    /// Start in the Imperial Age.
    ImperialAge = 3,
    /// Start in the Imperial Age with all technologies researched.
    PostImperialAge = 4,
}

impl StartingAge {
    /// Convert a starting age number to the appropriate enum for a particular
    /// data version.
    pub fn try_from(n: i32, version: f32) -> Result<Self, ParseStartingAgeError> {
        if version < 1.25 {
            match n {
                -1 => Ok(StartingAge::Default),
                0 => Ok(StartingAge::DarkAge),
                1 => Ok(StartingAge::FeudalAge),
                2 => Ok(StartingAge::CastleAge),
                3 => Ok(StartingAge::ImperialAge),
                4 => Ok(StartingAge::PostImperialAge),
                _ => Err(ParseStartingAgeError { version, found: n }),
            }
        } else {
            match n {
                -1 | 0 => Ok(StartingAge::Default),
                1 => Ok(StartingAge::Nomad),
                2 => Ok(StartingAge::DarkAge),
                3 => Ok(StartingAge::FeudalAge),
                4 => Ok(StartingAge::CastleAge),
                5 => Ok(StartingAge::ImperialAge),
                6 => Ok(StartingAge::PostImperialAge),
                _ => Err(ParseStartingAgeError { version, found: n }),
            }
        }
    }

    pub fn to_i32(self, version: f32) -> i32 {
        if version < 1.25 {
            match self {
                StartingAge::Default => -1,
                StartingAge::Nomad | StartingAge::DarkAge => 0,
                StartingAge::FeudalAge => 1,
                StartingAge::CastleAge => 2,
                StartingAge::ImperialAge => 3,
                StartingAge::PostImperialAge => 4,
            }
        } else {
            match self {
                StartingAge::Default => 0,
                StartingAge::Nomad => 1,
                StartingAge::DarkAge => 2,
                StartingAge::FeudalAge => 3,
                StartingAge::CastleAge => 4,
                StartingAge::ImperialAge => 5,
                StartingAge::PostImperialAge => 6,
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VictoryCondition {
    Capture = 0,
    Create = 1,
    Destroy = 2,
    DestroyMultiple = 3,
    BringToArea = 4,
    BringToObject = 5,
    Attribute = 6,
    Explore = 7,
    CreateInArea = 8,
    DestroyAll = 9,
    DestroyPlayer = 10,
    Points = 11,
}

/// All the versions an SCX file uses in a single struct.
#[derive(Debug, Clone, PartialEq)]
pub struct VersionBundle {
    /// The version of the 'container' file format.
    pub format: SCXVersion,
    /// The version of the header.
    pub header: u32,
    /// The version of the HD Edition DLC Options, only if `header` >= 3.
    pub dlc_options: i32,
    /// The compressed data version.
    pub data: f32,
    /// The version of embedded bitmaps.
    pub picture: u32,
    /// The version of the victory conditions data.
    pub victory: f32,
    /// The version of the trigger system.
    pub triggers: f64,
}

impl VersionBundle {
    /// A version bundle with the parameters AoE1 uses by default.
    pub fn aoe() -> Self {
        unimplemented!()
    }

    /// A version bundle with the parameters AoE1: Rise of Rome uses by default.
    pub fn ror() -> Self {
        unimplemented!()
    }

    /// A version bundle with the parameters AoK uses by default.
    pub fn aok() -> Self {
        unimplemented!()
    }

    /// A version bundle with the parameters AoC uses by default
    pub fn aoc() -> Self {
        Self {
            format: *b"1.21",
            header: 2,
            dlc_options: -1,
            data: 1.22,
            picture: 1,
            victory: 2.0,
            triggers: 1.6,
        }
    }

    /// A version bundle with the parameters UserPatch 1.4 uses by default.
    pub fn userpatch_14() -> Self {
        Self::aoc()
    }

    /// A version bundle with the parameters UserPatch 1.5 uses by default.
    pub fn userpatch_15() -> Self {
        Self::userpatch_14()
    }

    /// A version bundle with the parameters HD Edition uses by default.
    pub fn hd_edition() -> Self {
        Self {
            format: *b"1.21",
            header: 3,
            dlc_options: 1000,
            data: 1.26,
            picture: 3,
            victory: 2.0,
            triggers: 1.6,
        }
    }

    /// Returns whether this version is (likely) for an AoK scenario.
    pub fn is_aok(&self) -> bool {
        match &self.format {
            b"1.18" | b"1.19" | b"1.20" => true,
            _ => false,
        }
    }

    /// Returns whether this version is (likely) for an AoC scenario.
    pub fn is_aoc(&self) -> bool {
        self.format == *b"1.21" && self.data <= 1.22
    }

    /// Returns whether this version is (likely) for an HD Edition scenario.
    pub fn is_hd_edition(&self) -> bool {
        self.format == *b"1.21" || self.format == *b"1.22" && self.data > 1.22
    }
}
