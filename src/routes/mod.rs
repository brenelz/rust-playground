mod index;
pub use self::index::get_index;

mod form_data;
pub use self::form_data::post_form_data;

mod products_fromdb;
pub use self::products_fromdb::get_products_fromdb;

mod products_insert;
pub use self::products_insert::get_products_insert;

mod products;
pub use self::products::get_products;
