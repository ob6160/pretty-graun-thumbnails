//! Default Compute@Edge template program.

use std::io::{Cursor, Read};
use chrono::DateTime;
use chrono::TimeZone;
use chrono::Timelike;
use chrono::Utc;

use fastly::http::{header, Method, StatusCode};
use fastly::{mime, Error, Request, Response};

use std::net::IpAddr;
use std::net::Ipv4Addr;
use std::str::SplitWhitespace;

use rand::{thread_rng, Rng};

use svg::node::element::path::Data;
use svg::node::element::Text as TextElement;
use svg::node::element::{Circle, Group, Path, Rectangle};
use svg::node::Text;
use svg::{Document, Node};

use image::{ColorType, DynamicImage, ImageDecoder, ImageFormat, Rgb, RgbImage, Rgba, Pixel, ImageBuffer, load_from_memory_with_format};
use image::imageops::{FilterType, vertical_gradient};
use image::jpeg::JpegDecoder;
use image::png::PngDecoder;
use imageproc::definitions::Image;
use imageproc::drawing::{Canvas, draw_cross_mut, draw_filled_circle_mut, draw_filled_rect_mut, draw_hollow_circle_mut, draw_hollow_rect_mut, draw_line_segment_mut, draw_text_mut};
use imageproc::rect::Rect;
use rusttype::{Font, Scale};
use urlencoding::decode;


/// The entry point for your application.
///
/// This function is triggered when your service receives a client request. It could be used to
/// route based on the request properties (such as method or path), send the request to a backend,
/// make completely new requests, and/or generate synthetic responses.
///
/// If `main` returns an error, a 500 error response will be delivered to the client.

