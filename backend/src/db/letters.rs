use crate::models;

use anyhow::Result;
use diesel::prelude::*;
use uuid::Uuid;

pub fn insert(
    conn: &mut PgConnection,
    entry_author: impl AsRef<str>,
    entry_message: impl AsRef<str>,
    entry_secret: bool,
) -> Result<models::Letter> {
    let entry_author = entry_author.as_ref();
    let entry_message = entry_message.as_ref();

    log::info!("[insert] posting letter");
    log::info!("[insert] author = {:?}", entry_author.len());
    log::info!("[insert] secret = {:?}", entry_secret);

    use crate::schema::letters::dsl::*;

    let new_letter = models::NewLetter {
        author: &entry_author,
        message: &entry_message,
        secret: entry_secret,
    };

    Ok(diesel::insert_into(letters)
        .values(&new_letter)
        .get_result(conn)?)
}

pub fn get_all(conn: &mut PgConnection, offset: usize) -> Result<Vec<models::Letter>> {
    log::info!("getting all entries");
    use crate::schema::letters::dsl::*;

    let collection = letters
        .offset(offset.max(0) as i64)
        .limit(10)
        .load::<models::Letter>(conn)?;

    Ok(collection)
}

pub fn get_all_public(conn: &mut PgConnection, offset: usize) -> Result<Vec<models::Letter>> {
    log::info!("getting all public entries");
    use crate::schema::letters::dsl::*;

    let collection = letters
        .filter(secret.eq(false))
        .offset(offset.max(0) as i64)
        .limit(10)
        .load::<models::Letter>(conn)?;

    Ok(collection)
}

pub fn find_by_id(conn: &mut PgConnection, uid: &Uuid) -> Result<Option<models::Letter>> {
    log::info!("finding entry by id = {}", uid);
    use crate::schema::letters::dsl::*;

    let letter = letters
        .filter(id.eq(uid))
        .first::<models::Letter>(conn)
        .optional()?;

    Ok(letter)
}

pub fn find_by_author(conn: &mut PgConnection, creator: &str) -> Result<Option<models::Letter>> {
    log::info!("finding entry by author = {}", creator);
    use crate::schema::letters::dsl::*;

    let letter = letters
        .filter(author.eq(creator))
        .first::<models::Letter>(conn)
        .optional()?;

    Ok(letter)
}
