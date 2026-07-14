use crate::structs::State;

pub fn get_text(url: &str, state: &mut State) -> Result<(String, String), reqwest::Error> {
    // Use the last site root for relative paths
    if reqwest::Url::parse(url).is_err() {
        let mut new_url: String = state.root_url.clone();
        new_url.push('/');
        new_url.push_str(url.strip_prefix('/').unwrap_or(url));
        if new_url != url {
            return get_text(&new_url, state);
        }
    }
    let resp = reqwest::blocking::get(url)?;
    let last_dot: usize = url.rfind('.').unwrap_or(0);
    let first_slash: usize = match url[last_dot..].find('/') {
        Some(idx) => idx + last_dot,
        None => url.len(),
    };
    state.root_url = String::from(&url[..first_slash]);
    Ok((resp.text()?, url.to_string()))
}
