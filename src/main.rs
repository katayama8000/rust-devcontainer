mod anyhow;
mod api_mock;
mod arc;
mod as_ref;
mod box_heap;
mod box_list;
mod btree;
mod closure;
mod custom_error;
mod custom_error2;
mod default;
mod deref;
mod display;
mod dyn_trait;
mod flatten;
mod from;
mod generics;
mod into;
mod lifetime;
mod macro_echo_num;
mod map;
mod matches;
mod misc;
mod mock_all;
mod mutex;
mod ok_or_else;
mod owned;
mod rc_arc;
mod ref_cell;
mod reference_counter;
mod rw_lock;
mod serde;
mod stack_heap;
mod str;
mod thread;
mod trait_obect;
mod traits;
mod transpose;
mod r#type;

#[tokio::main]
async fn main() {
    // generics::run();
    // lifetime::run();
    // stack_heap::run();
    // box_heap::run();
    // dyn_trait::run();
    // traits::run();
    // trait_obect::run();
    // serde::run();
    // btree::run();
    // from::run();
    // as_ref::run();
    // map::run();
    // misc::run();
    // thread::run();
    // flatten::run();
    // generics::run();
    // reference_counter::run();
    // str::run();
    // macro_echo_num::run();
    // box_list::run();
    // transpose::run();
    // ok_or_else::run();
    // rw_lock::run();
    // rc_arc::run();
    // ref_cell::run();
    // arc::run();
    // mutex::run();
    // closure::run();
    // anyhow::run();
    // display::run();
    // custom_error::run();
    // matches::run();
    // into::run();
    // owned::run();
    // custom_error2::run();
    // deref::run();
    // r#type::run();
    // api_mock::run().await;
    // default::run();
    mock_all::run();
}
