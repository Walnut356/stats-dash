use std::iter::zip;
use std::str::FromStr;

use dioxus_router::prelude::*;

use dioxus::prelude::*;
use log::LevelFilter;

use polars::prelude::*;
use slpprocess::{parse, stats::StatType};

#[inline_props]
pub fn DFTable(cx: Scope, dataframe: DataFrame) -> Element {
    let height = dataframe.height();
    let column_names = dataframe.get_column_names();
    let columns = dataframe.get_columns();
    render!(
        table { padding: "10px", role: "grid",
            tr {
                for name in column_names {
                    th { "{name}" }
                }
            }
            for i in 0..height {
                tr {
                    for column in columns {
                        td { text_wrap: "nowrap",
                            {
                                let val = column.get(i).unwrap();
                                anyval_to_string(&val)
                                // match val {
                                //     datatypes::AnyValue::Struct(_, _, c) => {
                                //         let mut temp = "{".to_string();
                                //         let mut buf = Vec::new();
                                //         val._materialize_struct_av(&mut buf);
                                //         for (field, data) in zip(c.iter(), buf.iter()) {
                                //             temp.push_str(&format!("{}: {}, ", field.name, anyval_to_string(data)));
                                //         }
                                //         temp = temp[0..temp.len() - 2].to_string();
                                //         temp.push('}');

                                //         temp
                                //     },
                                //     datatypes::AnyValue::Utf8(x) => x.to_string(),
                                //     _ => val.to_string(),
                                // }
                            }
                        }
                    }
                }
            }
        }
    )
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
                match data {
                    datatypes::AnyValue::Float32(x) => {
                        temp.push_str(&format!("{}: {:.2}, ", field.name, x))
                    }
                    datatypes::AnyValue::Float64(x) => {
                        temp.push_str(&format!("{}: {:.2}, ", field.name, x))
                    }
                    _ => temp.push_str(&format!("{}: {}, ", field.name, anyval_to_string(data))),
                }
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

        // it's a little dumb, but this results in printing 'String' instead of '"String"'
        datatypes::AnyValue::Utf8(x) => x.to_string(),

        _ => val.to_string(),
    }
}
