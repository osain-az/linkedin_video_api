use crate::utils::{InitVideoResponse, UploadInstructions, VideoUploadStatus};

use crate::handle_file::extract_file_by_position;
use crate::request_handler::{upload_chunk_as_bytes, video_request, video_request_final};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::prelude::*;

#[derive(Deserialize, Serialize, Clone)]
pub struct VideoApi {
    base_url: String,
    person_id: String,
    access_token: String,
}

impl VideoApi {
    pub fn new(base_url: String, person_id: String, access_token: String) -> VideoApi {
        VideoApi {
            base_url,
            person_id,
            access_token,
        }
    }

    pub async fn init_video_upload(
        &self,
        init_params: InitVideoParams,
    ) -> Result<InitVideoResponse, String> {
        let base_url = self.base_url.to_owned().replace("videos/", "videos?");
        let url = base_url + "action=initializeUpload";

        let resp = video_request(
            url.to_string(),
            serde_json::to_value(&init_params).unwrap(),
            self.access_token.clone(),
        )
        .await;
        if resp.is_ok() {
            println!("video_id :{}", resp.as_ref().unwrap().value.video);
            Ok(resp.unwrap())
        } else {
            println!("result: {:?}", resp.as_ref().err().unwrap());

            Err(resp.err().unwrap())
        }
    }

    pub async fn upload_media(
        self,
        upload_url: String,
        buffer_file: Vec<u8>,
    ) -> Result<String, String> {
        let token = self.access_token.clone();
        let resp = upload_chunk_as_bytes(upload_url, buffer_file, token, "PUT").await;
        if resp.is_ok() {
            Ok(resp.unwrap())
        } else {
            Err(resp.err().unwrap())
        }
    }

    pub async fn confirm_final_upload(
        self,
        uploading_data: FinalUploadData,
    ) -> Result<String, String> {
        let base_url = self.base_url.replace("videos/", "videos?");
        let url = base_url + "action=finalizeUpload";
        let token = self.access_token.clone();
        let resp =
            video_request_final(url, serde_json::to_value(&uploading_data).unwrap(), token).await;
        if resp.is_ok() {
            Ok(resp.unwrap())
        } else {
            Err("error".to_string())
        }
    }

    pub async fn video_upload_handeler(
        &self,
        video: File,
        video_caption: Option<File>,
        video_thumbnail: Option<File>,
        purpose: String,
    ) -> Result<String, String> {
        let token = self.clone().access_token.clone();
        let url = self.clone().base_url.clone();

        let mut etag_list: Vec<String> = Vec::new();
        let single_upload_size = 4194303; // 4mb

        let video_file = video.try_clone().unwrap_or(video);
        let file_size = video_file.metadata().unwrap().len();

        let video_thumb_nail = if video_thumbnail.is_some() {
            true
        } else {
            false
        };

        let _video_caption = if video_caption.is_some() { true } else { false };

        let video_init_params = InitVideoParams {
            initializeUploadRequest: InitializeUploadRequest {
                owner: self.person_id.clone(),
                purpose,
                fileSizeBytes: file_size,
                uploadCaptions: _video_caption,
                uploadThumbnail: video_thumb_nail,
            },
        };

        //Initialize request
        let init_resp = self.clone().init_video_upload(video_init_params).await; // send init request
        let mut init_video_params: InitVideoResponse;

        match init_resp {
            Ok(init_data) => init_video_params = init_data,
            Err(err) => {
                println!("err: {:?}", err);
                return Err(err.to_string());
            }
        };

        let uploading_list = init_video_params.value.uploadInstructions.clone();
        let upload_video_id = init_video_params.value.video.clone();

        if file_size < single_upload_size {
            // if the video is less than 4mb

            let mut buffer = vec![];
            let mut file = video_file.try_clone().unwrap();
            file.read_to_end(&mut buffer);

            let upload_url = uploading_list[0].uploadUrl.clone();

            //Upload video
            let video_resp = self
                .clone()
                .upload_media(upload_url.to_string(), buffer)
                .await;

            match video_resp {
                Ok(resp) => {
                    etag_list.push(resp);
                }
                Err(er) => return Err("err".to_string()),
            };

            //get response header
        } else {
            for upload_data in uploading_list.iter() {
                let upload_url = upload_data.uploadUrl.clone();
                let end = upload_data.lastByte.clone();
                let start = upload_data.firstByte.clone();

                let chunk_data =
                    extract_file_by_position(video_file.try_clone().unwrap(), end, start);
                println!("size : {:?}", chunk_data.len());

                let etag_resp = self.clone().upload_media(upload_url, chunk_data).await;

                match etag_resp {
                    Ok(etag) => {
                        //  println!("size : {:?}", etag);
                        etag_list.push(etag);
                    }
                    Err(er) => return Err("err".to_string()),
                };
            }
        }
        //confirm and finalize upload
        let confirm_data =
            UploadConfirmationRequest::new(upload_video_id.clone(), "".to_owned(), etag_list);

        let data = FinalUploadData::new(confirm_data);

        let final_resp = self.clone().confirm_final_upload(data).await;

        return match final_resp {
            Ok(resp) => Ok(resp),
            Err(err) => Err("".to_string()),
        };
    }
}

#[derive(Serialize, Debug)]
pub struct FinalUploadData {
    pub finalizeUploadRequest: UploadConfirmationRequest,
}

impl FinalUploadData {
    pub fn new(finalizeUploadRequest: UploadConfirmationRequest) -> Self {
        FinalUploadData {
            finalizeUploadRequest,
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct UploadConfirmationRequest {
    video: String,
    uploadToken: String,
    uploadedPartIds: Vec<String>,
}

impl UploadConfirmationRequest {
    pub fn new(video: String, uploadToken: String, uploadedPartIds: Vec<String>) -> Self {
        UploadConfirmationRequest {
            video,
            uploadToken,
            uploadedPartIds,
        }
    }
    pub fn set_video(&mut self, video: String) {
        self.video = video;
    }
    pub fn set_uploadToken(&mut self, uploadToken: String) {
        self.uploadToken = uploadToken;
    }
    pub fn set_uploadedPartIds(&mut self, uploadedPartIds: Vec<String>) {
        self.uploadedPartIds = uploadedPartIds;
    }
}

#[derive(Serialize)]
pub struct InitializeUploadRequest {
    pub owner: String,
    pub purpose: String,
    pub fileSizeBytes: u64,
    pub uploadCaptions: bool,
    pub uploadThumbnail: bool,
}

#[derive(Serialize)]
pub struct InitVideoParams {
    pub initializeUploadRequest: InitializeUploadRequest,
}
