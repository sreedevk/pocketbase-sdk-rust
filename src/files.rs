use crate::client::{Auth, Client};
use crate::httpc::Httpc;
use anyhow::{anyhow, Result};
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
struct FileTokenResponse {
    token: String,
}

pub struct FilesManager<'a, State> {
    pub client: &'a Client<State>,
}

impl<'a, State> FilesManager<'a, State> {
    pub fn url(&self, collection: &str, record_id: &str, filename: &str) -> String {
        format!(
            "{}/api/files/{}/{}/{}",
            self.client.base_url, collection, record_id, filename
        )
    }

    pub fn download(&self, collection: &'a str, record_id: &'a str, filename: &'a str) -> FileDownloadBuilder<'a, State> {
        FileDownloadBuilder {
            client: self.client,
            collection,
            record_id,
            filename,
            thumb: None,
            token: None,
            download: false,
        }
    }
}

impl<'a> FilesManager<'a, Auth> {
    pub fn token(&self) -> Result<String> {
        let url = format!("{}/api/files/token", self.client.base_url);
        match Httpc::post(self.client, &url, "{}".to_string()) {
            Ok(response) => Ok(response.into_json::<FileTokenResponse>()?.token),
            Err(e) => Err(anyhow!("{}", e)),
        }
    }
}

pub struct FileDownloadBuilder<'a, State> {
    pub client: &'a Client<State>,
    collection: &'a str,
    record_id: &'a str,
    filename: &'a str,
    thumb: Option<String>,
    token: Option<String>,
    download: bool,
}

impl<'a, State> FileDownloadBuilder<'a, State> {
    pub fn thumb(mut self, thumb: &str) -> Self {
        self.thumb = Some(thumb.to_string());
        self
    }

    pub fn token(mut self, token: &str) -> Self {
        self.token = Some(token.to_string());
        self
    }

    pub fn download(mut self, download: bool) -> Self {
        self.download = download;
        self
    }

    pub fn call(&self) -> Result<Vec<u8>> {
        let url = format!(
            "{}/api/files/{}/{}/{}",
            self.client.base_url, self.collection, self.record_id, self.filename
        );
        let mut build_opts: Vec<(&str, &str)> = vec![];
        if let Some(thumb) = &self.thumb {
            build_opts.push(("thumb", thumb))
        }
        if let Some(token) = &self.token {
            build_opts.push(("token", token))
        }
        if self.download {
            build_opts.push(("download", "true"))
        }
        let query = if build_opts.is_empty() { None } else { Some(build_opts) };
        match Httpc::get(self.client, &url, query) {
            Ok(response) => Httpc::read_bytes(response),
            Err(e) => Err(anyhow!("{}", e)),
        }
    }
}
