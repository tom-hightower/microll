extern crate reqwest;

pub fn get_text(url: &String) -> Result<String, reqwest::Error> {
    let body = reqwest::get(url)?.text()?;
    Ok(body)
}
