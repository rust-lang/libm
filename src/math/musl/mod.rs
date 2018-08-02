// Public modules
mod acosf;
mod asinf;
mod cbrtf;
mod cosf;
mod exp2f;
mod floorf;
mod hypotf;
mod sinf;
mod tanf;

pub use self::acosf::acosf;
pub use self::asinf::asinf;
pub use self::cbrtf::cbrtf;
pub use self::cosf::cosf;
pub use self::exp2f::exp2f;
pub use self::floorf::floorf;
pub use self::hypotf::hypotf;
pub use self::sinf::sinf;
pub use self::tanf::tanf;

// Private modules
mod k_cosf;
mod k_sinf;
mod k_tanf;
mod rem_pio2f;

// Private re-imports
use self::k_cosf::k_cosf;
use self::k_sinf::k_sinf;
use self::k_tanf::k_tanf;
use self::rem_pio2f::rem_pio2f;
