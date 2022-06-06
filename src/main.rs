mod prelude;
mod vm;

use crate::prelude::*;
use crate::vm::{Pattern, Variable};

fn main() {
    let pattern = {
        let entity: Variable = ("?e").into();
        let attribute: Variable = ("user/name", "alex").into();
        let mut p = Pattern::default();
        p.entity = Some(entity);
        p.attribute = Some(attribute);
        p
    };
    println!("{:?}", pattern);
}
