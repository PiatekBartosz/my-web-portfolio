use crate::pages::home::*;
use crate::pages::about::*;
use crate::components::footer::Footer;
use crate::components::topnav::TopNav;

use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Html lang="en"/>
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/leptos_start.css"/>

        // sets the document title
        <Title text="My Web Portfolio"/>

        // content for this welcome page
        <Router>
            <Routes>
                <Route path="" view=Home/>
                <Route path="/about" view=About/>
                <Route path="/*any" view=NotFound/>
            </Routes>
        </Router>
    }
}

// / Renders the home page of your application.
// #[component]
// fn HomePage() -> impl IntoView {
//     // Creates a reactive value to update the button
//     let (count, set_count) = create_signal(0);
//     let on_click = move |_| set_count.update(|count| *count += 1);

//     view! {
//         <h1>"Welcome to Leptos!"</h1>
//         <button on:click=on_click>"Click Me: " {count}</button>
//     }
// }

/// 404 - Not Found
#[component]
fn NotFound() -> impl IntoView {
    // set an HTTP status code 404
    // this is feature gated because it can only be done during
    // initial server-side rendering
    // if you navigate to the 404 page subsequently, the status
    // code will not be set because there is not a new HTTP request
    // to the server
    #[cfg(feature = "ssr")]
    {
        // this can be done inline because it's synchronous
        // if it were async, we'd use a server function
        let resp = expect_context::<leptos_actix::ResponseOptions>();
        resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
    }

    view! {
    <head>
        // <meta charset="UTF-8">
        <title>Your Page Title</title>
    </head>
    
    <body class="bg-gray-900 flex flex-col min-h-screen">
        <TopNav/>
    
        <section class="container pt-24 md:pt-36 mx-auto flex flex-wrap flex-col md:flex-row items-center">
            <p class="text-black">"404 - Not Found"</p>
        </section>
    
        <Footer/>
    </body>
    }
}