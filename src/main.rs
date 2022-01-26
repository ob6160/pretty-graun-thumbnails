//! Default Compute@Edge template program.

use chrono::DateTime;
use chrono::TimeZone;
use chrono::Timelike;
use chrono::Utc;

use fastly::http::{header, Method, StatusCode};
use fastly::{mime, Error, Request, Response};

use std::net::IpAddr;
use std::net::Ipv4Addr;

use rand::{thread_rng, Rng};

use svg::node::element::path::Data;
use svg::node::element::Text as TextElement;
use svg::node::element::{Circle, Group, Path, Rectangle};
use svg::node::Text;
use svg::{Document, Node};

use image::{ColorType, ImageFormat, Rgb, RgbImage};
use imageproc::drawing::{Canvas, draw_cross_mut, draw_filled_circle_mut, draw_filled_rect_mut, draw_hollow_circle_mut, draw_hollow_rect_mut, draw_line_segment_mut};
use imageproc::rect::Rect;

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

    let size = 120;
    let thickness = 2;
    let padding = thickness * 2;
    let inner_width = size - (padding * 2 + thickness);

    // Pattern match on the path...
    match req.get_path() {
        "/" => Ok(Response::from_status(StatusCode::OK)
            .with_content_type(mime::TEXT_HTML_UTF_8)
            .with_body(include_str!("index.html"))),

        "/thumbnail.jpg" => {
            // let article_url = req.get_query_parameter("article").unwrap_or("https://m.code.dev-theguardian.com/artanddesign/2021/oct/27/two-lovers-kiss-behind-a-tree-clifford-prince-kings-best-photograph");
            // let page_req = Request::get([article_url, ".json"].join(""));
            // let mut page_resp = page_req.send("steve")?;
            // let page_resp_body = page_resp.take_body().into_string();
            // let v = serde_json::from_str::<serde_json::Value>(&page_resp_body)?;
            //
            // let mut thumbnail_url = v["config"]["page"]["thumbnail"].to_string().replace("\"", "");
            // if thumbnail_url == "null" {
            //     thumbnail_url = v["config"]["thumbnail"].to_string().replace("\"", "");
            // }
            //
            //
            // let thumb_req = Request::get(thumbnail_url);
            // let mut thumb_resp = thumb_req.send("guim")?;
            // let thumb_resp_body = thumb_resp.take_body();
            //
            // return Ok(Response::from_body(thumb_resp_body))
            let red   = Rgb([255u8, 0u8,   0u8]);
            let green = Rgb([0u8,   255u8, 0u8]);
            let blue  = Rgb([0u8,   0u8,   255u8]);
            let white = Rgb([255u8, 255u8, 255u8]);

            let mut image = RgbImage::new(200, 200);

            // Draw some crosses within bounds
            draw_cross_mut(&mut image, white, 5, 5);
            draw_cross_mut(&mut image, red, 9, 9);
            draw_cross_mut(&mut image, blue, 9, 5);
            draw_cross_mut(&mut image, green, 5, 9);
            // Draw a cross totally outside image bounds - does not panic but nothing is rendered
            draw_cross_mut(&mut image, white, 250, 0);
            // Draw a cross partially out of bounds - the part in bounds is rendered
            draw_cross_mut(&mut image, white, 2, 0);

            // Draw a line segment wholly within bounds
            draw_line_segment_mut(&mut image, (20f32, 12f32), (40f32, 60f32), white);
            // Draw a line segment totally outside image bounds - does not panic but nothing is rendered
            draw_line_segment_mut(&mut image, (0f32, -30f32), (40f32, -20f32), white);
            // Draw a line segment partially out of bounds - the part in bounds is rendered
            draw_line_segment_mut(&mut image, (20f32, 180f32), (20f32, 220f32), white);

            // Draw a hollow rect within bounds
            draw_hollow_rect_mut(&mut image, Rect::at(60, 10).of_size(20, 20), white);
            // Outside bounds
            draw_hollow_rect_mut(&mut image, Rect::at(300, 10).of_size(20, 20), white);
            // Partially outside bounds
            draw_hollow_rect_mut(&mut image, Rect::at(90, -10).of_size(30, 20), white);

            // Draw a filled rect within bounds
            draw_filled_rect_mut(&mut image, Rect::at(130, 10).of_size(20, 20), white);
            // Outside bounds
            draw_filled_rect_mut(&mut image, Rect::at(300, 10).of_size(20, 20), white);
            // Partially outside bounds
            draw_filled_rect_mut(&mut image, Rect::at(180, -10).of_size(30, 20), white);

            // Draw a hollow circle within bounds
            draw_hollow_circle_mut(&mut image, (100, 100), 15, white);
            // Outside bounds
            draw_hollow_circle_mut(&mut image, (400, 400), 20, white);
            // Partially outside bounds
            draw_hollow_circle_mut(&mut image, (100, 190), 20, white);

            // Draw a filled circle within bounds
            draw_filled_circle_mut(&mut image, (150, 100), 15, white);
            // Outside bounds
            draw_filled_circle_mut(&mut image, (450, 400), 20, white);
            // Partially outside bounds
            draw_filled_circle_mut(&mut image, (150, 190), 20, white);
            let mut v = Vec::with_capacity((200 * 200) as usize);
            let mut je = image::codecs::jpeg::JpegEncoder::new(&mut v);
            je.encode_image(&image);
            Ok(Response::from_body(v))
            // Ok(Response::from_status(StatusCode::OK)
            //     .with_content_type(mime::IMAGE_SVG)
            //     .with_body(image.))


        }

        "/clock.svg" => {
            let mut rng = thread_rng();
            let dt: DateTime<Utc> = match req.get_query_parameter("rand") {
                Some(_) => Utc.ymd(2022, 1, 26).and_hms(
                    rng.gen_range(0..24),
                    rng.gen_range(0..60),
                    rng.gen_range(0..60),
                ),

                None => Utc::now(),
            };

            let am = dt.hour12().0;

            let dark = ["olivedrab", "teal", "darkslategray", "maroon"][rng.gen_range(0..4)];
            let light = ["cornsilk", "bisque", "papayawhip", "palegoldenrod"][rng.gen_range(0..4)];

            let fg = if am { dark } else { light };
            let bg = if am { light } else { dark };

            let data = Data::new()
                .move_to((thickness / 2, thickness / 2 + padding))
                .elliptical_arc_by((padding, padding, 0, 0, 1, padding, -padding))
                .horizontal_line_by(inner_width)
                .elliptical_arc_by((padding, padding, 0, 0, 1, padding, padding))
                .vertical_line_by(inner_width)
                .elliptical_arc_by((padding, padding, 0, 0, 1, -padding, padding))
                .horizontal_line_by(-inner_width)
                .elliptical_arc_by((padding, padding, 0, 0, 1, -padding, -padding))
                .close();

            let grid = 9;

            let mut hours = Group::new().set("fill", fg).set("stroke", "none").set(
                "transform",
                format!(
                    "translate({},{})",
                    padding * 2 + grid * 6,
                    padding * 2 + grid * 10
                ),
            );

            for n in 0..(dt.hour() % 12) {
                let x = (n % 6) as i32 * grid;
                let y = (n / 6) as i32 * grid;
                let rect = Circle::new()
                    .set("cx", x)
                    .set("cy", y)
                    .set("r", thickness * 2);

                hours.append(rect);
            }

            let mut minutes = Group::new()
                .set("fill", "none")
                .set("stroke", fg)
                .set("stroke-width", thickness)
                .set(
                    "transform",
                    format!("translate({},{})", padding * 2, padding * 2),
                );

            for n in 0..(dt.minute()) {
                let x = (n % 12) as i32 * grid;
                let y = (n / 12) as i32 * grid * 2 + (n % 2) as i32 * grid;
                let rect = Circle::new()
                    .set("cx", x)
                    .set("cy", y)
                    .set("r", thickness * 2);

                minutes.append(rect);
            }

            let mut seconds = Group::new().set("fill", fg).set("stroke", "none").set(
                "transform",
                format!("translate({},{})", padding * 2, padding * 2),
            );

            for n in 0..(dt.second()) {
                let x = (n / 12) as i32 * grid;
                let y = (n % 12) as i32 * grid;
                let rect = Circle::new().set("cx", x).set("cy", y).set("r", thickness);

                seconds.append(rect);
            }

            let path = Path::new()
                .set("fill", bg)
                .set("stroke", fg)
                .set("stroke-width", thickness)
                .set("d", data);

            let document = Document::new()
                .set("width", size * 3)
                .set("height", size * 3)
                .set("viewBox", (0, 0, size, size))
                .add(path)
                .add(hours)
                .add(minutes)
                .add(seconds);

            Ok(Response::from_status(StatusCode::OK)
                .with_content_type(mime::IMAGE_SVG)
                .with_body(document.to_string()))
        }

        "/info.svg" => {
            let mut rng = thread_rng();

            let fg = ["olivedrab", "teal", "darkslategray", "maroon"][rng.gen_range(0..4)];
            let bg = ["cornsilk", "bisque", "papayawhip", "palegoldenrod"][rng.gen_range(0..4)];

            let rect = Rectangle::new()
                .set("fill", bg)
                .set("stroke", fg)
                .set("stroke-width", thickness)
                .set("x", thickness / 2)
                .set("y", thickness / 2)
                .set("rx", padding)
                .set("width", size - thickness)
                .set("height", size - thickness);

            let ip: String = req
                .get_client_ip_addr()
                .unwrap_or(IpAddr::V4(Ipv4Addr::new(1, 1, 1, 1)))
                .to_string();

            let text = TextElement::new()
                .set("x", size / 2)
                .set("text-anchor", "middle")
                .set("y", 16 + padding)
                .set("font-family", "GuardianTextSansWeb, Helvetica, sans-serif")
                .set("font-size", 16)
                .set("fill", fg)
                .add(Text::new(["IP: ", &ip].concat()));

            let document = Document::new()
                .set("width", size * 3)
                .set("height", size * 3)
                .set("viewBox", (0, 0, size, size))
                .add(rect)
                .add(text);

            Ok(Response::from_status(StatusCode::OK)
                .with_content_type(mime::IMAGE_SVG)
                .with_body(document.to_string()))
        }

        // Catch all other requests and return a 404.
        _ => Ok(Response::from_status(StatusCode::NOT_FOUND)
            .with_body_text_plain("The page you requested could not be found\n")),
    }
}
