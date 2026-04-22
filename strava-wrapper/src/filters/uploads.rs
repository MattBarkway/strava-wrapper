use crate::models::Upload;
use crate::query::{
    get_with_query_and_path, post_multipart, Endpoint, ErrorWrapper, PathQuery, Query, Sendable, ID,
};
use async_trait::async_trait;
use reqwest::multipart::{Form, Part};
use std::collections::HashMap;
use strava_wrapper_macros::{Endpoint, PathQuery, Query, ID};

/// Strava-accepted upload file formats.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum UploadDataType {
    Fit,
    FitGz,
    Tcx,
    TcxGz,
    Gpx,
    GpxGz,
}

impl UploadDataType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Fit => "fit",
            Self::FitGz => "fit.gz",
            Self::Tcx => "tcx",
            Self::TcxGz => "tcx.gz",
            Self::Gpx => "gpx",
            Self::GpxGz => "gpx.gz",
        }
    }
}

/// Builder for `POST /uploads`. Submits an activity file and returns an
/// [`Upload`] record. The `id` on the response can then be polled via
/// [`GetUpload`] to check processing progress.
#[must_use = "this request is not executed until you call .send().await"]
pub struct UploadActivity {
    url: String,
    token: String,
    file: Vec<u8>,
    data_type: UploadDataType,
    name: Option<String>,
    description: Option<String>,
    trainer: Option<bool>,
    commute: Option<bool>,
    external_id: Option<String>,
}

impl UploadActivity {
    pub fn new(
        url: impl Into<String>,
        token: impl Into<String>,
        file: Vec<u8>,
        data_type: UploadDataType,
    ) -> Self {
        Self {
            url: url.into(),
            token: token.into(),
            file,
            data_type,
            name: None,
            description: None,
            trainer: None,
            commute: None,
            external_id: None,
        }
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn trainer(mut self, trainer: bool) -> Self {
        self.trainer = Some(trainer);
        self
    }

    pub fn commute(mut self, commute: bool) -> Self {
        self.commute = Some(commute);
        self
    }

    pub fn external_id(mut self, id: impl Into<String>) -> Self {
        self.external_id = Some(id.into());
        self
    }
}

#[async_trait]
impl Sendable<Upload> for UploadActivity {
    async fn send(self) -> Result<Upload, ErrorWrapper> {
        let endpoint = format!("{}/v3/uploads", self.url);

        let mut form = Form::new()
            .part(
                "file",
                Part::bytes(self.file).file_name(format!("upload.{}", self.data_type.as_str())),
            )
            .text("data_type", self.data_type.as_str().to_string());

        if let Some(name) = self.name {
            form = form.text("name", name);
        }
        if let Some(description) = self.description {
            form = form.text("description", description);
        }
        if let Some(trainer) = self.trainer {
            form = form.text("trainer", (trainer as u8).to_string());
        }
        if let Some(commute) = self.commute {
            form = form.text("commute", (commute as u8).to_string());
        }
        if let Some(external_id) = self.external_id {
            form = form.text("external_id", external_id);
        }

        post_multipart(&endpoint, &self.token, form).await
    }
}

#[derive(Debug, Clone, Endpoint, Query, PathQuery, ID)]
#[must_use = "this request is not executed until you call .send().await"]
pub struct GetUpload {
    url: String,
    token: String,
    path: String,
    query: Vec<(String, String)>,
    path_params: Vec<(String, String)>,
}

#[async_trait]
impl Sendable<Upload> for GetUpload {
    async fn send(self) -> Result<Upload, ErrorWrapper> {
        let token = self.token.clone();
        get_with_query_and_path(self, &token).await
    }
}
