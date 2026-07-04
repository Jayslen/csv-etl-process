use std::io::Error;

pub fn string_to_vec(value: Result<String, Error>) -> Result<Vec<String>, Error> {
    let v = value?;
    let list = v
        .split(',')
        .map(|s| s.to_string().replace("\n", ""))
        .collect();
    Ok(list)
}
