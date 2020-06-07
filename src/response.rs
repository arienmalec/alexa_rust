extern crate serde;
extern crate serde_derive;
extern crate serde_json;

use self::serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;

enum Version {
    V1_0,
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match *self {
            Version::V1_0 => "1.0",
        };
        write!(f, "{}", s)
    }
}

impl Response {
    /// Constructs a new response with only required elements
    pub fn new(should_end: bool) -> Response {
        Response {
            version: Version::V1_0.to_string(),
            session_attributes: None,
            body: ResBody {
                output_speech: None,
                card: None,
                reprompt: None,
                should_end_session: should_end,
            },
        }
    }

    /// Constructs a basic plain response with a simple card
    pub fn new_simple(title: &str, text: &str) -> Response {
        Response::simple(title, text)
    }

    /// Constructs a basic plain response with a simple card
    pub fn simple(title: &str, text: &str) -> Response {
        Response::new(true)
            .card(Card::simple(title, text))
            .speech(Speech::plain(text))
    }

    /// Constructs an empty response ending the session
    pub fn end() -> Response {
        Response::new(true)
    }

    /// adds a speach element to the response
    pub fn speech(mut self, speech: Speech) -> Self {
        self.body.output_speech = Some(speech);
        self
    }

    /// adds a card to the response
    pub fn card(mut self, card: Card) -> Self {
        self.body.card = Some(card);
        self
    }

    /// adds an attribute key/value pair to the response
    /// attributes can be read on the next request for basic state
    /// persistance
    pub fn add_attribute(&mut self, key: &str, val: &str) {
        if let Some(ref mut h) = self.session_attributes {
            let _ = h.insert(String::from(key), String::from(val));
        } else {
            let mut h = HashMap::new();
            h.insert(String::from(key), String::from(val));
            self.session_attributes = Some(h)
        }
    }
}

/// Response struct implementing the [Alexa JSON spec](https://developer.amazon.com/docs/custom-skills/request-and-response-json-reference.html#response-parameters)
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Response {
    version: String,
    #[serde(rename = "sessionAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    session_attributes: Option<HashMap<String, String>>,
    #[serde(rename = "response")]
    body: ResBody,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResBody {
    #[serde(rename = "outputSpeech")]
    #[serde(skip_serializing_if = "Option::is_none")]
    output_speech: Option<Speech>,
    #[serde(skip_serializing_if = "Option::is_none")]
    card: Option<Card>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reprompt: Option<Reprompt>,
    #[serde(rename = "shouldEndSession")]
    should_end_session: bool,
}

enum SpeechType {
    Plain,
    Ssml,
}

impl fmt::Display for SpeechType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match *self {
            SpeechType::Plain => "PlainText",
            SpeechType::Ssml => "SSML",
        };
        write!(f, "{}", s)
    }
}

/// Play behavior for output speech
pub enum PlayBehavior {
    Enqueue,
    ReplaceAll,
    ReplaceEnqueued,
}

impl fmt::Display for PlayBehavior {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match *self {
            PlayBehavior::Enqueue => "ENQUEUE",
            PlayBehavior::ReplaceAll => "REPLACE_ALL",
            PlayBehavior::ReplaceEnqueued => "REPLACE_ENQUEUED",
        };
        write!(f, "{}", s)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Speech {
    #[serde(rename = "type")]
    speech_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ssml: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "playBehavior")]
    play_behavior: Option<String>,
}

impl Speech {
    /// Constructs a plain text output speech
    pub fn plain(s: &str) -> Speech {
        Speech {
            speech_type: SpeechType::Plain.to_string(),
            text: Some(String::from(s)),
            ssml: None,
            play_behavior: None,
        }
    }

    /// Constructs an SSML output speech (with supplied SSML)
    pub fn ssml(s: &str) -> Speech {
        Speech {
            speech_type: SpeechType::Ssml.to_string(),
            ssml: Some(String::from(s)),
            text: None,
            play_behavior: None,
        }
    }

    /// Adds play behavior to a speech object
    pub fn play_behavior(&mut self, behavior: PlayBehavior) {
        self.play_behavior = Some(behavior.to_string());
    }
}

/// Types of cards for an Alexa response
#[allow(dead_code)]
pub enum CardType {
    Simple,
    Standard,
    LinkAccount,
    AskForPermission,
}

impl fmt::Display for CardType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match *self {
            CardType::Simple => "Simple",
            CardType::Standard => "Standard",
            CardType::LinkAccount => "LinkAccount",
            CardType::AskForPermission => "AskForPermissionsConsent",
        };
        write!(f, "{}", s)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Card {
    #[serde(rename = "type")]
    card_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    image: Option<Image>,
    #[serde(skip_serializing_if = "Option::is_none")]
    permissions: Option<Vec<String>>,
}

