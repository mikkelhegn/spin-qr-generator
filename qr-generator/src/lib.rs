use anyhow::Result;
use bytes::Bytes;
use http::StatusCode;
use percent_encoding::percent_decode;
use qrcode_generator::QrCodeEcc;
use spin_sdk::{
    http::{Request, Response},
    http_component,
};

#[http_component]
fn genereta_qr_code(req: Request) -> Result<Response> {
    let (status, body) = match req.uri().query() {
        Some(p) => {
            let decoded_query = percent_decode(p.as_bytes()).decode_utf8();
            match qrcode_generator::to_svg_to_string(
                decoded_query.unwrap().as_ref(),
                QrCodeEcc::Medium,
                512,
                None::<String>,
            ) {
                Ok(q) => (StatusCode::OK, Some(Bytes::from(q))),
                _ => (StatusCode::INTERNAL_SERVER_ERROR, None),
            }
        }
        _ => (StatusCode::NOT_FOUND, None),
    };

    Ok(http::Response::builder().status(status).body(body)?)
}
