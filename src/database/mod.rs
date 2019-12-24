pub mod models;
pub mod schema;

use diesel::dsl::insert_into;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::r2d2;
use failure::Error;

pub type Connection = PgConnection;
pub type ConnectionManager = r2d2::ConnectionManager<Connection>;
pub type Pool = r2d2::Pool<ConnectionManager>;

pub fn find_tidbit(conn: &Connection, tidbit_id: &i32) -> Result<models::Tidbit, Error> {
    use schema::tidbits::dsl::*;

    tidbits
        .filter(id.eq(tidbit_id))
        .get_result(conn)
        .map_err(Error::from)
}

pub fn find_wine(conn: &Connection, wine_id: &i32) -> Result<models::Wine, Error> {
    use schema::wines::dsl::*;

    wines
        .filter(id.eq(wine_id))
        .get_result(conn)
        .map_err(Error::from)
}

pub fn save_order(
    conn: &Connection,
    customer_id: &i32,
    comment: Option<&String>,
) -> Result<models::Order, Error> {
    use crate::database::schema::orders::dsl;

    Ok(insert_into(dsl::orders)
        .values((dsl::customer_id.eq(customer_id), dsl::comment.eq(comment)))
        .get_result::<models::Order>(conn)?)
}

pub fn save_order_item(
    conn: &Connection,
    order_id: &i32,
    name: &String,
    price: &i32,
    tax_rate: i32,
) -> Result<models::OrderItem, Error> {
    use schema::order_items::dsl;

    Ok(insert_into(dsl::order_items)
        .values((
            dsl::order_id.eq(order_id),
            dsl::name.eq(name),
            dsl::price.eq(price),
            dsl::tax_rate.eq(tax_rate),
        ))
        .get_result(conn)?)
}
