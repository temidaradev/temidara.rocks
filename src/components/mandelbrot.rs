use leptos::prelude::*;

#[component]
pub fn MandelbrotViewer() -> impl IntoView {
    #[cfg(feature = "hydrate")]
    {
        use wasm_bindgen::prelude::*;
        use wasm_bindgen::JsCast;

        Effect::new(move |_| {
            let window = web_sys::window().expect("no global window");
            let document = window.document().expect("no document");

            let script_exists = document
                .query_selector("script[src='/mq_js_bundle.js']")
                .ok()
                .flatten()
                .is_some();

            if !script_exists {
                let script = document.create_element("script").unwrap();
                let script: web_sys::HtmlScriptElement = script.unchecked_into();
                script.set_src("/mq_js_bundle.js");
                script.set_type("text/javascript");

                let onload = Closure::wrap(Box::new(move || {
                    let _ = web_sys::js_sys::eval(
                        r#"
                        if (typeof load === 'function' && !window.mandelbrotLoaded) {
                            window.mandelbrotLoaded = true;
                            setTimeout(() => load('/mandelbrot-rust.wasm'), 100);
                        }
                        "#,
                    );
                }) as Box<dyn Fn()>);

                script.set_onload(Some(onload.as_ref().unchecked_ref()));
                onload.forget();

                document.head().unwrap().append_child(&script).unwrap();
            } else {
                let _ = web_sys::js_sys::eval(
                    r#"
                    if (typeof load === 'function' && !window.mandelbrotLoaded) {
                        window.mandelbrotLoaded = true;
                        setTimeout(() => load('/mandelbrot-rust.wasm'), 100);
                    }
                    "#,
                );
            }
        });
    }

    view! {
        <div class="mandelbrot-container my-8">
            <div class="mb-4">
                <h3 class="text-lg font-bold text-white mb-2">"Interactive Mandelbrot Set Viewer"</h3>
                <p class="text-sm text-gray-400 mb-4">
                    "Explore the Mandelbrot set rendered in real-time with Rust and WebAssembly. "
                    "Click and drag to zoom into the fractal, right-click to reset."
                </p>
            </div>

            <div class="relative mx-auto bg-black rounded border border-white/20 overflow-hidden" style="width: 800px; height: 600px;">
                <canvas
                    id="glcanvas"
                    tabindex="1"
                    class="rounded"
                    style="width: 800px; height: 600px; display: block; margin: 0; padding: 0;"
                    width="800"
                    height="600"
                >
                </canvas>
            </div>

            <div class="mt-4 text-center">
                <p class="text-xs text-gray-500 font-mono">
                    "Powered by Rust + WebAssembly + miniquad"
                </p>
            </div>
        </div>
    }
}
