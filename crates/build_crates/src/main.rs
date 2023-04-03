use rayon::prelude::*;
use crate::members::{get_members};
use crate::build::build_project;

mod members;
mod build;

fn main() {
    let workspace_members = get_members();

    println!("[Members] {:?}", workspace_members);

    workspace_members
        .par_iter()
        .for_each(|member| build_project(member));
}
