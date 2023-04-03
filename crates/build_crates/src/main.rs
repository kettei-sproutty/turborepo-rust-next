use crate::build::build_project;
use crate::members::get_members;
use rayon::prelude::*;

mod build;
mod members;

fn main() {
    let workspace_members = get_members();

    println!("[Members] {:?}", workspace_members);

    workspace_members.par_iter().for_each(build_project);
}
