
use leptos::prelude::*;

/// Default Home Page
#[component]
pub fn Home() -> impl IntoView {
    let dot_dot_dot = RwSignal::new(".  ");
    let dark = RwSignal::new(true);
    set_interval(
        move || {
            dot_dot_dot.update(|this| {
                *this = match *this {
                    ".  " => ".. ",
                    ".. " => "...",
                    "..." => "   ",
                    "   " => ".  ",
                    _ => panic!("Fucking panicked here")
                };
            });
        },
        std::time::Duration::from_secs(1),
    );
    
    view! {
        <ErrorBoundary fallback=|errors| {
            view! {
                <h1>"Uh oh! Something went wrong!"</h1>

                <p>"Errors: "</p>
                // Render a list of errors as strings - good for development purposes
                <ul>
                    {move || {
                        errors
                            .get()
                            .into_iter()
                            .map(|(_, e)| view! { <li>{e.to_string()}</li> })
                            .collect_view()
                    }}

                </ul>
            }
        }>

            <nav>
                <label class= "theme-toggle">
                    <input type="checkbox" on:click = move |_| dark.update(|x| *x = !*x)/>
                    <svg
                        viewBox="0 0 960 960"
                        width="20"
                        height="20"
                        xmlns="http://www.w3.org/2000/svg"
                    >
                        <path class:morph-path = move || *dark.read() 
                            class:morph-back-path = move || !*dark.read()
                            fill="#e0def4" />
                    </svg>
                </label>
                <button>
                    <img src="public/github.png" />
                </button>
            </nav>
            <main>
            <div class="container">"This guy is onto something" {move || *dot_dot_dot.read()}
            // {move || *dot_dot_dot.read()}
            </div>
            </main>
        </ErrorBoundary>
    }
}


