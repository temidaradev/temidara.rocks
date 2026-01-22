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
        <Card class="p-4 flex flex-col justify-between group min-h-[140px]">
            <div class="space-y-2">
                <h3 class="text-base font-bold text-primary group-hover:text-secondary transition-colors duration-300">
                    {title}
                </h3>
                <p class="text-foreground/80 text-xs leading-relaxed line-clamp-3">
                    {description}
                </p>
            </div>
            <div class="flex justify-end">
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
pub fn PlexCard(
    #[prop(into)] track: Option<crate::api::plex::TrackInfo>,
    #[prop(optional, into)] class: String,
) -> impl IntoView {
    let status = track.as_ref().map(|t| t.status.clone()).unwrap_or_else(|| "stopped".to_string());
    let thumb_url = track.as_ref().and_then(|t| t.thumb_url.clone());
    let is_playing = status == "playing";

    let anim_class = if is_playing { "animate-equalizer" } else { "h-[10%]" };

    view! {
        <div class=tw_merge!("relative overflow-hidden bg-background/80 backdrop-blur-md border border-[#e5a00d]/20 p-4 rounded-xl flex items-center gap-4 hover:border-[#e5a00d]/40 transition-all duration-300 group min-h-[140px]", class)>
            <div class="relative flex-shrink-0">
                {move || match &thumb_url {
                    Some(url) => view! {
                        <div class="w-24 h-24 rounded-lg overflow-hidden shadow-lg group-hover:shadow-[#e5a00d]/20 transition-shadow duration-300">
                            <img 
                                src=url.clone() 
                                alt="Album Art" 
                                class="w-full h-full object-cover"
                            />
                        </div>
                    }.into_any(),
                    None => view! {
                        <div class="w-24 h-24 rounded-lg bg-[#e5a00d]/10 flex items-center justify-center border border-[#e5a00d]/20">
                            <i class="fa-solid fa-music text-[#e5a00d]/50 text-2xl"></i>
                        </div>
                    }.into_any()
                }}
            </div>

            <div class="flex flex-col min-w-0 z-10 flex-1 justify-center">
                <div class="flex items-center gap-1.5 mb-2">
                     <span class="text-[10px] uppercase tracking-wider text-[#e5a00d] font-bold">
                        {move || match status.as_str() {
                            "playing" => "Listening Now",
                            "paused" => "Paused",
                            _ => "Offline"
                        }}
                     </span>
                     {move || if is_playing {
                         view! {
                             <div class="flex gap-[2px] items-end h-3 ml-1">
                                 <div class=format!("w-0.5 bg-[#e5a00d] rounded-t-sm {}", anim_class) style="animation-delay: -0.15s"></div>
                                 <div class=format!("w-0.5 bg-[#e5a00d] rounded-t-sm {}", anim_class) style="animation-delay: -0.35s"></div>
                                 <div class=format!("w-0.5 bg-[#e5a00d] rounded-t-sm {}", anim_class) style="animation-delay: -0.55s"></div>
                             </div>
                         }.into_any()
                     } else {
                         view! { <></> }.into_any()
                     }}
                </div>
                
                {move || match &track {
                    Some(t) => view! {
                        <>
                            <span class="text-lg font-bold text-foreground truncate w-full group-hover:text-[#e5a00d] transition-colors duration-300">
                                {t.title.clone()}
                            </span>
                            <span class="text-sm text-muted truncate w-full group-hover:text-foreground/80 transition-colors duration-300">
                                {format!("{} • {}", t.artist.clone(), t.album.clone())}
                            </span>
                        </>
                    }.into_any(),
                    None => view! {
                        <span class="text-base font-medium text-muted/50 italic">
                            "Silence is golden..."
                        </span>
                    }.into_any()
                }}
            </div>

            <i class="fa-brands fa-plex absolute -right-6 -bottom-8 text-[8rem] text-[#e5a00d]/5 rotate-[15deg] group-hover:rotate-[20deg] group-hover:scale-110 transition-all duration-500"></i>
        </div>
    }
}