impl Card {
    /// Constructs a simple card for an Alexa repsonse object
    pub fn simple(title: &str, text: &str) -> Card {
        Card {
            card_type: CardType::Simple.to_string(),
            title: Some(String::from(title)),
            content: Some(String::from(text)),
            text: None,
            image: None,
            permissions: None,
        }
    }

    /// Constructs a standard card for an Alexa response object
    pub fn standard(title: &str, text: &str, image: Image) -> Card {
        Card {
            card_type: CardType::Standard.to_string(),
            title: Some(String::from(title)),
            content: None,
            text: Some(String::from(text)),
            image: Some(image),
            permissions: None,
        }
    }

    /// Constructs a link account card for the Alexa response object
    pub fn link_account() -> Card {
        Card {
            card_type: CardType::LinkAccount.to_string(),
            title: None,
            content: None,
            text: None,
            image: None,
            permissions: None,
        }
    }

    /// Constructs a permissions request card with the requested permissions
    pub fn ask_for_permission(permissions: Vec<String>) -> Card {
        Card {
            card_type: CardType::AskForPermission.to_string(),
            title: None,
            content: None,
            text: None,
            image: None,
            permissions: Some(permissions),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Reprompt {
    #[serde(rename = "outputSpeech")]
    output_speech: Speech,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Image {
    #[serde(rename = "smallImageUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    small_image_url: Option<String>,
    #[serde(rename = "largeImageUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    large_image_url: Option<String>,
}

impl Image {
    pub fn new() -> Image {
        Image::default()
    }

    pub fn small_image_url(mut self, url: String) -> Self {
        self.small_image_url = Some(url);
        self
    }

    pub fn large_image_url(mut self, url: String) -> Self {
        self.large_image_url = Some(url);
        self
    }
}

impl Default for Image {
    fn default() -> Self {
        Image {
            small_image_url: None,
            large_image_url: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        let r = Response::simple("hello, world", "hello, dude");
        assert_eq!(r.version, "1.0");
    }

    #[test]
    fn test_builder() {
        let mut res = Response::new(false)
            .card(Card::standard(
                "foo",
                "bar",
                Image {
                    small_image_url: Some(String::from("baaz.png")),
                    large_image_url: Some(String::from("baazLarge.png")),
                },
            ))
            .speech(Speech::plain("hello"));
        res.add_attribute("attr", "value");
        let t = res.body.card.as_ref().unwrap().title.as_ref().unwrap();
        assert_eq!(t, "foo");
        let txt = res.body.card.as_ref().unwrap().text.as_ref().unwrap();
        assert_eq!(txt, "bar");
        let attr = res
            .session_attributes
            .as_ref()
            .unwrap()
            .get("attr")
            .unwrap();
        assert_eq!(attr, "value");
    }

    #[test]
    fn test_builder_with_image_builder() {
        let mut res = Response::new(false)
            .card(Card::standard(
                "foo",
                "bar",
                Image::new()
                    .small_image_url(String::from("baaz.png"))
                    .large_image_url(String::from("baazLarge.png")),
            ))
            .speech(Speech::plain("hello"));
        res.add_attribute("attr", "value");
        let t = res.body.card.as_ref().unwrap().title.as_ref().unwrap();
        assert_eq!(t, "foo");
        let txt = res.body.card.as_ref().unwrap().text.as_ref().unwrap();
        assert_eq!(txt, "bar");
        let small_img = res
            .body
            .card
            .as_ref()
            .unwrap()
            .image
            .as_ref()
            .unwrap()
            .small_image_url
            .as_ref()
            .unwrap();
        let large_img = res
            .body
            .card
            .as_ref()
            .unwrap()
            .image
            .as_ref()
            .unwrap()
            .large_image_url
            .as_ref()
            .unwrap();

        assert_eq!(small_img, "baaz.png");
        assert_eq!(large_img, "baazLarge.png");

        let attr = res
            .session_attributes
            .as_ref()
            .unwrap()
            .get("attr")
            .unwrap();
        assert_eq!(attr, "value");
    }

    #[test]
    fn test_title() {
        let t = "hello, world";
        let r = Response::simple(t, "hello, dude");

        assert_eq!(r.body.card.unwrap().title.unwrap(), t);
    }

    #[test]
    fn test_text() {
        let t = "hello, dude";
        let r = Response::simple("hello,world", t);

        assert_eq!(r.body.card.unwrap().content.unwrap(), t);
    }

    #[test]
    fn test_should_end() {
        let r = Response::simple("foo", "bar");
        assert_eq!(r.body.should_end_session, true);
    }
}
