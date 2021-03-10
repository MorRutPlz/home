pub mod set;
pub mod show;

use serenity::framework::standard::macros::group;

use crate::commands::me::{set::*, show::*};
use crate::commands::*;

#[group]
#[prefix = "me"]
#[description = "Group of commands for profile management."]
#[summary = "Commands for profile stuff"]
#[commands(show, set)]
#[checks(Channel)]
struct Me;
