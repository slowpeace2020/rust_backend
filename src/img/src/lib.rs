extern crate file;

use ic_cdk::export::Principal;
use base64::{encode, decode};
use rand::{self, Rng};
use rand::distributions::Alphanumeric;
use regex::Regex;

type ImgStore = BTreeMap<Principal, UploadedImage>;

#[derive(Debug, Serialize, Deserialize)]
pub struct UploadedImage {
    // 用於儲存 base64 編碼的檔案
    data: String,
    url: String
}

#[post("/images", format = "json", data="<image>")]
#[update("uploadImg")]
fn upload_image(image: Json<UploadedImage>,user_principal_id:String) -> String {
    // 檢查格式
    let re = Regex::new(r"^data:image/jpeg;base64,[a-zA-Z0-9+/=]+$").unwrap();

    if !re.is_match(image.data.as_str())||image.url.as_str()==null {
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

    let uploadedImage = UploadedImage{
        data:image_bytes,
        url: image.url.as_str(),
    };

    let img_store = storage::get_mut::<UploadedImage>();
    img_store.insert(user_principal_id, uploadedImage);
    return String::from("Upload successful");
}