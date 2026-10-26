use serde::{Deserialize, Deserializer};

pub mod field_as_string;
pub mod option_field_as_string;

/// Deserializes JSON `null` as [`Default::default`].
///
/// Used for API objects that are nullable but easier to keep as a plain value,
/// such as an address-lookup-table map that arrives as `{}` or `null`.
pub fn null_as_default<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de> + Default,
{
    Ok(Option::<T>::deserialize(deserializer)?.unwrap_or_default())
}
