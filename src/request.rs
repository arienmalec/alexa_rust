extern crate serde_derive;
extern crate serde;
extern crate serde_json;

use self::serde_derive::{Serialize, Deserialize};
use std::convert::From;
use std::collections::HashMap;

/// Request struct corresponding to the [Alexa spec](https://developer.amazon.com/docs/custom-skills/request-and-response-json-reference.html#request-body-parameters)
#[derive(Serialize,Deserialize,Debug,Clone)]
pub struct Request {
	version: String,
	session: Option<Session>,
    #[serde(rename = "request")]
	body: ReqBody,
	context: Context
}

#[derive(Serialize,Deserialize,Debug,Clone)]
pub struct Session {
    new: bool,
    #[serde(rename = "sessionId")]
    session_id: String,
    attributes: Option<HashMap<String,String>>,
    application: Application,
    user: User,
}

#[derive(Serialize,Deserialize,Debug,Clone)]
pub struct Application {
    #[serde(rename = "applicationId")]
    application_id: String
}

#[derive(Serialize,Deserialize,Debug,Clone)]
pub struct User {
    #[serde(rename = "userId")]
    user_id: String,
    #[serde(rename = "accessToken")]
    access_token: Option<String>
}

#[derive(Serialize,Deserialize,Debug,Clone)]
pub struct Device {
    #[serde(rename = "deviceId")]
    device_id: String
}


#[derive(Serialize,Deserialize,Debug,Clone)]
pub struct ReqBody {
    #[serde(rename = "type")]
    reqtype: String,
    #[serde(rename = "requestId")]
    request_id: String,
    timestamp: String,
    locale: String,
    intent: Option<Intent>,
    reason: Option<String>,
    #[serde(rename = "dialogState")]
    dialog_state: Option<String>
}

#[derive(Serialize,Deserialize,Debug,Clone)]
pub struct Context {
    #[serde(rename = "System")]
    system: System,
    #[serde(rename = "AudioPlayer")]
    audio_player: Option<AudioPlayer>,
}

#[derive(Serialize,Deserialize,Debug,Clone)]
pub struct System {
    #[serde(rename = "apiAccessToken")]
    api_access_token: String,
    device: Option<Device>,
    application: Option<Application>
}

#[derive(Serialize,Deserialize,Debug,Clone)]
pub struct AudioPlayer {
    token: String,
    #[serde(rename = "offsetInMilliseconds")]
    offset_in_milliseconds: u64,
    #[serde(rename = "playerActivity")]
    player_activity: String
}


#[derive(Serialize,Deserialize,Debug,Clone)]
pub struct Intent {
    name: String,
    #[serde(rename = "confirmationStatus")]
    confirmation_status: String,
    slots: Option<HashMap<String,Slot>>
}

impl Intent {
    fn get_slot(&self, name: &str) -> Option<&Slot> {
        self.slots.as_ref()?
            .get(name)
    }
}


#[derive(Serialize,Deserialize,Debug,Clone)]
pub struct Slot {
    name: String,
    value: String,
    #[serde(rename = "confirmationStatus")]
    confirmation_status: String,
    resolutions: Option<Resolution>
}

#[derive(Serialize,Deserialize,Debug,Clone)]
pub struct Resolution {
    #[serde(rename = "resolutionsPerAuthority")]
    resolutions_per_authority: Vec<ResolutionsPerAuthority>
}

#[derive(Serialize,Deserialize,Debug,Clone)]
pub struct ResolutionsPerAuthority {
    authority: String,
    status: Status,
    values: Vec<Value>
}

#[derive(Serialize,Deserialize,Debug,Clone)]
pub struct Status {
    code: String
}

#[derive(Serialize,Deserialize,Debug,Clone)]
pub struct Value {
    name: String,
    id: String,
}

/// Enumeration of Alexa intent types
/// Custom intents will be User enum values discrimiated by the `String` value
#[derive(Debug,PartialEq)]
pub enum IntentType {
    None,
    Help,
    Cancel,
    Fallback,
    LoopOff,
    LoopOn,
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
    User(String)
}

/// Alexa standard locales
#[derive(Debug,PartialEq)]
pub enum Locale {
	Italian,
	German,
	AustralianEnglish,
	CanadianEnglish,
    BritishEnglish,
    IndianEnglish,
    AmericanEnglish,
	Japanese,
    Unknown
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
            _ => false
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
            _       => Locale::Unknown
        }
    }
}

impl From<String> for Locale {
    fn from (s: String) -> Locale {
        Locale::from(s.as_str())
    }
}

impl Request {

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
                _ => IntentType::User(i.name.clone())
            }
        } else {
            IntentType::None
        }
    }

    /// retrieves the string value of named slot from the request, if it exists
    pub fn slot_value(&self, slot: &str) -> Option<String> {
        Some(
            self.body.intent.as_ref()?
                .get_slot(slot).as_ref()?
                .value.clone()
            )
    }

    /// retrieves the attribute value with the given key, if it exists
    pub fn attribute_value(&self, key: &str) -> Option<&String> {
        self.session.as_ref()?
            .attributes.as_ref()?
            .get(key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        let p: Result<Request,serde_json::Error> = self::serde_json::from_str(default_req());
        match p {
            Ok(req) => assert_eq!(req.version, "1.0"),
            Err(e) => panic!(e.to_string())
        }
    }

    #[test]
    fn test_locale() {
        let p: Result<Request,serde_json::Error> = self::serde_json::from_str(default_req());
        match p {
            Ok(req) => assert_eq!(req.locale(), Locale::AmericanEnglish),
            Err(e) => panic!(e.to_string())
        }
 
    }

    #[test]
    fn test_is_english() {
        let p: Result<Request,serde_json::Error> = self::serde_json::from_str(default_req());
        match p {
            Ok(req) => assert!(req.locale().is_english()),
            Err(e) => panic!(e.to_string())
        }
 
    }

    #[test]
    fn test_intent() {
        let p: Result<Request,serde_json::Error> = self::serde_json::from_str(default_req());
        match p {
            Ok(req) => assert_eq!(req.intent(),IntentType::User(String::from("hello"))),
            Err(e) => panic!(e.to_string())
        }
 
    }

    #[test]
    fn test_slot() {
        let p: Result<Request,serde_json::Error> = self::serde_json::from_str(req_with_slots());
        match p {
            Ok(req) => assert_eq!(req.slot_value("name"), Some(String::from("bob"))),
            Err(e) => panic!(e.to_string())
        }
    }

    #[test]
    fn test_attribute() {
        let p: Result<Request,serde_json::Error> = self::serde_json::from_str(default_req());
        match p {
            Ok(req) => {
                assert!(req.session.is_some());
                assert!(req.session.unwrap().attributes.is_some());
            },
            Err(e) => panic!(e.to_string())
        }
 
    }

    #[test]
    fn test_attribute_val() {
        let p: Result<Request,serde_json::Error> = self::serde_json::from_str(default_req());
        match p {
            Ok(req) => assert_eq!(req.attribute_value("lastSpeech"), Some(&String::from("Jupiter has the shortest day of all the planets"))),
            Err(e) => panic!(e.to_string())
        }
 
    }


    fn default_req () -> &'static str {
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