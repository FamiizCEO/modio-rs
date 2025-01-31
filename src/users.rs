//! Users interface
use url::form_urlencoded;

use crate::prelude::*;

pub use crate::types::{Avatar, User};

/// Interface for users.
pub struct Users {
    modio: Modio,
}

impl Users {
    pub(crate) fn new(modio: Modio) -> Self {
        Self { modio }
    }

    /// List all users registered on [mod.io](https:://mod.io).
    ///
    /// See [Filters and sorting](filters/index.html).
    pub fn list(&self, filter: &Filter) -> Future<List<User>> {
        let mut uri = vec!["/users".into()];
        let query = filter.to_query_string();
        if !query.is_empty() {
            uri.push(query);
        }
        self.modio.get(&uri.join("?"))
    }

    /// Provides a stream over all users registered on [mod.io](https:://mod.io).
    ///
    /// See [Filters and sorting](filters/index.html).
    pub fn iter(&self, filter: &Filter) -> Stream<User> {
        let mut uri = vec!["/users".into()];
        let query = filter.to_query_string();
        if !query.is_empty() {
            uri.push(query);
        }
        self.modio.stream(&uri.join("?"))
    }

    /// Return a user by id
    pub fn get(&self, id: u32) -> Future<User> {
        self.modio.get(&format!("/users/{}", id))
    }

    /// Return the user that is the original submitter of a resource. [required: token]
    pub fn get_owner(&self, resource: Resource) -> Future<User> {
        token_required!(self.modio);
        let params = resource.to_query_string();
        self.modio.post("/general/ownership", params)
    }
}

/// Options used to filter user listings
///
/// # Filter parameters
/// - Fulltext
/// - Id
/// - NameId
/// - DateOnline
/// - Username
/// - Timezone
/// - Language
///
/// # Sorting
/// - Id
/// - Username
///
/// See [modio docs](https://docs.mod.io/#get-all-users) for more information.
///
/// By default this returns up to `100` items. You can limit the result by using `limit` and
/// `offset`.
/// # Example
/// ```
/// use modio::filter::prelude::*;
/// use modio::users::filters::Id;
///
/// let filter = Id::_in(vec![1, 2]).order_by(Id::desc());
/// ```
#[rustfmt::skip]
pub mod filters {
    #[doc(inline)]
    pub use crate::filter::prelude::Fulltext;
    #[doc(inline)]
    pub use crate::filter::prelude::Id;
    #[doc(inline)]
    pub use crate::filter::prelude::NameId;

    filter!(DateOnline, DATE_ONLINE, "date_online", Eq, NotEq, In, Cmp, OrderBy);
    filter!(Username, USERNAME, "username", Eq, NotEq, In, Like, OrderBy);
    filter!(Timezone, TIMEZONE, "timezone", Eq, NotEq, In, Like);
    filter!(Language, LANGUAGE, "language", Eq, NotEq, In, Like);
}

#[derive(Clone, Copy)]
pub enum Resource {
    Game(u32),
    Mod(u32),
    File(u32),
}

impl QueryString for Resource {
    fn to_query_string(&self) -> String {
        let (_type, id) = match *self {
            Resource::Game(id) => ("games", id),
            Resource::Mod(id) => ("mods", id),
            Resource::File(id) => ("files", id),
        };
        form_urlencoded::Serializer::new(String::new())
            .append_pair("resource_type", _type)
            .append_pair("resource_id", &id.to_string())
            .finish()
    }
}
