extern crate serde;
extern crate serde_derive;
extern crate serde_json;

use self::serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;
use std::convert::From;

/// Request struct corresponding to the [Alexa spec](https://developer.amazon.com/docs/custom-skills/request-and-response-json-reference.html#request-body-parameters)
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Request {
    pub version: String,
    pub session: Option<Session>,
    #[serde(rename = "request")]
    pub body: ReqBody,
    pub context: Context,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Session {
    pub new: bool,
    #[serde(rename = "sessionId")]
    pub session_id: String,
    pub attributes: Option<HashMap<String, String>>,
    pub application: Application,
    pub user: User,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Application {
    #[serde(rename = "applicationId")]
    pub application_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    #[serde(rename = "userId")]
    pub user_id: String,
    #[serde(rename = "accessToken")]
    pub access_token: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Device {
    #[serde(rename = "deviceId")]
    pub device_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ReqBody {
    #[serde(rename = "type")]
    pub reqtype: String,
    #[serde(rename = "requestId")]
    pub request_id: String,
    pub timestamp: String,
    pub locale: String,
    pub intent: Option<Intent>,
    pub reason: Option<String>,
    #[serde(rename = "dialogState")]
    pub dialog_state: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Context {
    #[serde(rename = "System")]
    pub system: System,
    #[serde(rename = "AudioPlayer")]
    pub audio_player: Option<AudioPlayer>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct System {
    #[serde(rename = "apiAccessToken")]
    pub api_access_token: Option<String>,
    pub device: Option<Device>,
    pub application: Option<Application>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AudioPlayer {
    pub token: Option<String>,
    #[serde(rename = "offsetInMilliseconds")]
    pub offset_in_milliseconds: Option<u64>,
    #[serde(rename = "playerActivity")]
    pub player_activity: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Intent {
    pub name: String,
    #[serde(rename = "confirmationStatus")]
    pub confirmation_status: Option<String>,
    pub slots: Option<HashMap<String, Slot>>,
}

impl Intent {
    fn get_slot(&self, name: &str) -> Option<&Slot> {
        self.slots.as_ref()?.get(name)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Slot {
    pub name: String,
    pub value: String,
    #[serde(rename = "confirmationStatus")]
    pub confirmation_status: Option<String>,
    pub resolutions: Option<Resolution>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Resolution {
    #[serde(rename = "resolutionsPerAuthority")]
    pub resolutions_per_authority: Vec<ResolutionsPerAuthority>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResolutionsPerAuthority {
    pub authority: String,
    pub status: Status,
    pub values: Vec<ValueWrapper>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Status {
    pub code: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ValueWrapper {
    pub value: Value,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Value {
    pub name: String,
    pub id: String,
}

/// Enumeration of Alexa request types
/// Not comprehensive, ones not defined are put into the Other `String` value
#[derive(Debug, PartialEq)]
pub enum ReqType {
    LaunchRequest,
    IntentRequest,
    SessionEndedRequest,
    CanFulfillIntentRequest,
    Other(String),
}

impl<'a> From<&'a str> for ReqType {
    fn from(s: &'a str) -> ReqType {
        match s {
            "LaunchRequest" => ReqType::LaunchRequest,
            "IntentRequest" => ReqType::IntentRequest,
            "SessionEndedRequest" => ReqType::SessionEndedRequest,
            "CanFulfillIntentRequest" => ReqType::CanFulfillIntentRequest,
            _ => ReqType::Other(s.to_string()),
        }
    }
}

impl From<String> for ReqType {
    fn from(s: String) -> ReqType {
        ReqType::from(s.as_str())
    }
}

/// Enumeration of Alexa intent types
/// Custom intents will be User enum values discrimiated by the `String` value
#[derive(Debug, PartialEq)]
pub enum IntentType {
    None,
    Help,
    Cancel,
    Fallback,
    LoopOff,
    LoopOn,
    NavigateHome,
    Next,
    No,
    Pause,
    Previous,
    Repeat,
    Resume,
    Select,
    ShuffleOn,
    ShuffleOff,
    StartOver,
    Stop,
    Yes,
    User(String),
}

/// Alexa standard locales
#[derive(Debug, PartialEq)]
pub enum Locale {
    Italian,
    German,
    AustralianEnglish,
    CanadianEnglish,
    BritishEnglish,
    IndianEnglish,
    AmericanEnglish,
    Japanese,
    Unknown,
}

impl Locale {
    /// returns true for all English speaking locals
    pub fn is_english(&self) -> bool {
        match *self {
            Locale::AmericanEnglish => true,
            Locale::AustralianEnglish => true,
            Locale::CanadianEnglish => true,
            Locale::BritishEnglish => true,
            Locale::IndianEnglish => true,
            _ => false,
        }
    }
}

impl<'a> From<&'a str> for Locale {
    fn from(s: &'a str) -> Locale {
        match s {
            "it-IT" => Locale::Italian,
            "de-DE" => Locale::German,
            "en-AU" => Locale::AustralianEnglish,
            "en-CA" => Locale::CanadianEnglish,
            "en-GB" => Locale::BritishEnglish,
            "en-IN" => Locale::IndianEnglish,
            "en-US" => Locale::AmericanEnglish,
            "ja-JP" => Locale::Japanese,
            _ => Locale::Unknown,
        }
    }
}

impl From<String> for Locale {
    fn from(s: String) -> Locale {
        Locale::from(s.as_str())
    }
}

impl Request {
    /// Extracts the request type from the request
    pub fn reqtype(&self) -> ReqType {
        ReqType::from(&*self.body.reqtype)
    }

    /// Extracts the locale from the request
    pub fn locale(&self) -> Locale {
        Locale::from(&*self.body.locale)
    }

    /// Extracts the intent from the request
    pub fn intent(&self) -> IntentType {
        if let Some(ref i) = self.body.intent {
            match i.name.as_str() {
                "AMAZON.HelpIntent" => IntentType::Help,
                "AMAZON.CancelIntent" => IntentType::Cancel,
                "AMAZON.FallbackIntent" => IntentType::Fallback,
                "AMAZON.LoopOffIntent" => IntentType::LoopOff,
                "AMAZON.LoopOnIntent" => IntentType::LoopOn,
                "AMAZON.NavigateHomeIntent" => IntentType::NavigateHome,
                "AMAZON.NextIntent" => IntentType::Next,
                "AMAZON.NoIntent" => IntentType::No,
                "AMAZON.PauseIntent" => IntentType::Pause,
                "AMAZON.PreviousIntent" => IntentType::Previous,
                "AMAZON.RepeatIntent" => IntentType::Repeat,
                "AMAZON.ResumeIntent" => IntentType::Resume,
                "AMAZON.SelectIntent" => IntentType::Select,
                "AMAZON.ShuffleOffIntent" => IntentType::ShuffleOff,
                "AMAZON.ShuffleOnIntent" => IntentType::ShuffleOn,
                "AMAZON.StartOverIntent" => IntentType::StartOver,
                "AMAZON.StopIntent" => IntentType::Stop,
                "AMAZON.YesIntent" => IntentType::Yes,
                _ => IntentType::User(i.name.clone()),
            }
        } else {
            IntentType::None
        }
    }

    /// retrieves the string value of named slot from the request, if it exists
    pub fn slot_value(&self, slot: &str) -> Option<String> {
        Some(
            self.body
                .intent
                .as_ref()?
                .get_slot(slot)
                .as_ref()?
                .value
                .clone(),
        )
    }

    /// retrieves the attribute value with the given key, if it exists
    pub fn attribute_value(&self, key: &str) -> Option<&String> {
        self.session.as_ref()?.attributes.as_ref()?.get(key)
    }

    /// returns whether or not this is a new request
    pub fn is_new(&self) -> bool {
        match &self.session {
            Some(s) => s.new,
            None => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        let p: Result<Request, serde_json::Error> = self::serde_json::from_str(default_req());
        match p {
            Ok(req) => assert_eq!(req.version, "1.0"),
            Err(e) => panic!(e.to_string()),
        }
    }

    #[test]
    fn test_locale() {
        let p: Result<Request, serde_json::Error> = self::serde_json::from_str(default_req());
        match p {
            Ok(req) => assert_eq!(req.locale(), Locale::AmericanEnglish),
            Err(e) => panic!(e.to_string()),
        }
    }

    #[test]
    fn test_is_english() {
        let p: Result<Request, serde_json::Error> = self::serde_json::from_str(default_req());
        match p {
            Ok(req) => assert!(req.locale().is_english()),
            Err(e) => panic!(e.to_string()),
        }
    }

    #[test]
    fn test_intent() {
        let p: Result<Request, serde_json::Error> = self::serde_json::from_str(default_req());
        match p {
            Ok(req) => assert_eq!(req.intent(), IntentType::User(String::from("hello"))),
            Err(e) => panic!(e.to_string()),
        }
    }

    #[test]
    fn test_slot() {
        let p: Result<Request, serde_json::Error> = self::serde_json::from_str(req_with_slots());
        match p {
            Ok(req) => assert_eq!(req.slot_value("name"), Some(String::from("bob"))),
            Err(e) => panic!(e.to_string()),
        }
    }

    #[test]
    fn test_attribute() {
        let p: Result<Request, serde_json::Error> = self::serde_json::from_str(default_req());
        match p {
            Ok(req) => {
                assert!(req.session.is_some());
                assert!(req.session.unwrap().attributes.is_some());
            }
            Err(e) => panic!(e.to_string()),
        }
    }

    #[test]
    fn test_attribute_val() {
        let p: Result<Request, serde_json::Error> = self::serde_json::from_str(default_req());
        match p {
            Ok(req) => assert_eq!(
                req.attribute_value("lastSpeech"),
                Some(&String::from(
                    "Jupiter has the shortest day of all the planets"
                ))
            ),
            Err(e) => panic!(e.to_string()),
        }
    }

    fn default_req() -> &'static str {
        r#"{
	"version": "1.0",
	"session": {
		"new": true,
		"sessionId": "amzn1.echo-api.session.abc123",
		"application": {
			"applicationId": "amzn1.ask.skill.myappid"
		},
        "attributes": {
            "lastSpeech": "Jupiter has the shortest day of all the planets"
        },
		"user": {
			"userId": "amzn1.ask.account.theuserid"
		}
	},
	"context": {
		"System": {
			"application": {
				"applicationId": "amzn1.ask.skill.myappid"
			},
			"user": {
				"userId": "amzn1.ask.account.theuserid"
			},
			"device": {
				"deviceId": "amzn1.ask.device.superfakedevice",
				"supportedInterfaces": {}
			},
			"apiEndpoint": "https://api.amazonalexa.com",
			"apiAccessToken": "53kr14t.k3y.d4t4-otherstuff"
		},
		"Viewport": {
			"experiences": [
				{
					"arcMinuteWidth": 246,
					"arcMinuteHeight": 144,
					"canRotate": false,
					"canResize": false
				}
			],
			"shape": "RECTANGLE",
			"pixelWidth": 1024,
			"pixelHeight": 600,
			"dpi": 160,
			"currentPixelWidth": 1024,
			"currentPixelHeight": 600,
			"touch": [
				"SINGLE"
			]
		}
	},
	"request": {
		"type": "IntentRequest",
		"requestId": "amzn1.echo-api.request.b8b49fde-4370-423f-bbb0-dc7305b788a0",
		"timestamp": "2018-12-03T00:33:58Z",
		"locale": "en-US",
		"intent": {
			"name": "hello",
			"confirmationStatus": "NONE"
		}
	}
}"#
    }

    fn req_with_slots() -> &'static str {
        r#"{
	"version": "1.0",
	"session": {
		"new": true,
		"sessionId": "amzn1.echo-api.session.blahblahblah",
		"application": {
			"applicationId": "amzn1.ask.skill.testappliction"
		},
		"user": {
			"userId": "amzn1.ask.account.longstringuseridentifier"
		}
	},
	"context": {
		"Display": {},
		"System": {
			"application": {
				"applicationId": "amzn1.ask.skill.tehappz"
			},
			"user": {
				"userId": "amzn1.ask.account.longstringuseridentifier"
			},
			"device": {
				"deviceId": "amzn1.ask.device.testdevice",
				"supportedInterfaces": {
					"Display": {
						"templateVersion": "1.0",
						"markupVersion": "1.0"
					}
				}
			},
			"apiEndpoint": "https://api.amazonalexa.com",
			"apiAccessToken": "teh.token.with-long-string-more-more-more-more"
		},
		"Viewport": {
			"experiences": [
				{
					"arcMinuteWidth": 246,
					"arcMinuteHeight": 144,
					"canRotate": false,
					"canResize": false
				}
			],
			"shape": "RECTANGLE",
			"pixelWidth": 1024,
			"pixelHeight": 600,
			"dpi": 160,
			"currentPixelWidth": 1024,
			"currentPixelHeight": 600,
			"touch": [
				"SINGLE"
			]
		}
	},
	"request": {
		"type": "IntentRequest",
		"requestId": "amzn1.echo-api.request.id",
		"timestamp": "2018-12-08T05:37:32Z",
		"locale": "en-US",
		"intent": {
			"name": "hello",
			"confirmationStatus": "NONE",
			"slots": {
				"name": {
					"name": "name",
					"value": "bob",
					"confirmationStatus": "NONE",
					"source": "USER"
				}
			}
		}
	}
}"#
    }
}
