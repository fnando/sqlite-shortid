use sqlite_loadable::prelude::*;
use sqlite_loadable::{api, define_scalar_function, Result};

use rand::{rng, Rng};

/// Returns the version of the shortid extension.
///
/// # Errors
///
/// This function will return an error if the `SQLite` API calls fail.
pub fn shortid_version(
    context: *mut sqlite3_context,
    _values: &[*mut sqlite3_value],
) -> Result<()> {
    api::result_text(context, env!("CARGO_PKG_VERSION"))?;
    Ok(())
}

/// Generates a short ID.
///
/// # Errors
///
/// This function will return an error if the `SQLite` API calls fail.
pub fn shortid(context: *mut sqlite3_context, values: &[*mut sqlite3_value]) -> Result<()> {
    let prefix = if let Some(value) = values.first() {
        api::value_text(value)?
    } else {
        ""
    };

    let size = if let Some(value) = values.get(1) {
        api::value_int(value)
    } else {
        10
    };

    let mut rng = rng();
    let charset: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
    let id_length = size;
    let id: String = (0..id_length)
        .map(|_| {
            let idx = rng.random_range(0..charset.len());
            charset[idx] as char
        })
        .collect();
    api::result_text(context, format!("{prefix}{id}"))?;
    Ok(())
}

#[sqlite_entrypoint]
/// Initializes the SQLite extension.
///
/// # Errors
///
/// This function will return an error if registering functions with `SQLite` fails.
pub fn sqlite3_shortid_init(db: *mut sqlite3) -> Result<()> {
    define_scalar_function(
        db,
        "shortid_version",
        0,
        shortid_version,
        FunctionFlags::UTF8 | FunctionFlags::DETERMINISTIC,
    )?;

    define_scalar_function(db, "shortid", 0, shortid, FunctionFlags::UTF8)?;
    define_scalar_function(db, "shortid", 1, shortid, FunctionFlags::UTF8)?;
    define_scalar_function(db, "shortid", 2, shortid, FunctionFlags::UTF8)?;

    Ok(())
}
