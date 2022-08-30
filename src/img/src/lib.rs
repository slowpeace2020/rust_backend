extern crate file;

use ic_cdk::export::Principal;
use base64::{encode, decode};
use rand::{self, Rng};
use rand::distributions::Alphanumeric;
use regex::Regex;

type ImgStore = BTreeMap<i128, String>;
type ImgId = i128;


// #[derive(Debug, Serialize, Deserialize)]
// pub struct UploadedImage {
//     // 用於儲存 base64 編碼的檔案
//     data: String,
//     url: String
// }

#[post("/images")]
#[update("uploadImg")]
fn upload_image(image: String) -> String {
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
    img_store.insert(latest_img_id, image);
    return String::from("Upload successful");
}