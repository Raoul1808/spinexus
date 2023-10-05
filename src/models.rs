use serde::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Chart {
    pub id: i32,
    pub title: String,
    pub subtitle: String,
    pub artist: String,
    pub charter: String,
    pub uploader: Option<i32>,
    pub cover: String,
    // todo: implement more on the way
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub avatar: String,
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

pub async fn get_hot_charts() -> Result<Vec<Chart>, reqwest::Error> {
    let res = reqwest::get("https://spinsha.re/api/songs/hotThisMonth/0")
        .await?
        .error_for_status()?
        .json::<SpinRequest<Vec<Chart>>>()
        .await?;
    Ok(res.data)
}

pub async fn get_user(id: i32) -> Result<User, reqwest::Error> {
    let res = reqwest::get(format!("https://spinsha.re/api/user/{}", id))
        .await?
        .error_for_status()?
        .json::<SpinRequest<User>>()
        .await?;
    Ok(res.data)    
}
