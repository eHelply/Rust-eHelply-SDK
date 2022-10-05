/*
 * eHelply SDK - 1.1.110
 *
 * eHelply SDK for SuperStack Services
 *
 * The version of the OpenAPI document: 1.1.110
 * Contact: support@ehelply.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct LoggingDynamo {
    #[serde(rename = "service")]
    pub service: String,
    #[serde(rename = "time")]
    pub time: String,
    #[serde(rename = "log")]
    pub log: serde_json::Value,
    #[serde(rename = "severity")]
    pub severity: String,
    #[serde(rename = "subject")]
    pub subject: String,
    #[serde(rename = "service_name")]
    pub service_name: String,
}

impl LoggingDynamo {
    pub fn new(service: String, time: String, log: serde_json::Value, severity: String, subject: String, service_name: String) -> LoggingDynamo {
        LoggingDynamo {
            service,
            time,
            log,
            severity,
            subject,
            service_name,
        }
    }
}


