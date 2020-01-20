#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
use rocket::request;
use rocket::response;
use rocket_contrib::json;
use serde::ser::{Serialize, SerializeStruct, Serializer};

pub struct HostHeader<'a>(pub &'a str);
impl<'a, 'r> request::FromRequest<'a, 'r> for HostHeader<'a> {
    type Error = ();
    fn from_request(request: &'a request::Request) -> rocket::request::Outcome<Self, Self::Error> {
        match request.headers().get_one("Host") {
            Some(h) => rocket::Outcome::Success(HostHeader(h)),
            None => rocket::Outcome::Forward(()),
        }
    }
}

#[get("/")]
fn index() -> response::Redirect {
    response::Redirect::to("/images")
}

struct WebImage {
    id: String,
    host: String,
}

impl WebImage {
    fn from(id: &str, host: &str) -> WebImage {
        WebImage {
            id: String::from(id),
            host: String::from(host),
        }
    }
    fn get_raw_path(&self) -> String {
        format!("static/images/{}.jpg", self.id)
    }
    fn get_raw_url(&self) -> String {
        format!("http://{}/images/{}/raw", self.host, self.id)
    }
}

impl Serialize for WebImage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("Image", 2)?;
        s.serialize_field("id", &self.id)?;
        s.serialize_field("raw_url", &self.get_raw_url())?;
        s.end()
    }
}

struct WebImageList {
    images: Vec<WebImage>,
}

impl WebImageList {
    fn new() -> WebImageList {
        WebImageList { images: Vec::new() }
    }
}

impl Serialize for WebImageList {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("ImageList", 1)?;
        s.serialize_field("images", &self.images)?;
        s.end()
    }
}

#[get("/images")]
fn images(host: HostHeader) -> json::Json<WebImageList> {
    println!("[end]");
    let example_id: Vec<&str> = vec!["11111", "22222", "33333", "23333"];
    let mut image_list: WebImageList = WebImageList::new();
    for id in example_id.iter() {
        image_list.images.push(WebImage::from(id, host.0))
    }
    json::Json(image_list)
}

#[get("/images/<id>")]
fn image_detail(id: String, host: HostHeader) -> json::Json<WebImage> {
    let pic: WebImage = WebImage::from(&id, host.0);
    json::Json(pic)
}

#[get("/images/<id>/raw")]
fn image_raw(id: String, host: HostHeader) -> Option<response::NamedFile> {
    response::NamedFile::open(WebImage::from(&id, host.0).get_raw_path()).ok()
}

#[get("/example/adder?<a>&<b>")]
fn example_adder(a: i32, b: i32) -> String {
    format!("{} + {} = {}", a, b, a + b)
}

fn main() {
    // rocket::ignite().mount("/", routes![index]).launch();
    rocket::ignite()
        .mount(
            "/",
            routes![index, images, image_detail, image_raw, example_adder],
        )
        .launch();
}
