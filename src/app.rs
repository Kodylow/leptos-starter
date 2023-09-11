use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    provide_meta_context(cx);

    view! {cx,

        <Stylesheet id="leptos" href="/pkg/tailwind.css"/>
        <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico"/>
        <Router>
            <Routes>
                <Route path="" view=  move |_| view! {cx, <Home/> }/>
            </Routes>
        </Router>
    }
}

#[component]
fn Home(cx: Scope) -> impl IntoView {
    let (balance, set_balance) = create_signal(cx, 0);

    view! {cx,
        <Title text="Example"/>
        <main class="bg-white min-h-screen flex flex-col items-center justify-center">
            <h1 class="text-6xl font-bold mb-4">"Example"</h1>
        </main>
    }
}
