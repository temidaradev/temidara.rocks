use leptos::prelude::*;
use std::{borrow::Cow, str::FromStr};
use strum::EnumString;
use tailwind_fuse::*;

#[derive(TwVariant, EnumString)]
#[strum(serialize_all = "lowercase")]
pub enum CardVariant {
    #[tw(
        default,
        class = "border border-white/5 bg-surface/50 backdrop-blur-sm hover:border-primary/30 hover:shadow-[0_0_30px_-10px_rgba(245,194,231,0.15)] transition-all duration-500 rounded-xl"
    )]
    Bordered,
    #[tw(class = "glass rounded-2xl")]
    Glass,
}

#[component]
pub fn Card(
    children: Children,
    #[prop(optional, into)] variant: Cow<'static, str>,
    #[prop(optional, into)] class: String,
) -> impl IntoView {
    let variant = CardVariant::from_str(&variant).unwrap_or_default();

    view! {
        <div class=tw_merge!(variant, class)>
            {children()}
        </div>
    }
}

#[component]
pub fn ProjectCard(
    #[prop(into)] title: String,
    #[prop(into)] description: String,
    #[prop(into)] link: String,
    #[prop(into)] link_text: String,
) -> impl IntoView {
    view! {
        <Card class="p-8 h-full flex flex-col justify-between min-h-[200px] group">
            <div class="space-y-4">
                <h3 class="text-2xl font-bold text-primary group-hover:text-secondary transition-colors duration-300">
                    {title}
                </h3>
                <p class="text-foreground/80 leading-relaxed">
                    {description}
                </p>
            </div>
            <div class="pt-6 flex justify-end">
                <a
                    href=link
                    class="text-sm font-medium text-muted hover:text-primary transition-colors duration-300 flex items-center gap-2"
                >
                    {link_text}
                    <i class="fa-solid fa-arrow-right text-xs transition-transform duration-300 group-hover:translate-x-1"></i>
                </a>
            </div>
        </Card>
    }
}

#[component]
pub fn LanguageCard(
    #[prop(into)] name: String,
    #[prop(into)] meta: String,
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] style: String,
) -> impl IntoView {
    view! {
         <div
            class=tw_merge!("border border-primary/20 bg-background/50 px-4 py-3 rounded-lg flex flex-col justify-center hover:border-primary/50 transition-colors duration-300", class)
            style=style
         >
            <span class="font-medium text-foreground">{name}</span>
            <span class="text-xs text-primary/80">{meta}</span>
        </div>
    }
}

#[component]
pub fn StatCard(
    #[prop(into)] value: String,
    #[prop(into)] label: String,
    #[prop(optional, into)] class: String,
) -> impl IntoView {
    view! {
        <div class=tw_merge!("bg-secondary/5 border border-secondary/10 p-4 rounded-xl flex flex-col justify-center items-center text-center hover:bg-secondary/10 hover:border-secondary/20 transition-all duration-300 group", class)>
            <span class="text-3xl font-black text-secondary group-hover:scale-110 transition-transform duration-300">{value}</span>
            <span class="text-[10px] text-muted font-bold uppercase tracking-widest">{label}</span>
        </div>
    }
}

#[component]
pub fn InfoCard(
    #[prop(into)] text: String,
    #[prop(into)] subtext: String,
    #[prop(optional, into)] class: String,
) -> impl IntoView {
    view! {
        <div class=tw_merge!("bg-accent/5 border border-accent/10 p-4 rounded-xl flex flex-col justify-center gap-1 hover:bg-accent/10 hover:border-accent/20 transition-colors duration-300", class)>
            <div class="flex items-center gap-2 text-accent/80">
                <i class="fa-solid fa-location-dot text-xs"></i>
                <span class="text-[10px] uppercase tracking-wider font-bold">"Location"</span>
            </div>
            <span class="text-lg font-bold text-foreground">{text}</span>
            <span class="text-xs text-muted">{subtext}</span>
        </div>
    }
}

#[component]
pub fn PlexCard(
    #[prop(into)] song: String,
    #[prop(into)] artist: String,
    #[prop(optional, into)] class: String,
) -> impl IntoView {
    view! {
        <div class=tw_merge!("relative overflow-hidden bg-[#e5a00d]/5 border border-[#e5a00d]/20 p-5 rounded-xl flex items-center gap-4 hover:border-[#e5a00d]/40 transition-all duration-300 group", class)>
            // Equalizer animation
            <div class="flex gap-[3px] items-end h-8 w-8 pb-1 opacity-80 group-hover:opacity-100 transition-opacity">
                 <div class="w-1.5 bg-[#e5a00d] h-[40%] animate-[bounce_1s_infinite] rounded-t-sm"></div>
                 <div class="w-1.5 bg-[#e5a00d] h-[100%] animate-[bounce_1.2s_infinite] rounded-t-sm"></div>
                 <div class="w-1.5 bg-[#e5a00d] h-[60%] animate-[bounce_0.8s_infinite] rounded-t-sm"></div>
                 <div class="w-1.5 bg-[#e5a00d] h-[80%] animate-[bounce_1.1s_infinite] rounded-t-sm"></div>
            </div>

            <div class="flex flex-col min-w-0 z-10">
                <div class="flex items-center gap-2 mb-0.5">
                     <i class="fa-brands fa-plex text-[#e5a00d] text-xs"></i>
                     <span class="text-[10px] uppercase tracking-wider text-[#e5a00d] font-bold">"Listening Now"</span>
                </div>
                <span class="text-lg font-bold text-foreground truncate w-full group-hover:text-[#e5a00d] transition-colors duration-300">{song}</span>
                <span class="text-xs text-muted truncate w-full group-hover:text-foreground/80 transition-colors duration-300">{artist}</span>
            </div>

            // Background subtle icon
            <i class="fa-brands fa-plex absolute -right-6 -bottom-8 text-[8rem] text-[#e5a00d]/5 rotate-[15deg] group-hover:rotate-[20deg] group-hover:scale-110 transition-all duration-500"></i>
        </div>
    }
}
