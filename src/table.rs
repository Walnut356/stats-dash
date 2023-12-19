use std::iter::zip;

use dioxus::prelude::*;

use polars::prelude::*;

#[inline_props]
pub fn DFTable(cx: Scope, dataframe: DataFrame) -> Element {
    let height = dataframe.height();
    let column_names = dataframe.get_column_names();
    let columns = dataframe.get_columns();
    let selected = use_state(cx, || usize::MAX);
    let highlighted = "rgba(16, 149, 193, 0.125)";
    cx.render(rsx!(
        table {  role: "grid", margin: 0, padding: 0,
            thead {
                tr {
                    for name in column_names {
                        th {
                            scope: "col",
                            position: "sticky",
                            top: "0",
                            b{
                                font_size: "16px",
                                "{name}"
                            }
                        }
                    }
                }
            }
            tbody {
                for i in 0..height {
                    tr {
                        id: "{i}",
                        background_color: "{(*selected == i).then(|| highlighted).unwrap_or(\"#11191f\")} ",
                        onclick: move |evt| {
                            if *selected == i {
                                selected.set(usize::MAX);
                            } else {
                                selected.set(i);
                            }
                        },
                        for column in columns {
                            td { text_wrap: "nowrap",
                                {
                                    let val = column.get(i).unwrap();
                                    anyval_to_string(&val)
                                }
                            }
                        }
                    }
                }
            }
        }
    ))
}

/// **Recursive**
///
/// Takes a single polars AnyValue and provides a nicely formated result. Recurses through nested
/// types Struct and List to format their interiors as well.
pub fn anyval_to_string(val: &datatypes::AnyValue) -> String {
    match val {
        // truncate to 2 decimal places
        datatypes::AnyValue::Float32(x) => format!("{:.2}", x),
        datatypes::AnyValue::Float64(x) => format!("{:.2}", x),

        // handle nested data
        datatypes::AnyValue::Struct(_, _, c) => {
            let mut temp = "{".to_string();
            let mut buf = Vec::new();
            val._materialize_struct_av(&mut buf);
            for (field, data) in zip(c.iter(), buf.iter()) {
                temp.push_str(&format!("{}: {}, ", field.name, anyval_to_string(data)));
            }
            temp = temp[0..temp.len() - 2].to_string();
            temp.push('}');

            temp
        }
        datatypes::AnyValue::List(x) => {
            if x.len() == 0 {
                "[]".to_string()
            } else {
                let mut temp = "[".to_string();
                for item in x.iter() {
                    temp.push_str(&format!("{}, ", anyval_to_string(&item)))
                }
                temp.pop();
                temp.pop();
                temp.push(']');

                temp
            }
        }

        // it's a little dumb, but this results in printing String instead of "String"
        datatypes::AnyValue::Utf8(x) => x.to_string(),
        datatypes::AnyValue::Null => "N/A".to_string(),
        datatypes::AnyValue::Boolean(x) => match x {
            true => "✓".to_string(),
            false => "X".to_string(),
        }
        // datatypes::AnyValue::Datetime(x, y, z) => {
        //     chrono::DateTime::<chrono::FixedOffset>::from_naive_utc_and_offset(
        //         chrono::naive::NaiveDateTime::from_timestamp_micros(x / 1000).unwrap(),
        //         chrono::offset::FixedOffset::from_str(z.as_ref().unwrap()).unwrap(),
        //     )
        //     .to_string()
        // }
        _ => val.to_string(),
    }
}

fn get_tooltip(header: &str) -> &'static str {
    match header {
        "HitsTaken" => "Total hits taken this game",
        "DIEfficacy" => "Percentage change of the KB angle due to DI where 0.0 is a 0 degree change and 1.0 is an 18 degree change",
        "MostHitBy" => "One or more moves that hit this player the most",
        "StateMostPunished" => "One or more states that the player was hit out of the most",
        "SDIPerHit" => "Average number of SDI inputs per hit they recieved",
        "LivableHitsLived" => "Number of times that a hit could kill, but the player lived via DI",
        "LivableHits" => "Number of times a hit could kill",
        "FrameIndex" => "In-game frame index\n(First frame per game\nengine is -123)",
        "Stocks" => "Remaining stocks on this frame",
        "Percent" => "Player's percent on this frame after any damage is applied",
        "LastHitBy" => "Attack ID of the move that last hit the player (usually on this frame)",
        "StateBeforeHit" => "Action state the player was in on the previous frame",
        "Grounded" => "True if the player was grounded on the previous frame",
        "CrouchCancel" => "True if the player's hitlag frame count suggests that they crouch cancelled\nnull if crouch cancelling was impossible",
        "VCancel" => "True if the player v-cancelled\nnull if a v-cancel was impossible",
        "ASDI" => "Indicates the ASDI direction",
        "HitlagFrames" => "Total number of frames in hitlag",
        "StickDuringHitlag" => "A list containing the stick's position during each frame of hitlag",
        "SDIInputs" => "A list containing each valid SDI input",
        "HitlagStart" => "Player's position upon getting hit",
        "HitlagEnd" => "Estimated player position upon exiting hitlag\n(i.e. with SDI and ASDI position change, but without first frame of KB travel)",
        "DIStick" => "Stick position that was used for DI calculation",
        "Knockback" => "Knockback velocities before DI was applied",
        "DIKnockback" => "Knockback velocities after DI was applied",
        "KBAngle" => "Knockback angle before DI was applied in degrees",
        "DIKBAngle" => "Knockback angle after DI was applied in degrees",
        "KillsWithDI" => "True if the move is projected to kill with the player's DIKnockback value",
        "KillsNoDI" => "True if the move is projected to kill assuming no DI at all",
        "KillsAllDI" => "True if the move is projected to kill regardless of DI angle",
        "KillsSomeDI" => "True if the move is projected to kill some DI angles, but not all possible angles",
        "Waveland" => "True if the player was not in jumpsquat within 5 frames of inputting airdodge",
        "Angle" => "Wavedash angle, measured in degrees below horizontal",
        "Attack" => "Which aerial was LCancelled",
        "LCancelled" => "True if the player successfully LCancelled their aerial",
        "TriggerFrame" => "The most recent trigger input relative to the frame the player landed. Should be 0 to -7 unless InputDuringHitlag is true",
        "FastFall" => "True if the player was fastfalling",
        "Position" => "Which ground the player landed on",
        "InputDuringHitlag" => "True if the player's LCancel input was input during hitlag. Inputs during hitlag extend the valid LCancel window",
        _ => "",
    }
}
