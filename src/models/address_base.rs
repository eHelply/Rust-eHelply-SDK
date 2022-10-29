/*
 * eHelply SDK - 1.1.119
 *
 * eHelply SDK for SuperStack Services
 *
 * The version of the OpenAPI document: 1.1.119
 * Contact: support@ehelply.com
 * Generated by: https://openapi-generator.tech
 */

/// AddressBase : **:param** address_type                        **type:** string or None  **:param** address_line_1                      **type:** string or None  **:param** address_line_2                      **type:** string or None  **:param** postal_zip_code                     **type:** string or None  **:param** city                                **type:** string or None  **:param** province_state                      **type:** string or None  **:param** country                             **type:** string or None  **:param** lat                                 **type:** string or None  **:param** lng                                 **type:** string or None



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct AddressBase {
    #[serde(rename = "address_type", skip_serializing_if = "Option::is_none")]
    pub address_type: Option<String>,
    #[serde(rename = "address_line_1", skip_serializing_if = "Option::is_none")]
    pub address_line_1: Option<String>,
    #[serde(rename = "address_line_2", skip_serializing_if = "Option::is_none")]
    pub address_line_2: Option<String>,
    #[serde(rename = "postal_zip_code", skip_serializing_if = "Option::is_none")]
    pub postal_zip_code: Option<String>,
    #[serde(rename = "city", skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[serde(rename = "province_state", skip_serializing_if = "Option::is_none")]
    pub province_state: Option<String>,
    #[serde(rename = "country", skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(rename = "lat", skip_serializing_if = "Option::is_none")]
    pub lat: Option<String>,
    #[serde(rename = "lng", skip_serializing_if = "Option::is_none")]
    pub lng: Option<String>,
}

impl AddressBase {
    /// **:param** address_type                        **type:** string or None  **:param** address_line_1                      **type:** string or None  **:param** address_line_2                      **type:** string or None  **:param** postal_zip_code                     **type:** string or None  **:param** city                                **type:** string or None  **:param** province_state                      **type:** string or None  **:param** country                             **type:** string or None  **:param** lat                                 **type:** string or None  **:param** lng                                 **type:** string or None
    pub fn new() -> AddressBase {
        AddressBase {
            address_type: None,
            address_line_1: None,
            address_line_2: None,
            postal_zip_code: None,
            city: None,
            province_state: None,
            country: None,
            lat: None,
            lng: None,
        }
    }
}


