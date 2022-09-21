use diesel::prelude::*;

#[derive(Queryable)]
pub struct User<'a> {
  pub token: String,
  pub salt: &'a[char; 10],
  pub balance: f64,
}
