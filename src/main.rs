// mod arr_module;
// mod tupla_module;
// mod if_module;
// mod loop_module;
// mod for_module;
// mod fn_module;
// mod ownership_borrowing_module;
// mod struct_module;
// mod enum_module;
// mod vector_module;
// mod hashmap_module;
// mod errors_module;
// mod custom_errors_module;
// mod mod_module;
// mod my2;

// pub fn my_function() {
//     println!("hello from my local function");
// }

// use mycustomcrate;
// use fltk::{app, prelude::*, window::Window};

use aggregator::{Summary, Tweet, NewsArticle, AveragedCollection};

fn main() {
    // arr_module::arr_module::slice_operations();
    // tupla_module::tupla_module::tupla_operations();
    // if_module::if_module::if_operations();
    // loop_module::rs_module::rs_operations();
    // for_module::for_module::for_function();
    //println!("{}", fn_module::fn_module::simple_function(12));
    // fn_module::fn_module::clouseres();
    // ownership_borrowing_module::ownership_borrowing_module::borrowing();
    // ownership_borrowing_module::ownership_borrowing_module::ownership();
    // ownership_borrowing_module::ownership_borrowing_module::borrowing_mutable();
    // ownership_borrowing_module::ownership_borrowing_module::borrowing_mutable_string();
    // ownership_borrowing_module::ownership_borrowing_module::fn_reference_primitive_type_box();
    // struct_module::struct_1_module::struct_custom3();
    // struct_module::struct_2_module::struct_2_custom();
    // enum_module::enum_module::program();
    // enum_module::enum_module::colorToString();
    // vector_module::vector_module::vector_1();
    // vector_module::vector_module::vector_2();
    // vector_module::vector_module::vector_3_enumerate();
    // vector_module::vector_module::vector_4_iter_mut();
    // hashmap_module::hashmap_module::hasmap_1();
    // hashmap_module::hashmap_module::login();
    // errors_module::errors_module::errors_1();
    // errors_module::errors_module::errors_2();
    // errors_module::errors_module::errors_3();
    // errors_module::errors_module::errors_4();
    // errors_module::errors_module::read_username_from_file();
    // errors_module::errors_module::read_username_from_file_2();
    // custom_errors_module::custom_errors_module::errors_5();
    // mod_module::my_mod::my_function();
    // my_function();
    // {
    //     use mod_module::my_mod::my_function;
    //     my_function();
    // }

    // mod_module::my_mod::call_my_external_function();

    // my2::function();

    // mycustomcrate::hello_function();

    // let app = app::App::default();
    // let mut wind = Window::new(100, 100, 400, 300, "Hello from rust");
    // wind.end();
    // wind.show();
    // app.run().unwrap();


    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know people"),
        reply: false,
        retweet: false
    };

    println!("1 new tweet: {}", tweet.summarize());
    println!("1 author info tweet: {}", tweet.summarize_author());

    let breaking_news = NewsArticle {
        headline: String::from("Noticias de Ejemplo"),
        location: String::from("Polkadot world"),
        author: String::from("El informante"),
        content: String::from("Contenido de Ejemplo")
    };

    aggregator::notify(&breaking_news);

    let breaking_news2 = aggregator::returns_summarizable();
    let news_summary = breaking_news2.summarize();
    println!("News Summary: {}", news_summary);

    let list_average_collection = vec![1];
    let average: f64 = 2.0;
    let mut average_collection_1 = AveragedCollection {
        list: list_average_collection,
        average: average,
    };

    average_collection_1.add(8);
    println!("Average is: {}", average_collection_1.average());
    
    average_collection_1.remove();
    println!("Average is: {}", average_collection_1.average());
    

}