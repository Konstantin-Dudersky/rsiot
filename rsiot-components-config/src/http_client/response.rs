use http::StatusCode;

pub struct Response {
    pub status_code: StatusCode,
    pub text: String,
}
