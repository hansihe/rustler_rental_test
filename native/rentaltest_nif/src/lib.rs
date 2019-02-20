#[macro_use] extern crate rustler;
#[macro_use] extern crate rustler_codegen;
#[macro_use] extern crate lazy_static;
#[macro_use]
extern crate rental;

use std::sync::Mutex;

use rustler::{Env, Term, NifResult, Encoder};

mod atoms {
    rustler_atoms! {
        atom ok;
        //atom error;
        //atom __true__ = "true";
        //atom __false__ = "false";
    }
}

rustler_export_nifs! {
    "Elixir.RentalTest.Nif",
    [("add", 2, add)],
    Some(on_load)
}

rental! {
    pub mod testing {
        #[rental]
        pub struct Testing {
            a: String,
            b: &'a str,
        }
    }
}

struct TestResource {
    inner: Mutex<testing::Testing>,
}

pub fn on_load<'a>(env: Env<'a>, _load_info: Term<'a>) -> bool {
    rustler::resource_struct_init!(TestResource, env);
    true
}

fn add<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    let num1: i64 = args[0].decode()?;
    let num2: i64 = args[1].decode()?;

    Ok((atoms::ok(), num1 + num2).encode(env))
}
