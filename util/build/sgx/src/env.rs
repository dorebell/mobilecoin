// Copyright (c) 2018-2023 The MobileCoin Foundation

//! SGX Build Utilities

use displaydoc::Display;
use mc_util_build_script::Environment;
use std::{env::VarError, fmt, result::Result as StdResult};

pub const ENV_IAS_MODE: &str = "IAS_MODE";
pub const ENV_SGX_MODE: &str = "SGX_MODE";

/// An enumeration of environment errors which occur when parsing SGX
/// environments
#[derive(Display)]
pub enum Error {
    /// The IAS mode '{0}' is unknown. Please see https://github.com/mobilecoinfoundation/mobilecoin/blob/master/BUILD.md#build-configuration for more information.
    UnknownIasMode(String),

    /// The SGX mode '{0}' is unknown. Please see https://github.com/mobilecoinfoundation/mobilecoin/blob/master/BUILD.md#build-configuration for more information.
    UnknownSgxMode(String),

    /// There was an error reading an environment variable '{0}': {1}. Please see https://github.com/mobilecoinfoundation/mobilecoin/blob/master/BUILD.md#build-configuration for more information.
    Variable(&'static str, VarError),
}

// Implement Debug by forwarding to Display so that .expect() shows the Display
// text
impl fmt::Debug for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::Display>::fmt(self, fmt)
    }
}

type Result<T> = StdResult<T, Error>;

// Wrapper around std::env::var which preserves more context about the error
fn var_helper(var_name: &'static str) -> Result<String> {
    std::env::var(var_name).map_err(|var_error| Error::Variable(var_name, var_error))
}

/// The style of interaction with IAS
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum IasMode {
    /// When contacting IAS, use the production API service
    Production,
    /// When contacting IAS, use the development API service
    Development,
}

impl TryFrom<&str> for IasMode {
    type Error = Error;

    fn try_from(src: &str) -> Result<Self> {
        match src {
            "PROD" => Ok(IasMode::Production),
            "DEV" => Ok(IasMode::Development),
            other => Err(Error::UnknownIasMode(other.to_owned())),
        }
    }
}

/// The type of SGX library linkage
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum SgxMode {
    /// Link against hardware-enabled SGX libraries
    Hardware,
    /// Link against SGX simulation libraries
    Simulation,
}

impl TryFrom<&str> for SgxMode {
    type Error = Error;

    fn try_from(src: &str) -> Result<Self> {
        match src {
            "HW" => Ok(SgxMode::Hardware),
            "SW" => Ok(SgxMode::Simulation),
            other => Err(Error::UnknownSgxMode(other.to_owned())),
        }
    }
}

/// The SGX environment variable reader structure
#[derive(Clone, Debug)]
pub struct SgxEnvironment {
    ias_mode: IasMode,
    sgx_mode: SgxMode,
}

impl SgxEnvironment {
    /// Construct a new SGX environment reader.
    pub fn new(env: &Environment) -> Result<Self> {
        // Prioritize feature selection over environment variables.
        let ias_mode = if env.feature("ias-dev") {
            IasMode::Development
        } else {
            let ias_mode = var_helper(ENV_IAS_MODE)?;
            IasMode::try_from(ias_mode.as_str())?
        };

        let sgx_mode = if env.feature("sgx-sim") {
            SgxMode::Simulation
        } else {
            let sgx_mode = var_helper(ENV_SGX_MODE)?;
            SgxMode::try_from(sgx_mode.as_str())?
        };

        Ok(Self { ias_mode, sgx_mode })
    }

    /// Get the IAS mode requested
    pub fn ias_mode(&self) -> IasMode {
        self.ias_mode
    }

    /// Get the SGX mode requested
    pub fn sgx_mode(&self) -> SgxMode {
        self.sgx_mode
    }
}
