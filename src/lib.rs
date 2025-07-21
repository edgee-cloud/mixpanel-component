use crate::exports::edgee::components::data_collection::Data;
use crate::exports::edgee::components::data_collection::{Dict, EdgeeRequest, Event, HttpMethod};
use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use exports::edgee::components::data_collection::Guest;
use std::collections::HashMap;

wit_bindgen::generate!({world: "data-collection", path: ".edgee/wit", generate_all});
export!(Component);

struct Component;

/*
* Implement the Guest trait for the Component struct
* to create the required functions for the data collection protocol
* for your provider.
* The functions are page, track, and user.
* The page function is called when the page event is triggered.
* The track function is called when the track event is triggered.
* The user function is called when the user event is triggered.
* The functions should return an EdgeeRequest or an error message.
* The EdgeeRequest contains the method, url, headers, and body of the request.
*/

fn insert_if_nonempty(map: &mut HashMap<String, String>, key: &str, value: &str) {
    if !value.trim().is_empty() {
        map.insert(key.to_string(), value.to_string());
    }
}

impl Guest for Component {
    fn page(edgee_event: Event, settings_dict: Dict) -> Result<EdgeeRequest, String> {
        let settings = Settings::new(settings_dict).map_err(|e| e.to_string())?;

        let mut props = HashMap::new();

        if let Data::Page(ref data) = edgee_event.data {
            insert_if_nonempty(&mut props, "url", &data.url);
            insert_if_nonempty(&mut props, "title", &data.title);
            insert_if_nonempty(&mut props, "path", &data.path);
            insert_if_nonempty(&mut props, "referrer", &data.referrer);
            insert_if_nonempty(&mut props, "category", &data.category);
            insert_if_nonempty(&mut props, "name", &data.name);

            for (k, v) in &data.properties {
                insert_if_nonempty(&mut props, k, v);
            }

            enrich_with_page_context(&mut props, &edgee_event.context.page);
            enrich_with_campaign_context(&mut props, &edgee_event.context.campaign);
            enrich_with_session_context(&mut props, &edgee_event.context.session);
            enrich_with_client_context(&mut props, &edgee_event.context.client);

            return build_mixpanel_request(&edgee_event, &settings, "Page View", props);
        }

        Err("Invalid event type for page".into())
    }

    fn track(edgee_event: Event, settings_dict: Dict) -> Result<EdgeeRequest, String> {
        let settings = Settings::new(settings_dict).map_err(|e| e.to_string())?;
        let mut props = HashMap::new();

        if let Data::Track(ref data) = edgee_event.data {
            for (k, v) in &data.properties {
                insert_if_nonempty(&mut props, k, v);
            }

            enrich_with_page_context(&mut props, &edgee_event.context.page);
            enrich_with_campaign_context(&mut props, &edgee_event.context.campaign);
            enrich_with_session_context(&mut props, &edgee_event.context.session);
            enrich_with_client_context(&mut props, &edgee_event.context.client);
            return build_mixpanel_request(&edgee_event, &settings, &data.name, props);
        }

        Err("Invalid event type for track".into())
    }

    fn user(edgee_event: Event, settings_dict: Dict) -> Result<EdgeeRequest, String> {
        let settings = Settings::new(settings_dict).map_err(|e| e.to_string())?;
        let user = &edgee_event.context.user;
        let client = &edgee_event.context.client;

        let distinct_id = if user.user_id.trim().is_empty() {
            user.edgee_id.clone()
        } else {
            user.user_id.clone()
        };

        let mut props = HashMap::new();
        props.insert("$distinct_id".into(), distinct_id.clone());
        insert_if_nonempty(&mut props, "$ip", &client.ip);

        for (k, v) in &user.properties {
            insert_if_nonempty(&mut props, k, v);
        }

        enrich_with_page_context(&mut props, &edgee_event.context.page);
        enrich_with_campaign_context(&mut props, &edgee_event.context.campaign);
        enrich_with_session_context(&mut props, &edgee_event.context.session);
        enrich_with_client_context(&mut props, client);

        build_mixpanel_user_request(&settings, distinct_id, props)
    }
}

pub struct Settings {
    pub api_secret: String,
    pub project_token: String,
    pub project_id: Option<String>,
    pub region: String,
}

