mod anyhow;
mod api_mock;
mod arc;
mod as_ref;
mod box_heap;
mod box_list;
mod btree;
mod cafe_scraper;
mod closure;
mod custom_error;
mod custom_error2;
mod date;
mod default;
mod deref;
mod display;
mod dyn_trait;
mod expo;
mod feature;
mod flatten;
mod from;
mod generics;
mod gyazo;
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
mod pattern_binding;
mod rc_arc;
mod ref_cell;
mod reference_counter;
mod rw_lock;
mod scrapbox;
mod serde;
mod stack_heap;
mod str;
mod supabase;
mod this_error;
mod thread;
mod trait_obect;
mod traits;
mod transpose;
mod try_json;
mod r#type;

#[tokio::main]
async fn main() {
    // anyhow::run();
    // api_mock::run().await;
    // arc::run();
    // as_ref::run();
    // box_heap::run();
    // box_list::run();
    // btree::run();
    // closure::run();
    // custom_error::run();
    // custom_error2::run();
    // date::run();
    // default::run();
    // deref::run();
    // display::run();
    // dyn_trait::run();
    // feature::run();
    // flatten::run();
    // from::run();
    // generics::run();
    // generics::run();
    // gyazo::run().await;
    // into::run();
    // lifetime::run();
    // macro_echo_num::run();
    // map::run();
    // matches::run();
    // misc::run();
    // mock_all::run();
    // mutex::run();
    // ok_or_else::run();
    // owned::run();
    // r#type::run();
    // rc_arc::run();
    // ref_cell::run();
    // reference_counter::run();
    // rw_lock::run();
    // scrapbox::run();
    // serde::run();
    // stack_heap::run();
    // str::run();
    // supabase::run().await;
    // this_error::run();
    // thread::run();
    // trait_obect::run();
    // traits::run();
    // transpose::run();
    // try_json::run();
    // expo::run().await;
    // cafe_scraper::run().await.unwrap();
    pattern_binding::run();
}
