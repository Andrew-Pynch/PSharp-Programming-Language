use std::fs;

pub fn parse_pspl_file(file_to_parse: String) -> String {
    // TODO: add args to specify file name

    let contents = fs::read_to_string(file_to_parse)
        .expect("Something went wrong while trying to read the file");

    return contents;
}
