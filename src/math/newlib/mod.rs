// Public modules
mod asinf;
mod cosf;
mod floorf;
mod sinf;
mod tanf;

pub use self::asinf::asinf;
pub use self::cosf::cosf;
pub use self::floorf::floorf;
pub use self::sinf::sinf;
pub use self::tanf::tanf;

// Private modules
mod k_cosf;
mod k_rem_pio2f;
mod k_sinf;
mod k_tanf;
mod rem_pio2f;

// Private re-imports
use self::k_cosf::k_cosf;
use self::k_rem_pio2f::k_rem_pio2f;
use self::k_rem_pio2f::Precision;
use self::k_sinf::k_sinf;
use self::k_tanf::k_tanf;
use self::rem_pio2f::rem_pio2f;
