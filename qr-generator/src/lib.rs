use base64;
use percent_encoding::percent_decode;
use qrcode_generator::{to_svg_to_string, QrCodeEcc};
use spin_sdk::http::{IntoResponse, Request, ResponseBuilder};
use spin_sdk::http_component;
use spin_sdk::variables;

/// A simple Spin HTTP component.
#[http_component]
fn handle_qr(req: Request) -> anyhow::Result<impl IntoResponse> {
    let mut url: String = "https://www.fermyon.com/spin".to_string();

    if req.query() != "" {
        url = req.query().to_string();
    } else if let Ok(qr_url) = variables::get("qr_url") {
        if !qr_url.is_empty() {
            url = qr_url;
        }
    };

    println!("Generating QR code for: {}", url);

    let decoded_query = percent_decode(url.as_bytes()).decode_utf8();
    let svg = to_svg_to_string(
        decoded_query.unwrap().as_ref(),
        QrCodeEcc::Medium,
        512,
        None::<String>,
    )
    .expect(format!("Failed to generate QR code for: {}", url).as_str());

    let body = format!(
        r#"
        <!DOCTYPE html>
        <html>
            <head>
                <title>QR Code Generator</title>
                <style>
                #divElement {{
                    margin: auto;
                    width: 50%;
                    border: 3px solid black;
                    padding: 10px;
                    text-align: center;
                    font-family: "Lucida Console", "Courier New", monospace;
                    font-size: x-large;
            }}
            </style>
            </head>
            <body>
                <div id="divElement">
                    <h1>QR Code Generator</h1>
                    <p>{}</p>
                    <p><img src="data:image/svg+xml;base64,{}" alt="QR Code" /></p>
                </div>
            </body>
        </html>"#,
        url,
        base64::encode(svg.as_bytes())
    );

    Ok(ResponseBuilder::new(200).body(body.into_bytes()).build())
}
