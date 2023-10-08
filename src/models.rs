use serde::{Serialize, Deserialize, de::DeserializeOwned};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct FullChart {
    pub id: i32,
    pub title: String,
    pub subtitle: String,
    pub artist: String,
    pub charter: String,
    pub uploader: i32,
    pub cover: String,
    pub paths: ChartPaths,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ChartPaths {
    pub ogg: String,
    pub cover: String,
    pub zip: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct PartialChart {
    pub id: i32,
    pub title: String,
    pub subtitle: String,
    pub artist: String,
    pub charter: String,
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

async fn request_data<T: DeserializeOwned>(endpoint: String) -> Result<T, reqwest::Error> {
    let res = reqwest::get(endpoint)
        .await?
        .error_for_status()?
        .json::<SpinRequest<T>>()
        .await?;
    Ok(res.data)
}

pub async fn get_chart(id: i32) -> Result<FullChart, reqwest::Error> {
    request_data(format!("https://spinsha.re/api/song/{}", id)).await
}

pub async fn get_new_charts(page: i32) -> Result<Vec<PartialChart>, reqwest::Error> {
    request_data(format!("https://spinsha.re/api/songs/new/{}", page)).await
}

pub async fn get_updated_charts(page: i32) -> Result<Vec<PartialChart>, reqwest::Error> {
    request_data(format!("https://spinsha.re/api/songs/updated/{}", page)).await
}

pub async fn get_weekly_hot_charts(page: i32) -> Result<Vec<PartialChart>, reqwest::Error> {
    request_data(format!("https://spinsha.re/api/songs/hotThisWeek/{}", page)).await
}

pub async fn get_monthly_hot_charts(page: i32) -> Result<Vec<PartialChart>, reqwest::Error> {
    request_data(format!("https://spinsha.re/api/songs/hotThisMonth/{}", page)).await
}

pub async fn get_user(id: i32) -> Result<User, reqwest::Error> {
    request_data(format!("https://spinsha.re/api/user/{}", id)).await
}

pub async fn get_charts_for_user(id: i32) -> Result<Vec<PartialChart>, reqwest::Error> {
    request_data(format!("https://spinsha.re/api/user/{}/charts", id)).await
}
