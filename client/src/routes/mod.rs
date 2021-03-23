pub mod about;
pub mod blog;
pub mod home;
pub mod film;
pub mod tech;

use yew_router::prelude::*;

#[derive(Switch, Debug, Clone)]
pub enum AppRoute {
    #[to="/about"]
    About,
    #[to="/blog/{id}"]
    BlogPage(String),
    #[to="/blog"]
    Blog,
    #[to="/film"]
    Film,
    #[to="/tech"]
    Tech,
    #[to="/"]
    Home
}