
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
                <div class="left">
                    <a href="https://github.com/thenil3sh">
                        <h3>thenil3sh</h3>
                    </a>
                </div>
                <div class="right">
                    <label class="theme-toggle">
                        <input type="checkbox" on:click=move |_| dark.update(|x| *x = !*x) />
                        <svg
                            viewBox="0 0 960 960"
                            width="20"
                            height="20"
                            xmlns="http://www.w3.org/2000/svg"
                        >
                            <path
                                class:morph-path=move || *dark.read()
                                class:morph-back-path=move || !*dark.read()
                                class="icon-color"
                                fill="#e0def4"
                            />
                        </svg>
                    </label>
                    <a href="https://github.com/thenil3sh/thenil3sh.github.io">
                        <svg
                            viewBox="0 0 20 20"
                            width="20"
                            height="20"
                            xmlns="http://www.w3.org/2000/svg"
                        >
                            <path
                                d="M 10 0 C 15.523 0 20 4.59 20 10.253 C 20 14.782 17.138 18.624 13.167 19.981 C 12.66 20.082 12.48 19.762 12.48 19.489 C 12.48 19.151 12.492 18.047 12.492 16.675 C 12.492 15.719 12.172 15.095 11.813 14.777 C 14.04 14.523 16.38 13.656 16.38 9.718 C 16.38 8.598 15.992 7.684 15.35 6.966 C 15.454 6.707 15.797 5.664 15.252 4.252 C 15.252 4.252 14.414 3.977 12.505 5.303 C 11.706 5.076 10.85 4.962 10 4.958 C 9.15 4.962 8.295 5.076 7.497 5.303 C 5.586 3.977 4.746 4.252 4.746 4.252 C 4.203 5.664 4.546 6.707 4.649 6.966 C 4.01 7.684 3.619 8.598 3.619 9.718 C 3.619 13.646 5.954 14.526 8.175 14.785 C 7.889 15.041 7.63 15.493 7.54 16.156 C 6.97 16.418 5.522 16.871 4.63 15.304 C 4.63 15.304 4.101 14.319 3.097 14.247 C 3.097 14.247 2.122 14.234 3.029 14.87 C 3.029 14.87 3.684 15.185 4.139 16.37 C 4.139 16.37 4.726 18.2 7.508 17.58 C 7.513 18.437 7.522 19.245 7.522 19.489 C 7.522 19.76 7.338 20.077 6.839 19.982 C 2.865 18.627 0 14.783 0 10.253 C 0 4.59 4.478 0 10 0"
                                class="icon-color"
                            />
                        </svg>
                    </a>
                </div>
            </nav>
            <main>
                <div class="container">
                    <WavyText text="under construction..." />
                </div>
            </main>
        </ErrorBoundary>
    }
}

#[component]
fn WavyText (
    #[prop(default = "")]
    text : &'static str
) -> impl IntoView {
    text.char_indices().map(|(index, char)| {
        view! { <span style=format!("--i:{index}")>{char}</span> }
    }).collect_view()
}