impl Settings {
    pub fn new(settings_dict: Dict) -> anyhow::Result<Self> {
        let settings_map: HashMap<String, String> = settings_dict
            .iter()
            .map(|(key, value)| (key.to_string(), value.to_string()))
            .collect();

        let api_secret = settings_map
            .get("api_secret")
            .filter(|t| !t.trim().is_empty())
            .ok_or_else(|| anyhow::anyhow!("Missing or empty 'api_secret' setting"))?
            .to_string();

        let project_token = settings_map
            .get("project_token")
            .filter(|t| !t.trim().is_empty())
            .ok_or_else(|| anyhow::anyhow!("Missing or empty 'project_token' setting"))?
            .to_string();

        let project_id = settings_map.get("project_id").cloned();

        let region = settings_map
            .get("region")
            .cloned()
            .unwrap_or_else(|| "api".to_string());

        Ok(Self {
            api_secret,
            project_token,
            project_id,
            region,
        })
    }
}

fn enrich_with_client_context(
    props: &mut HashMap<String, String>,
    client: &crate::exports::edgee::components::data_collection::Client,
) {
    insert_if_nonempty(props, "ip", &client.ip);
    insert_if_nonempty(props, "city", &client.city);
    insert_if_nonempty(props, "country_code", &client.country_code);
    insert_if_nonempty(props, "country_name", &client.country_name);
    insert_if_nonempty(props, "continent", &client.continent);
    insert_if_nonempty(props, "region", &client.region);
    insert_if_nonempty(props, "locale", &client.locale);
    insert_if_nonempty(props, "timezone", &client.timezone);
    insert_if_nonempty(props, "os_name", &client.os_name);
    insert_if_nonempty(props, "os_version", &client.os_version);
    insert_if_nonempty(props, "user_agent", &client.user_agent);
    insert_if_nonempty(
        props,
        "user_agent_architecture",
        &client.user_agent_architecture,
    );
    insert_if_nonempty(props, "user_agent_bitness", &client.user_agent_bitness);
    insert_if_nonempty(
        props,
        "user_agent_full_version_list",
        &client.user_agent_full_version_list,
    );
    insert_if_nonempty(
        props,
        "user_agent_version_list",
        &client.user_agent_version_list,
    );
    insert_if_nonempty(props, "user_agent_mobile", &client.user_agent_mobile);
    insert_if_nonempty(props, "user_agent_model", &client.user_agent_model);
    props.insert("screen_width".to_string(), client.screen_width.to_string());
    props.insert(
        "screen_height".to_string(),
        client.screen_height.to_string(),
    );
    props.insert(
        "screen_density".to_string(),
        client.screen_density.to_string(),
    );
}

fn enrich_with_page_context(
    props: &mut HashMap<String, String>,
    page: &crate::exports::edgee::components::data_collection::PageData,
) {
    insert_if_nonempty(props, "url", &page.url);
    insert_if_nonempty(props, "path", &page.path);
    insert_if_nonempty(props, "title", &page.title);
    insert_if_nonempty(props, "category", &page.category);
    insert_if_nonempty(props, "name", &page.name);
    insert_if_nonempty(props, "referrer", &page.referrer);

    if !page.keywords.is_empty() {
        if let Ok(serialized_keywords) = serde_json::to_string(&page.keywords) {
            props.insert("keywords".into(), serialized_keywords);
        }
    }

    for (k, v) in &page.properties {
        insert_if_nonempty(props, k, v);
    }
}

fn enrich_with_campaign_context(
    props: &mut HashMap<String, String>,
    campaign: &crate::exports::edgee::components::data_collection::Campaign,
) {
    insert_if_nonempty(props, "campaign_name", &campaign.name);
    insert_if_nonempty(props, "campaign_source", &campaign.source);
    insert_if_nonempty(props, "campaign_medium", &campaign.medium);
    insert_if_nonempty(props, "campaign_term", &campaign.term);
    insert_if_nonempty(props, "campaign_content", &campaign.content);
    insert_if_nonempty(props, "campaign_creative_format", &campaign.creative_format);
    insert_if_nonempty(
        props,
        "campaign_marketing_tactic",
        &campaign.marketing_tactic,
    );
}

