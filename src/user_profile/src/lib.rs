use easy_hasher::easy_hasher::raw_keccak256;
use ic_cdk::export::{
    candid::{CandidType, Deserialize},
    Principal,
};
use ic_cdk::println;
use ic_cdk::*;
use ic_cdk_macros::*;
use libsecp256k1::recover;
use std::{collections::BTreeMap, convert::TryInto};
use std::str::FromStr;

type ProfileStore = BTreeMap<Principal, Profile>;
type NameProfileStore = BTreeMap<String, Profile>;


#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct Profile {
    pub principal_id: Principal,
    pub address: String,
    pub name: String,
    pub description: String,
    pub img: String,
}

impl Default for Profile {
    fn default() -> Self {
        Profile {
            principal_id: ic_cdk::caller(),
            address: String::from(""),
            name: String::from(""),
            description: String::from(""),
            img: String::from(""),
        }
    }
}


#[query(name = "getProfileByPrincipal")]
fn get_by_principal(principal: Principal) -> Option<&'static Profile> {
    let profile_store = storage::get::<ProfileStore>();

    for (p, profile) in profile_store.iter() {
        if p.eq(&principal) {
            return Some(profile);
        }
    }

    None
}

#[query(name = "getProfileByEth")]
fn get_by_eth(eth_address: String) -> Option<&'static Profile> {
    let profile_store = storage::get::<ProfileStore>();

    for (_, profile) in profile_store.iter() {
        if profile.address.eq(&eth_address) {
            return Some(profile);
        }
    }

    None
}

#[query(name = "getProfileByName")]
fn get_by_name(name: String) -> Option<&'static Profile> {
    let profile_store = storage::get::<ProfileStore>();

    for (_, profile) in profile_store.iter() {
        if profile.name.eq(&name) {
            return Some(profile);
        }
    }

    None
}

#[query(name = "getOwnProfile")]
fn get_own_profile() -> Profile {
    let principal_id = ic_cdk::caller();
    let profile_store = storage::get::<ProfileStore>();

    profile_store
        .get(&principal_id)
        .cloned()
        .unwrap_or_else(|| Profile::default())
}


#[query(name = "getOwnPrincipal")]
fn get_own_principal_id() -> Principal {
    ic_cdk::caller()
}

#[query(name = "getPrincipalByEth")]
fn get_principal_by_eth(eth_address: String) -> Option<Principal> {
    let profile_store = storage::get::<ProfileStore>();

    for (principal, profile) in profile_store.iter() {
        if profile.address.to_lowercase().eq(&eth_address.to_lowercase()) {
            return Some(*principal);
        }
    }

    None
}

#[query(name = "getPrincipalByName")]
fn get_principal_by_name(name: String) -> Option<Principal> {
    let nameprofile_store = storage::get_mut::<NameProfileStore>();
    println!("nameprofile_store: {:?}",nameprofile_store);
    for (username, profile) in nameprofile_store.iter() {
        if username.eq(&name) {
            return Some(profile.principal_id);
        }
    }

    None
}

#[query]
fn search(text: String) -> Option<&'static Profile> {
    let text = text.to_lowercase();
    let profile_store = storage::get::<ProfileStore>();
    println!("profile_store: {:?}",profile_store);
    for (_, profile) in profile_store.iter() {
        if profile.name.to_lowercase().contains(&text) || profile.description.to_lowercase().contains(&text) {
            return Some(profile);
        }
    }

    None
}

#[query]
fn list() -> Vec<&'static Profile> {
    let profile_store = storage::get::<ProfileStore>();

    let mut profiles: Vec<&'static Profile> = Vec::new();

    for (_, profile) in profile_store.iter() {
        profiles.push(profile);
    }

    return profiles;
}

#[update(name = "createUserProfile")]
fn create_new_user(name:String,description:String,address:String) -> Profile{
    let nameprofile_store = storage::get_mut::<NameProfileStore>();
    println!("nameprofile_store: {:?}",nameprofile_store);
    if nameprofile_store.contains_key(&name){
        return Profile{
            principal_id: Principal::from_str(&String::from("renrk-eyaaa-aaaaa-aaada-cai")).unwrap(),
            address,
            name,
            description: String::from("The name already exist, please try another name"),
            img: String::from("")
        };
    }
    let profile = Profile{
        principal_id: Principal::from_str(&String::from("renrk-eyaaa-aaaaa-aaada-cai")).unwrap(),
        address,
        name,
        description,
        img: String::from("")
    };
    // let cid2 = Principal::from_str(&text).unwrap();
    // let user = get_by_principal(cid2);
    // println!("user: {:?}",user);
    // let text = "jkies-sibbb-ap6";
    // let cid2 = Principal::from_str(&text).unwrap();
    // let user = get_by_principal(cid2);
    // println!("user: {:?}",user);

    // let principal_id = Principal::from_str(&text).unwrap();
    // println!("principal_id: {}",principal_id);
    // let profile_store = storage::get_mut::<ProfileStore>();
    // profile_store.insert(principal_id, profile.clone());
    //
    // //save new user
    // let mut profiles: Vec<(&Principal, &Profile)> = Vec::new();
    // for (principal, profile) in profile_store.iter() {
    //     profiles.push((principal, profile));
    // }
    //
    // storage::stable_save((profiles,)).unwrap();
    // println!("profile_store:{:?}",profile_store);
    _save_name_profile(profile.clone());
    profile
}

