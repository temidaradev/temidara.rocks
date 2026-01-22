use std::{borrow::Cow, str::FromStr};

use leptos::prelude::*;
use strum::EnumString;
use tailwind_fuse::*;

#[derive(TwVariant, EnumString)]
#[strum(serialize_all = "lowercase")]
pub enum Size {
    #[tw(default, class = "h-9 px-4 py-2")]
    Md,
    #[tw(class = "h-8 px-3")]
    Sm,
    #[tw(class = "h-10 px-8")]
    Lg,
}

#[derive(TwVariant, EnumString)]
#[strum(serialize_all = "lowercase")]
pub enum Variant {
    #[tw(default, class = "bg-primary text-background font-bold shadow-[0_0_20px_-5px_rgba(245,194,231,0.4)] hover:shadow-[0_0_25px_-5px_rgba(245,194,231,0.6)] hover:scale-105 active:scale-95 transition-all duration-300 rounded-xl")]
    Primary,
    #[tw(class = "bg-secondary text-background font-bold shadow-[0_0_20px_-5px_rgba(137,180,250,0.4)] hover:shadow-[0_0_25px_-5px_rgba(137,180,250,0.6)] hover:scale-105 active:scale-95 transition-all duration-300 rounded-xl")]
    Secondary,
    #[tw(class = "border-2 border-primary/50 text-primary hover:border-primary hover:bg-primary/10 hover:scale-105 active:scale-95 transition-all duration-300 rounded-xl")]
    Outline,
    #[tw(class = "text-muted hover:text-primary hover:bg-white/5 transition-all duration-300 rounded-lg")]
    Ghost,
}

#[component]
pub fn Button(
    #[prop(into)] text: Cow<'static, str>,
    #[prop(into, optional)] variant: Cow<'static, str>,
    #[prop(into, optional)] size: Cow<'static, str>,
    #[prop(optional, into)] on_click: Option<Box<dyn Fn(web_sys::MouseEvent) + Send + Sync>>, // Basic click handler support
) -> impl IntoView {
    let variant = Variant::from_str(&variant).unwrap_or_default();
    let size = Size::from_str(&size).unwrap_or_default();

    view! {
        <button 
            class=tw_merge!(variant, size)
            on:click=move |e| {
                if let Some(ref cb) = on_click {
                    cb(e);
                }
            }
        >
            {text}
        </button>
    }
}