fn enrich_with_session_context(
    props: &mut HashMap<String, String>,
    session: &crate::exports::edgee::components::data_collection::Session,
) {
    insert_if_nonempty(props, "session_id", &session.session_id);
    insert_if_nonempty(props, "previous_session_id", &session.previous_session_id);
    props.insert("session_count".into(), session.session_count.to_string());
    props.insert("session_start".into(), session.session_start.to_string());
    props.insert("first_seen".into(), session.first_seen.to_string());
    props.insert("last_seen".into(), session.last_seen.to_string());
}

fn build_mixpanel_request(
    event: &Event,
    settings: &Settings,
    name: &str,
    properties: HashMap<String, String>,
) -> Result<EdgeeRequest, String> {
    let mut props = serde_json::Map::new();

    let user = &event.context.user;
    let distinct_id = if user.user_id.trim().is_empty() {
        user.edgee_id.clone()
    } else {
        user.user_id.clone()
    };

    props.insert("token".into(), settings.api_secret.clone().into());
    props.insert("distinct_id".into(), distinct_id.into());
    props.insert("time".into(), serde_json::json!(event.timestamp));
    props.insert("$insert_id".into(), serde_json::json!(event.uuid.clone()));

    for (k, v) in properties {
        props.insert(k, v.into());
    }

    let event_obj = serde_json::json!({
        "event": name,
        "properties": props
    });

    let payload = serde_json::json!([event_obj]);

    let mut url = format!("https://{}.mixpanel.com/import?strict=1", settings.region);
    if let Some(id) = &settings.project_id {
        url.push_str(&format!("&project_id={id}"));
    }

    let encoded = STANDARD.encode(format!("{}:", settings.api_secret).as_bytes());
    let auth = format!("Basic {encoded}");

    Ok(EdgeeRequest {
        method: HttpMethod::Post,
        url,
        headers: vec![
            ("Content-Type".into(), "application/json".into()),
            ("Accept".into(), "application/json".into()),
            ("Authorization".into(), auth),
        ],
        body: payload.to_string(),
        forward_client_headers: false,
    })
}