#[fastly::main]
fn main(req: Request) -> Result<Response, Error> {
    // Filter request methods...
    match req.get_method() {
        // Allow GET and HEAD requests.
        &Method::GET | &Method::HEAD => (),

        // Deny anything else.
        _ => {
            return Ok(Response::from_status(StatusCode::METHOD_NOT_ALLOWED)
                .with_header(header::ALLOW, "GET, HEAD")
                .with_body_text_plain("This method is not allowed\n"))
        }
    };

    // Pattern match on the path...
    match req.get_path() {
        "/" => Ok(Response::from_status(StatusCode::OK)
            .with_content_type(mime::TEXT_HTML_UTF_8)
            .with_body(include_str!("index.html"))),

        "/thumbnail.jpg" => {
            let article_url = decode(req.get_query_parameter("article").unwrap_or("https://m.code.dev-theguardian.com/artanddesign/2021/oct/27/two-lovers-kiss-behind-a-tree-clifford-prince-kings-best-photograph"))?;
            let mut page_req = Request::get([&article_url.to_string(), ".json"].join(""));
            page_req.set_header("host", "www.theguardian.com");

            let mut backend_choice= "news";
            if page_req.get_url_str().contains("m.code.dev-theguardian.com") {
                backend_choice = "steve";
            }
            let mut page_resp = page_req.send(backend_choice)?;
            let page_resp_body = page_resp.take_body().into_string();



            let v = serde_json::from_str::<serde_json::Value>(&page_resp_body)?;

            let mut thumbnail_url = v["config"]["page"]["thumbnail"].to_string().replace("\"", "");
            if thumbnail_url == "null" {
                thumbnail_url = v["config"]["thumbnail"].to_string().replace("\"", "");
            }

            let mut headline = v["config"]["page"]["headline"].to_string().replace("\"", "");
            if headline == "null" {
                headline = v["headline"].to_string().replace("\"", "");
            }

            let mut tagline = [v["config"]["page"]["series"].to_string().replace("\"", "") ,"/".to_string()].join("");
            if tagline.len() > 23 {
                tagline = [v["config"]["page"]["sectionName"].to_string().replace("\"", "") ,"/".to_string()].join("");
            }
            if tagline == "null/" {
                tagline = [v["config"]["page"]["keywords"].to_string().replace("\"", "").to_string().split(",").collect::<Vec<_>>().first().unwrap_or(&"").to_string(),"/".to_string()].join("");
            }

            let headline_bits = headline.split_whitespace();
            let headline_withnewline = headline_bits.enumerate().map(|(i, s)| {
                if i != 0 && (i % 7) == 0 {
                    return ["\n",s ].join("");
                }
                return s.to_string();
            }).collect::<Vec<_>>().join(" ");
            let headline_pieces = headline_withnewline.split("\n");

            let thumb_req = Request::get(thumbnail_url);
            let mut thumb_resp = thumb_req.send("guim")?;
            let thumb_resp_body = thumb_resp.take_body();


            let mut decoder = JpegDecoder::new(thumb_resp_body)?;


            let graun_png = include_bytes!("graun_new.png");
            let graun_loaded = load_from_memory_with_format(graun_png, ImageFormat::Png)?;


            let mut image = DynamicImage::from_decoder(decoder)?.to_rgba8();


            let red = Rgb([255u8, 0u8, 0u8]);
            let green = Rgb([0u8, 255u8, 0u8]);
            let blue = Rgb([0u8, 0u8, 255u8]);
            let graun_blue = Rgba([5u8, 41u8, 98u8, 255u8]);
            let graun_orange = Rgba([255u8, 229u8, 0u8, 255u8]);
            let graun_text_bg = Rgba([246u8, 246u8, 246u8, 255u8]);
            let white = Rgba([255u8, 255u8, 255u8, 255u8]);

            let isLightModeOn = (req.get_query_parameter("lightMode").unwrap_or("true")).parse().unwrap();

            let mut title_rect = Rect::at(0, 0).of_size(500, 80);
            draw_filled_rect_mut(&mut image, title_rect, graun_blue);

            if isLightModeOn {
                let mut title_text_rect = Rect::at(0, 0).of_size(325, 80);
                draw_filled_rect_mut(&mut image, title_text_rect, graun_text_bg);
            }


            let resized_graun = image::imageops::resize(&graun_loaded, 156, 65, FilterType::Lanczos3);
            image::imageops::overlay(&mut image, &resized_graun, 335, 5);


            let graun_font = Vec::from(include_bytes!("GuardianTextEgyptian-Regular.ttf") as &[u8]);
            let graun_font = Font::try_from_vec(graun_font).unwrap();

            let height = 18.0;
            let scale = Scale {
                x: height,
                y: height,
            };

            let normal_text_colour = if isLightModeOn {Rgba([8u8, 8u8, 8u8, 255u8])} else { Rgba([255u8, 255u8, 255u8, 255u8])};

            let mut nummy_pieces = headline_pieces.enumerate();
            let count = nummy_pieces.clone().count();
            let headline_modifier = if count >= 1 { 1.5 } else {2.0};
            let text_modifier = if count >= 1 { 10 } else {0};
            for (i, headline_chunk) in &mut nummy_pieces {

                draw_text_mut(&mut image, normal_text_colour, 5, (i as f32 * height) as u32 + 35 as u32 - text_modifier as u32, scale, &graun_font, &headline_chunk.trim_start());
            }


            let scale = Scale {
                x: height * headline_modifier,
                y: height * headline_modifier,
            };

            let headline_colour = if isLightModeOn {Rgba([199u8, 0u8, 0u8, 255u8])} else { graun_orange };
            draw_text_mut(&mut image, headline_colour, 5, 0, scale, &graun_font, &tagline);

            let mut v = Vec::with_capacity((500 * 300) as usize);
            let mut je = image::codecs::png::PngEncoder::new(&mut v);
            je.encode(&image, 500, 300, ColorType::Rgba8);
            Ok(Response::from_body(v))
        }

        // Catch all other requests and return a 404.
        _ => Ok(Response::from_status(StatusCode::NOT_FOUND)
            .with_body_text_plain("The page you requested could not be found\n")),
    }
}
