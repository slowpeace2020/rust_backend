extern crate file;

use ic_cdk::export::Principal;
use base64::{encode, decode};
use rand::{self, Rng};
use rand::distributions::Alphanumeric;
use regex::Regex;

type ImgStore = BTreeMap<Principal, UploadedImage>;

#[derive(Debug, Serialize, Deserialize)]
struct UploadedImage {
    // 用於儲存 base64 編碼的檔案
    data: String
}

#[post("/images", format = "json", data="<image>")]
#[update("uploadImg")]
fn upload_image(image: Json<UploadedImage>,user_principal_id:String) -> String {

    // 檢查格式
    let re = Regex::new(r"^data:image/jpeg;base64,[a-zA-Z0-9+/=]+$").unwrap();

    if !re.is_match(image.data.as_str()) {
        return String::from("File format invalid");
    }

    // 因為 base64 格式的會以 data:[MIME_TYPE];base64,來作為開頭
    // 但是前面是必要資料，其中包含檔案的 MIME_TYPE
    // 所以我們把 ;base64, 作為切分資料的依據
    // 此外也可以僅用 , 作為切分依據，
    // 因為 base64 的字元集僅包含：26個大小寫字母、10個數字，
    // 以及加號(+)、斜槓(/) 還有等號(=)
    let image_data = image.data.split(";base64,");
    let vec: Vec<&str> = image_data.collect();

    // 這邊第 2 個 element 就會是 base64 的資料
    let image_bytes = base64::decode(vec[1]).unwrap();

    let img_store = storage::get_mut::<UploadedImage>();
    img_store.insert(user_principal_id, image_bytes);
    return String::from("Upload successful");
}

fn upload_file(cont_type: &ContentType, data: Data) -> Result<Custom<String>, Custom<String>> {
    // this and the next check can be implemented as a request guard but it seems like just
    // more boilerplate than necessary
    if !cont_type.is_form_data() {
        return Err(Custom(
            Status::BadRequest,
            "Content-Type not multipart/form-data".into()
        ));
    }

    let (_, boundary) = cont_type.params()
        .find(|&(k, _)| k == "boundary")
        .ok_or_else(
            || Custom(
                Status::BadRequest,
                "`Content-Type: multipart/form-data` boundary param not provided".into()
            )
        )?;

    // The hot mess that ensues is some weird combination of the two links that follow
    // and a LOT of hackery to move data between closures.
    // https://github.com/SergioBenitez/Rocket/issues/106
    // https://github.com/abonander/multipart/blob/master/examples/rocket.rs
    let mut d = Vec::new();
    data.stream_to(&mut d).expect("Unable to read");
    let mut mp = Multipart::with_body(Cursor::new(d), boundary);

    let mut file_name = String::new();
    let mut categories_string = String::new();
    let mut raw_file_data = Vec::new();

    mp.foreach_entry(|mut entry| {
        if *entry.headers.name == *"fileName" {
            let file_name_vec = entry.data.fill_buf().unwrap().to_owned();
            file_name = from_utf8(&file_name_vec).unwrap().to_string()
        } else if *entry.headers.name == *"tags" {
            let tags_vec = entry.data.fill_buf().unwrap().to_owned();
            categories_string = from_utf8(&tags_vec).unwrap().to_string();
        } else if *entry.headers.name == *"file" {
            raw_file_data = entry.data.fill_buf().unwrap().to_owned()
        }
    }).expect("Unable to iterate");

    let s3_file_manager = s3_interface::S3FileManager::new(None, None, None, None);
    s3_file_manager.put_file_in_bucket(file_name.clone(), raw_file_data);

    let tag_name_val_pairs = vec![("tags".to_string(), categories_string)];
    s3_file_manager.put_tags_on_file(file_name, tag_name_val_pairs);

    return Ok(
        Custom(Status::Ok, "Image Uploaded".to_string())
    );
}

//图片处理
// https://codeantenna.com/a/yxU7uOLzOf

use wasm_bindgen::prelude::*;
extern crate image;
use base64::encode;
use std::io::{Cursor, Read, Seek, SeekFrom};
use std::vec::Vec;

// 加载图片获取图片信息
fn load_image_from_array(arr: &[u8]) -> image::DynamicImage {
    let img = match image::load_from_memory_with_format(arr, image::ImageFormat::Png) {
        Ok(img) => img,
        Err(err) => {
            panic!("format error, {:?}", err);
        }
    };
    img
}
//将图片转成base64返回
fn get_base64_image(img: image::DynamicImage) -> String {
    let mut c = Cursor::new(Vec::new());
    match img.write_to(&mut c, image::ImageFormat::Png) {
        Ok(c) => c,
        Err(error) => panic!(
            "There was a problem writing the resulting buffer: {:?}",
            error
        ),
    };
    c.seek(SeekFrom::Start(0)).unwrap();
    let mut out = Vec::new();
    c.read_to_end(&mut out).unwrap();
    let stt = encode(&mut out);
    let together = format!("{}{}", "data:image/png;base64,", stt);
    return together;
}

// 需要传到前端的处理图片函数
#[wasm_bindgen]
pub fn get_image_blur(data: &[u8]) -> String {
    let mut image = load_image_from_array(data);
    image = image.blur(2.0);// 将图片模糊处理、也可以缩放、灰度、对比度等处理
    let base64_str = get_base64_image(image);
    return base64_str;
}


fn main() {
    rocket::ignite().mount("/", routes![
        upload_image
    ]).launch();
}