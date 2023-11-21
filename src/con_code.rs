// input {
//                 r#type: "text",
//                 placeholder: "Connect Code (e.g. CODE#123)",
//                 maxlength: 8,
//                 pattern: regex,
//                 oninvalid: |ev| {},
//                 onchange: |ev| {
//                     connect_code.set(ev.value.clone());
//                     if game.read().is_some() {
//                         df.set(
//                             game
//                                 .read()
//                                 .as_ref()
//                                 .unwrap()
//                                 .player_by_code(&connect_code.read())
//                                 .unwrap()
//                                 .stats
//                                 .get(*stat_type.read())
//                                 .unwrap(),
//                         );
//                     }
//                 }
//             }