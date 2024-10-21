use std::path::PathBuf;
use crate::turing::def::TuringDef;

pub fn def_to_strict_format(def: &TuringDef) -> String {
    format!(
        "\
        {}\n\
        {}\n\
        {}\n\
        {}\n\
        {}\n\
        {}\
        ",
        def.state_count,
        def.input_alphabet.iter().collect::<String>(),
        def.tape_alphabet.iter().collect::<String>(),
        def.start_state,
        def.end_state,
        def.transition_function.iter()
            .filter_map(|tf| {
                if tf.fail_state {
                     None
                } else{
                    Some(format!(
                        "{} {} {} {} {}\n",
                        tf.state, tf.input, tf.next_state, tf.write, tf.move_dir
                    ))
                }
            })
            .collect::<String>()
    )
}

pub fn compute_strict_path(tm_def: &PathBuf) -> PathBuf {
    let mut new_path = tm_def.clone();
    if let Some(file_stem) = tm_def.file_stem() {
        if let Some(extension) = tm_def.extension() {
            if extension == "TM" {
                let new_file_name = format!("{}_strict.{}", file_stem.to_string_lossy(), extension.to_string_lossy());
                new_path.set_file_name(new_file_name);
            }
        }
    }

    new_path
}