fn build_mixpanel_user_request(
    settings: &Settings,
    distinct_id: String,
    props: HashMap<String, String>,
) -> Result<EdgeeRequest, String> {
    let set_props: serde_json::Map<String, serde_json::Value> = props
        .into_iter()
        .map(|(k, v)| (k, serde_json::Value::String(v)))
        .collect();

    let payload = serde_json::json!([{
        "$distinct_id": distinct_id,
        "$token": settings.project_token,
        "$set": set_props
    }]);

    let url = format!("https://{}.mixpanel.com/engage", settings.region);

    Ok(EdgeeRequest {
        method: HttpMethod::Post,
        url,
        headers: vec![
            ("Content-Type".into(), "application/json".into()),
            ("Accept".into(), "application/json".into()),
        ],
        body: payload.to_string(),
        forward_client_headers: true,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::exports::edgee::components::data_collection::{
        Campaign, Client, Context, Data, EventType, PageData, Session, UserData,
    };
    use exports::edgee::components::data_collection::Consent;
    use uuid::Uuid;

    fn sample_user_data(edgee_id: String) -> UserData {
        UserData {
            user_id: "123".to_string(),
            anonymous_id: "456".to_string(),
            edgee_id,
            properties: vec![
                ("prop1".to_string(), "value1".to_string()),
                ("prop2".to_string(), "10".to_string()),
            ],
        }
    }

    fn sample_context(edgee_id: String, locale: String, session_start: bool) -> Context {
        Context {
            page: sample_page_data(),
            user: sample_user_data(edgee_id),
            client: Client {
                city: "Paris".to_string(),
                ip: "192.168.0.1".to_string(),
                locale,
                timezone: "CET".to_string(),
                user_agent: "Chrome".to_string(),
                user_agent_architecture: "x86".to_string(),
                user_agent_bitness: "64".to_string(),
                user_agent_full_version_list: "abc".to_string(),
                user_agent_version_list: "abc".to_string(),
                user_agent_mobile: "mobile".to_string(),
                user_agent_model: "don't know".to_string(),
                os_name: "MacOS".to_string(),
                os_version: "latest".to_string(),
                screen_width: 1024,
                screen_height: 768,
                screen_density: 2.0,
                continent: "Europe".to_string(),
                country_code: "FR".to_string(),
                country_name: "France".to_string(),
                region: "West Europe".to_string(),
            },
            campaign: Campaign {
                name: "random".to_string(),
                source: "random".to_string(),
                medium: "random".to_string(),
                term: "random".to_string(),
                content: "random".to_string(),
                creative_format: "random".to_string(),
                marketing_tactic: "random".to_string(),
            },
            session: Session {
                session_id: "random".to_string(),
                previous_session_id: "random".to_string(),
                session_count: 2,
                session_start,
                first_seen: 123,
                last_seen: 123,
            },
        }
    }

    fn sample_page_data() -> PageData {
        PageData {
            name: "page name".to_string(),
            category: "category".to_string(),
            keywords: vec!["value1".to_string(), "value2".into()],
            title: "page title".to_string(),
            url: "https://example.com/full-url?test=1".to_string(),
            path: "/full-path".to_string(),
            search: "?test=1".to_string(),
            referrer: "https://example.com/another-page".to_string(),
            properties: vec![
                ("prop1".to_string(), "value1".to_string()),
                ("prop2".to_string(), "10".to_string()),
                ("currency".to_string(), "USD".to_string()),
            ],
        }
    }

    fn sample_page_event(
        consent: Option<Consent>,
        edgee_id: String,
        locale: String,
        session_start: bool,
    ) -> Event {
        Event {
            uuid: Uuid::new_v4().to_string(),
            timestamp: 123,
            timestamp_millis: 123,
            timestamp_micros: 123,
            event_type: EventType::Page,
            data: Data::Page(sample_page_data()),
            context: sample_context(edgee_id, locale, session_start),
            consent,
        }
    }

    #[test]
    fn user_works_fine() {
        let event = sample_page_event(
            Some(Consent::Granted),
            "edgee-123".to_string(),
            "fr-FR".to_string(),
            false,
        );

        let settings = vec![
            ("api_secret".to_string(), "abc123".to_string()),
            ("project_token".to_string(), "tok123".to_string()),
            ("region".to_string(), "api-eu".to_string()),
            ("project_id".to_string(), "123456".to_string()),
        ];
        let result = Component::user(event, settings);

        assert!(result.is_ok());
        let req = result.unwrap();
        assert!(req.url.contains("https://api-eu.mixpanel.com/engage"));
        assert!(req.body.contains("\"$token\":\"tok123\""));
        assert!(req.body.contains("\"$distinct_id\":\"123\""));
    }

    #[test]
    fn track_works_fine() {
        let mut event = sample_page_event(
            Some(Consent::Granted),
            "edgee-456".to_string(),
            "en-GB".to_string(),
            false,
        );

        if let Data::Page(page_data) = event.data {
            event.data = Data::Track(
                crate::exports::edgee::components::data_collection::TrackData {
                    name: "Signup".to_string(),
                    properties: page_data.properties.clone(),
                    products: vec![],
                },
            );
        }

        let settings = vec![
            ("api_secret".to_string(), "abc123".to_string()),
            ("project_token".to_string(), "tok123".to_string()),
            ("region".to_string(), "api".to_string()),
            ("project_id".to_string(), "7891011".to_string()),
        ];
        let result = Component::track(event, settings);

        assert!(result.is_ok());
        let req = result.unwrap();
        assert!(req.url.contains("https://api.mixpanel.com/import"));
        assert!(req.url.contains("project_id=7891011"));
        assert!(req.body.contains("\"event\":\"Signup\""));
        assert!(req.body.contains("\"token\":\"abc123\""));
    }

    #[test]
    fn page_works_fine() {
        let event = sample_page_event(
            Some(Consent::Granted),
            "edgee-789".to_string(),
            "fr".to_string(),
            true,
        );

        let settings = vec![
            ("api_secret".to_string(), "abc123".to_string()),
            ("project_token".to_string(), "tok123".to_string()),
            ("region".to_string(), "api-in".to_string()),
            ("project_id".to_string(), "987654".to_string()),
        ];
        let result = Component::page(event, settings);

        assert!(result.is_ok());
        let req = result.unwrap();
        assert!(req.url.contains("https://api-in.mixpanel.com/import"));
        assert!(req.url.contains("project_id=987654"));
        assert!(req.body.contains("\"event\":\"Page View\""));
        assert!(req.body.contains("\"token\":\"abc123\""));
        assert!(req.body.contains("\"url\""));
    }
}
