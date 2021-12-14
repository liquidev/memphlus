//! Common things - math, conversions, etc.

use mint::IntoMint;

/// Converts linear algebra values around.
pub fn mint<T, U>(value: T) -> U
where
   T: IntoMint,
   T::MintType: Into<U>,
{
   value.into().into()
}
