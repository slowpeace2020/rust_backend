extern crate file;
extern crate image_base64;

use ic_cdk::export::Principal;
use base64::{encode, decode};
use rand::{self, Rng};
use rand::distributions::Alphanumeric;
use regex::Regex;

type ImgStore = BTreeMap<i128, String>;
type ImgId = i128;

// https://floatflower.me/programming/rust/rocket.rs/glimpse/upload-image-with-base64/
#[post("/images", format = "json", data="<image>")]
#[update("uploadImg")]
fn upload_image(image: String) -> i128 {
    // 檢查格式
    let re = Regex::new(r"^data:image/jpeg;base64,[a-zA-Z0-9+/=]+$").unwrap();

    if !re.is_match(image) {
        return String::from("File format invalid");
    }

    let image_data = image.split(";base64,");
    let vec: Vec<&str> = image_data.collect();

    // 這邊第 2 個 element 就會是 base64 的資料
    let image_bytes = base64::decode(vec[1]).unwrap();
    let latest_img_id = storage::get_mut::<LatestPostId>();


    let img_store = storage::get_mut::<ImgStore>();
    img_store.insert(latest_img_id+10000, image);
    return latest_img_id+10000;
}