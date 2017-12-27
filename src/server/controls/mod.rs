use tasdcailloux::models::Error;
use diesel::result::Error as DieselError;

pub(self) fn wrap_diesel_error(diesel_error: DieselError) -> Error {
    match diesel_error {
        DieselError::NotFound => Error::ElementNotFound,
        _ => Error::InternalError
    }
}

pub mod mineral;
