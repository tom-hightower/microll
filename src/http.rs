extern crate reqwest;
use crate::structs::State;

pub fn get_text(url: &String, state: &mut State) -> Result<(String, String), reqwest::Error> {
    let body = match reqwest::get(url) {
        Ok(mut resp) => {
            let last_dot: usize = match url.rfind('.') {
                Some(idx) => idx,
                _ => 0,
            };
            let first_slash: usize = match url[last_dot..].find('/') {
                Some(idx) => idx + last_dot,
                _ => url.len(),
            };
            state.root_url = string!(&url[..first_slash]);
            resp.text()?
        }
        Err(e) => {
            if e.to_string() == string!("relative URL without a base") {
                let mut new_url: String = state.root_url.clone();
                new_url.push('/');
                if url.starts_with('/') {
                    new_url.push_str(&url[1..])
                } else {
                    new_url.push_str(url);
                }
                println!("New URL: {}", new_url);
                return get_text(&new_url, state);
            } else {
                println!("Error: {}", e);
                panic!(e)
            }
        }
    };
    Ok((body, url.clone()))
}
