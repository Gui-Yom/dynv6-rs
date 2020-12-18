use serde::{Deserialize, Serialize};

static BASE_URL: &str = "https://dynv6.com/api/v2";

pub struct DynV6 {
    pub token: String,
}

// Remaining : Create zone and Update record

impl DynV6 {
    pub fn new(token: &str) -> DynV6 {
        DynV6 {
            token: token.to_string(),
        }
    }

    pub fn list_zones(&self) -> Vec<Zone> {
        ureq::get(&format!("{}/zones", BASE_URL))
            .auth_kind("Bearer", &self.token)
            .set("Accept", "application/json")
            .call()
            .into_json_deserialize::<Vec<Zone>>()
            .expect("Can't retrieve zones list")
    }

    pub fn get_zone(&self, id: u64) -> Zone {
        ureq::get(&format!("{}/zones/{}", BASE_URL, id))
            .auth_kind("Bearer", &self.token)
            .set("Accept", "application/json")
            .call()
            .into_json_deserialize::<Zone>()
            .expect("Can't retrieve zone details by id")
    }

    pub fn get_zone_by_name(&self, name: &str) -> Zone {
        ureq::get(&format!("{}/zones/by-name/{}", BASE_URL, name))
            .auth_kind("Bearer", &self.token)
            .set("Accept", "application/json")
            .call()
            .into_json_deserialize::<Zone>()
            .expect("Can't retrieve zone details by name")
    }

    pub fn update_zone(
        &self,
        zone_id: u64,
        ipv4: &str,
        ipv6: Option<&str>,
    ) -> Result<(), ureq::Error> {
        ureq::patch(&format!("{}/zones/{}", BASE_URL, zone_id))
            .auth_kind("Bearer", &self.token)
            .set("Accept", "application/json")
            .send_json(serde_json::json!({
                "ipv4address": ipv4,
                "ipv6prefix": ipv6.or(Some(""))
            }))
            .ok();
        Ok(())
    }

    pub fn delete_zone(&self, id: u64) -> bool {
        ureq::delete(&format!("{}/zones/{}", BASE_URL, id))
            .auth_kind("Bearer", &self.token)
            .set("Accept", "application/json")
            .call()
            .ok()
    }

    pub fn list_records(&self, zone_id: u64) -> Vec<Record> {
        ureq::get(&format!("{}/zones/{}/records", BASE_URL, zone_id))
            .auth_kind("Bearer", &self.token)
            .set("Accept", "application/json")
            .call()
            .into_json_deserialize::<Vec<Record>>()
            .expect("Can't retrieve records list")
    }

    pub fn get_record(&self, zone_id: u64, record_id: u64) -> Record {
        ureq::get(&format!(
            "{}/zones/{}/records/{}",
            BASE_URL, zone_id, record_id
        ))
        .auth_kind("Bearer", &self.token)
        .set("Accept", "application/json")
        .call()
        .into_json_deserialize::<Record>()
        .expect("Can't retrieve record by id")
    }

    pub fn delete_record(&self, zone_id: u64, record_id: u64) -> bool {
        ureq::delete(&format!(
            "{}/zones/{}/records/{}",
            BASE_URL, zone_id, record_id
        ))
        .auth_kind("Bearer", &self.token)
        .set("Accept", "application/json")
        .call()
        .ok()
    }

    pub fn add_record(&self, zone_id: u64, new: CreateRecord) -> Record {
        ureq::post(&format!("{}/zones/{}/records", BASE_URL, zone_id))
            .auth_kind("Bearer", &self.token)
            .set("Accept", "application/json")
            .send_json(serde_json::to_value(new).unwrap())
            .into_json_deserialize::<Record>()
            .expect("Can't retrieve record by id")
    }
}

// objects

#[derive(Serialize, Deserialize)]
pub struct Zone {
    pub id: u64,
    pub name: String,
    pub ipv4address: String,
    pub ipv6prefix: String,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
}

#[derive(Serialize, Deserialize)]
pub struct Record {
    pub name: String,
    pub priority: Option<u16>,
    pub port: Option<u16>,
    pub weight: Option<u16>,
    pub flags: Option<u8>,
    pub tag: Option<String>,
    pub data: String,
    #[serde(rename = "expandedData", default)]
    pub expanded_data: String,
    pub id: u64,
    #[serde(rename = "zoneID")]
    pub zone_id: u64,
    #[serde(rename = "type")]
    pub record_type: String,
}

#[derive(Serialize, Deserialize, Default)]
pub struct CreateRecord {
    pub name: String,
    pub priority: u16,
    pub port: u16,
    pub weight: u16,
    pub flags: u8,
    pub tag: String,
    pub data: String,
    #[serde(rename = "type")]
    pub record_type: String,
}
