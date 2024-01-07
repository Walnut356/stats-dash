use std::{
    fs::{self, DirEntry},
    sync::Arc, time::Instant,
};

use dioxus::prelude::*;
use dioxus_router::prelude::*;

use slpprocess::{parse, parse_stubs};
use ssbm_utils::enums::{Character, Costume};

use crate::{
    app::{Filters, Sets, Stubs, WorkingDir},
    components::{Card, DefaultHead, StockIcon},
    utils::{get_sets, parse_entries}, static_str,
};

#[component]
pub fn Browse(cx: Scope) -> Element {
    let working_dir = use_shared_state::<WorkingDir>(cx).unwrap();
    let stubs = use_shared_state::<Stubs>(cx).unwrap();
    let sets = use_shared_state::<Sets>(cx).unwrap();
    let loading = use_state(cx, || false);
    let progress: &UseState<(usize, usize)> = use_state(cx, || (0, 0));

    let filters = use_ref(cx, Filters::default);


    render!(
        DefaultHead {}
        body { class: "dark mask", visibility: "hidden",
            nav { class: "left drawer",
                header { class: "responsive",
                    button {
                        i { "folder_open" }
                        label { "Replay Folder" }
                        input {
                            r#type: "file",
                            directory: true,
                            onchange: |evt| {
                                let fs = evt.files.as_ref().unwrap().files();
                                let file = fs.get(0).map(|x| x.to_owned());
                                if let Some(f) = file {
                                    if working_dir.read().0 != f && !(**loading) {
                                        loading.set(true);
                                        working_dir.write().0 = f.to_string();
                                        cx.spawn({
                                            to_owned!(stubs, loading, progress);
                                            async move {
                                                let mut files: Vec<DirEntry> = fs::read_dir(f).unwrap().filter_map(|file| {
                                                        if let Ok(entry) = file {
                                                            let path = entry.path();
                                                            if path.is_file() && path.extension().unwrap() == "slp" {
                                                                Some(entry)
                                                            } else {
                                                                None
                                                            }
                                                        } else {
                                                            None
                                                        }
                                                    })
                                                    .collect();
                                                // sort files newest to oldest
                                                files.sort_by_key(|a| std::cmp::Reverse(a.metadata().unwrap().created().unwrap()));

                                                stubs.write().0 = Vec::with_capacity(files.len());

                                                progress.set((0, files.len()));


                                                let paths = files.into_iter().map(|x| x.path()).collect::<Vec<_>>();


                                                let mut now = Instant::now();
                                                for (i, chunk) in paths.chunks(25).enumerate() {
                                                    let c = chunk.to_vec();
                                                    let g = tokio::spawn(async {parse_entries(c)}).await;
                                                    progress.set((i * 25, paths.len()));
                                                    // prevent fast parses groups (e.g. lots end up skipped)
                                                    // from causing stubs to be updated too quickly, which locks up
                                                    // the renderer
                                                    if now.elapsed().as_millis() > 100 {
                                                        stubs.write().0.extend(g.unwrap());
                                                        now = Instant::now();
                                                    } else {
                                                        stubs.write_silent().0.extend(g.unwrap());
                                                    }
                                                }

                                                // sets.write().0 = get_sets(&stubs.read().0);

                                                dbg!("Done");

                                                loading.set(false);

                                            }
                                        })
                                    }
                                }
                            }
                        }
                    }
                }

                if !stubs.read().0.is_empty() {
                    render!(
                        div { span { "Filter" } }

                        div { class: "field label border",
                            input {
                                r#type: "text",
                                // dumb workaround to stop the helper text from overlapping input text
                                placeholder: "{filters.read().code.is_empty().then_some(\"\").unwrap_or(\" \")}",
                                oninput: |evt| {
                                    dbg!(evt.value.clone());
                                    filters.write().code = evt.value.clone()},
                                    }
                            label { color: "#999", "CODE#123"}
                        }

                        div { class: "space" }

                        div { class: "field label border",
                            input {
                                r#type: "text",
                                // dumb workaround to stop the helper text from overlapping input text
                                placeholder: "{filters.read().code.is_empty().then_some(\"\").unwrap_or(\" \")}",
                                oninput: |evt| {
                                    dbg!(evt.value.clone());
                                    filters.write().display_name = evt.value.clone()
                                },
                            }
                            label { color: "#999", "Display Name"}
                        }

                        div { class: "space" }
                        // div {
                            // span { "Character"}

                        div { class: "field suffix border",
                            select {
                                oninput: |evt| filters.write().character = Character::try_from(evt.value.as_str()).ok(),
                                // ordered by 2021 tier list cuz it's vaguely similar to usage chart
                                // and is mostly intuitive. Alternative is alphabeticle, which is
                                // VERY different from the usage chart albeit more intuitive.
                                option { "Character" }
                                option { static_str!(Character::Fox) }
                                option { static_str!(Character::Marth) }
                                option { static_str!(Character::Jigglypuff) }
                                option { static_str!(Character::Falco) }
                                option { static_str!(Character::Sheik) }
                                option { static_str!(Character::CaptainFalcon) }
                                option { static_str!(Character::Peach) }
                                option { static_str!(Character::IceClimbers) }
                                option { static_str!(Character::Pikachu) }
                                option { static_str!(Character::Yoshi) }
                                option { static_str!(Character::Samus) }
                                option { static_str!(Character::Luigi) }
                                option { static_str!(Character::DrMario) }
                                option { static_str!(Character::Ganondorf) }
                                option { static_str!(Character::Mario) }
                                option { static_str!(Character::DonkeyKong) }
                                option { static_str!(Character::YoungLink) }
                                option { static_str!(Character::Link) }
                                option { static_str!(Character::GameAndWatch) }
                                option { static_str!(Character::Mewtwo) }
                                option { static_str!(Character::Roy) }
                                option { static_str!(Character::Pichu) }
                                option { static_str!(Character::Ness) }
                                option { static_str!(Character::Zelda) }
                                option { static_str!(Character::Kirby) }
                                option { static_str!(Character::Bowser) }
                            }
                            i { "arrow_drop_down" }
                        }
                        // }

                        div { class: "max" }
                    )
                }
            }
            main { class: "responsive",
                if **loading {
                    rsx!{
                        progress {
                            value: "{progress.0}",
                            max: "{progress.1}",
                        }
                        label {
                            "{progress.0} of {progress.1}"
                        }
                    }
                }
                for set in get_sets(&stubs.read().0).into_iter() {
                    rsx!(
                        Card { set: set, filters: filters.read().clone() }

                    )
                }
            }
        }
    )
}
