use crate::exports::edgee::components::data_collection::Data;
use crate::exports::edgee::components::data_collection::{Dict, EdgeeRequest, Event, HttpMethod};
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

impl Guest for Component {
    #[allow(unused_variables)]
    fn page(edgee_event: Event, settings_dict: Dict) -> Result<EdgeeRequest, String> {
        let settings = Settings::new(settings_dict).map_err(|e| e.to_string())?;

        let mut props = HashMap::new();

        if let Data::Page(ref data) = edgee_event.data {
            props.insert("url".into(), data.url.clone());
            props.insert("title".into(), data.title.clone());
            props.insert("path".into(), data.path.clone());
            props.insert("referrer".into(), data.referrer.clone());
            props.insert("category".into(), data.category.clone());
            props.insert("name".into(), data.name.clone());

            for (k, v) in &data.properties {
                props.insert(k.clone(), v.clone());
            }

            return build_mixpanel_request(&edgee_event, &settings.token, "Page View", props);
        }

        Err("Invalid event type for page".into())
    }


    #[allow(unused_variables)]
    fn track(edgee_event: Event, settings_dict: Dict) -> Result<EdgeeRequest, String> {
        let settings = Settings::new(settings_dict).map_err(|e| e.to_string())?;
        let mut props = HashMap::new();

        if let Data::Track(ref data) = edgee_event.data {
            for (k, v) in &data.properties {
                props.insert(k.clone(), v.clone());
            }
            return build_mixpanel_request(&edgee_event, &settings.token, &data.name, props);
        }

        Err("Invalid event type for track".into())
    }

    #[allow(unused_variables)]
    fn user(edgee_event: Event, settings_dict: Dict) -> Result<EdgeeRequest, String> {
        let settings = Settings::new(settings_dict).map_err(|e| e.to_string())?;

        let mut props = HashMap::new();
        let user = &edgee_event.context.user;

        props.insert("user_id".into(), user.user_id.clone());
        props.insert("anonymous_id".into(), user.anonymous_id.clone());

        for (k, v) in &user.properties {
            props.insert(k.clone(), v.clone());
        }

        build_mixpanel_request(&edgee_event, &settings.token, "User Identified", props)
    }

}

pub struct Settings {
    pub token: String,
}

impl Settings {
    pub fn new(settings_dict: Dict) -> anyhow::Result<Self> {
        let settings_map: HashMap<String, String> = settings_dict
            .iter()
            .map(|(key, value)| (key.to_string(), value.to_string()))
            .collect();

        let token = settings_map
            .get("mixpanel_token")
            .filter(|t| !t.trim().is_empty())
            .ok_or_else(|| anyhow::anyhow!("Missing or empty 'mixpanel_token' setting"))?
            .to_string();

        Ok(Self { token })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::exports::edgee::components::data_collection::{
        Campaign, Client, Context, Data, EventType, PageData, Session, UserData,
    };
    use exports::edgee::components::data_collection::Consent;
    use pretty_assertions::assert_eq;
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

        let settings = vec![("mixpanel_token".to_string(), "abc123".to_string())];
        let result = Component::user(event, settings);

        assert!(result.is_ok());
        let req = result.unwrap();
        assert_eq!(req.url, "https://api.mixpanel.com/track");
        assert!(req.body.contains("\"event\":\"User Identified\""));
        assert!(req.body.contains("\"token\":\"abc123\""));
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
            event.data = Data::Track(crate::exports::edgee::components::data_collection::TrackData {
                name: "Signup".to_string(),
                properties: page_data.properties.clone(),
                products: vec![],
            });
        }

        let settings = vec![("mixpanel_token".to_string(), "xyz456".to_string())];
        let result = Component::track(event, settings);

        assert!(result.is_ok());
        let req = result.unwrap();
        assert_eq!(req.url, "https://api.mixpanel.com/track");
        assert!(req.body.contains("\"event\":\"Signup\""));
        assert!(req.body.contains("\"token\":\"xyz456\""));
    }

    #[test]
    fn page_works_fine() {
        let event = sample_page_event(
            Some(Consent::Granted),
            "edgee-789".to_string(),
            "fr".to_string(),
            true,
        );
        let settings = vec![("mixpanel_token".to_string(), "token789".to_string())];
        let result = Component::page(event, settings);

        assert!(result.is_ok());
        let req = result.unwrap();
        assert_eq!(req.url, "https://api.mixpanel.com/track");
        assert!(req.body.contains("\"event\":\"Page View\""));
        assert!(req.body.contains("\"token\":\"token789\""));
        assert!(req.body.contains("\"url\""));
    }
}

fn build_mixpanel_request(event: &Event, token: &str, name: &str, properties: HashMap<String, String>) -> Result<EdgeeRequest, String> {
    let mut props = serde_json::Map::new();

    props.insert("token".into(), token.into());
    props.insert("distinct_id".into(), event.context.user.user_id.clone().into());
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

    log::info!("Sending Mixpanel event '{}': {}", name, payload.to_string());

    Ok(EdgeeRequest {
        method: HttpMethod::Post,
        url: "https://api.mixpanel.com/track".to_string(),
        headers: vec![
            ("Content-Type".into(), "application/json".into()),
            ("Accept".into(), "text/plain".into()),
        ],
        body: payload.to_string(),
        forward_client_headers: false,
    })
}

