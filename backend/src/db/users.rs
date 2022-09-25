use crate::models::{self, NewUser};

use anyhow::Result;
use diesel::prelude::*;
use uuid::Uuid;

pub fn insert(conn: &mut PgConnection, new_user: NewUser) -> Result<models::User> {
    use crate::schema::users::dsl::*;
    log::info!("creating new user = {}", new_user.name);

    Ok(diesel::insert_into(users)
        .values(new_user)
        .get_result(conn)?)
}

pub fn find_by_id(conn: &mut PgConnection, uuid: &Uuid) -> Result<Option<models::User>> {
    use crate::schema::users::dsl::*;
    log::info!("getting from id = {:?}", uuid);

    let user = users
        .filter(id.eq(uuid))
        .first::<models::User>(conn)
        .optional()?;

    Ok(user)
}

pub fn find_by_username(
    conn: &mut PgConnection,
    username: impl AsRef<str>,
) -> Result<Option<models::User>> {
    use crate::schema::users::dsl::*;

    let username = username.as_ref();
    log::info!("getting from username = {:?}", username);

    let user = users
        .filter(name.eq(username))
        .first::<models::User>(conn)
        .optional()?;

    Ok(user)
}
