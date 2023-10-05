use serde::{Serialize, Deserialize};

#[derive(PartialEq, Serialize, Deserialize)]
pub struct Chart {
    pub id: i32,
    pub title: String,
    pub artist: String,
    pub charter: String,
    pub uploader: i32,
    // todo: implement more on the way
}

#[derive(Serialize, Deserialize)]
struct SpinRequest<T> {
    version: i32,
    status: i32,
    data: T,
}

pub async fn get_chart(id: i32) -> Result<Chart, reqwest::Error> {
    let res = reqwest::get(format!("https://spinsha.re/api/song/{}", id))
        .await?
        .error_for_status()?
        .json::<SpinRequest<Chart>>()
        .await?;
    Ok(res.data)
}