#[update(name = "linkPrincipalID")]
fn link_principal_id(username: String) -> String {
    let nameprofile_store = storage::get_mut::<NameProfileStore>();
    let mut profile_option = nameprofile_store.get(&username);
    match profile_option.as_mut() {
        Some(profile) =>  {
            if profile.principal_id.eq(&Principal::from_str("renrk-eyaaa-aaaaa-aaada-cai").unwrap()){
                let mut cur_profile = profile.clone();
                cur_profile.principal_id =  ic_cdk::caller();
                nameprofile_store.insert(username,cur_profile.clone());
                return "success".to_string();
            }


        },
        None => {
            return "could not find the user profile".to_string();
        },
    }

    return "the user profile already linked to another user".to_string();

}

fn _save_profile(profile: Profile) -> () {
    let principal_id = ic_cdk::caller();

    let profile_store = storage::get_mut::<ProfileStore>();

    profile_store.insert(principal_id, profile.clone());
}

fn _save_name_profile(profile: Profile) -> (){
    let nameprofile_store = storage::get_mut::<NameProfileStore>();
    nameprofile_store.insert(profile.clone().name,profile.clone());
    println!("nameprofile_store: {:?}",nameprofile_store);
}

#[update(name = "setName")]
fn set_name(handle: String) -> Profile {
    let mut profile = get_own_profile();
    let nameprofile_store = storage::get_mut::<NameProfileStore>();
    if nameprofile_store.contains_key(&handle){
        return Profile{
            principal_id: Principal::from_str(&String::from("renrk-eyaaa")).unwrap(),
            address: String::from(""),
            name:handle,
            description: String::from("The name already exist, please try another name"),
            img: String::from("")
        };
    }

    if profile.name !=""{
        nameprofile_store.remove(&profile.clone().name);
    }
    profile.name = handle;
    _save_profile(profile.clone());
    _save_name_profile(profile.clone());
    return profile;
}

#[update(name = "setDescription")]
fn set_description(description: String) -> Profile {
    let mut profile = get_own_profile();
    profile.description = description;
    _save_profile(profile.clone());
    return profile;
}

#[update(name = "linkAddress")]
fn link_address(message: String, signature: String) -> Profile {
    let mut signature_bytes = hex::decode(signature.trim_start_matches("0x")).unwrap();
    let recovery_byte = signature_bytes.pop().expect("No recovery byte");
    let recovery_id = libsecp256k1::RecoveryId::parse_rpc(recovery_byte).unwrap();
    let signature_slice = signature_bytes.as_slice();
    let signature_bytes: [u8; 64] = signature_slice.try_into().unwrap();
    let signature = libsecp256k1::Signature::parse_standard(&signature_bytes).unwrap();
    let message_bytes = hex::decode(message.trim_start_matches("0x")).unwrap();
    let message_bytes: [u8; 32] = message_bytes.try_into().unwrap();
    let message = libsecp256k1::Message::parse(&message_bytes);
    let key = recover(&message, &signature, &recovery_id).unwrap();
    let key_bytes = key.serialize();
    let keccak256 = raw_keccak256(key_bytes[1..].to_vec());
    let keccak256_hex = keccak256.to_hex_string();
    let mut address: String = "0x".to_owned();
    address.push_str(&keccak256_hex[24..]);

    println!("Linked eth address {:?}", address);

    let mut profile = get_own_profile();
    profile.address = address.to_lowercase().clone();
    _save_profile(profile.clone());

    return profile;
}

#[pre_upgrade]
fn pre_upgrade() {
    let profile_store = storage::get::<ProfileStore>();

    let mut profiles: Vec<(&Principal, &Profile)> = Vec::new();

    for (principal, profile) in profile_store.iter() {
        profiles.push((principal, profile));
    }
    storage::stable_save((profiles,)).unwrap();
}

#[post_upgrade]
fn post_upgrade() {
    let profile_store = storage::get_mut::<ProfileStore>();

    let res:Result<(Vec<(Principal, Profile)>,), String> = storage::stable_restore();
    match res {
        Ok((old_profiles,)) => {
            for profile in old_profiles {
                profile_store.insert(profile.0, profile.1.clone());
            }
            return;
        }
        Err(_) => return
    }
}