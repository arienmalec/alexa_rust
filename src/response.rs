extern crate serde_derive;
extern crate serde;
extern crate serde_json;

use self::serde_derive::{Serialize, Deserialize};
use std::collections::HashMap;
use std::fmt;

impl Response {
    pub fn new_simple(title: &str, text: &str) -> Response {
        Response {
            version: String::from("1.0"),
            session_attributes: None,
            body: ResBody {
                output_speech: Some(Speech::plain(text)),
                card: Some(Card::simple(title, text)),
                reprompt: None,
                should_end_session: true
            }
        }
    }

    pub fn end() -> Response {
        Response {
            version: String::from("1.0"),
            session_attributes: None,
            body: ResBody {
                output_speech: None,
                card: None,
                reprompt: None,
                should_end_session: true
            }
        }
    }
}

#[derive(Serialize,Deserialize,Debug,Clone)]
pub struct Response {
    version: String,
    #[serde(rename = "sessionAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    session_attributes: Option<HashMap<String,String>>,
    #[serde(rename = "response")]
    body: ResBody
}

#[derive(Serialize,Deserialize,Debug,Clone)]
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
    Ssml
}

impl fmt::Display for SpeechType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match *self {
            SpeechType::Plain => "PlainText",
            SpeechType::Ssml => "SSML"
        };
        write!(f,"{}",s)
    }
}

pub enum PlayBehavior {
    Enqueue,
    ReplaceAll,
    ReplaceEnqueued
}

impl fmt::Display for PlayBehavior {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match *self {
            PlayBehavior::Enqueue         => "ENQUEUE",
            PlayBehavior::ReplaceAll      => "REPLACE_ALL",
            PlayBehavior::ReplaceEnqueued => "REPLACE_ENQUEUED",
        };
        write!(f,"{}",s)
    }
}


#[derive(Serialize,Deserialize,Debug,Clone)]
pub struct Speech {
    #[serde(rename = "type")]
    speech_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ssml: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "playBehavior")]
    play_behavior: Option<String>
}

impl Speech {
    pub fn plain(s: &str) -> Speech {
        Speech {
            speech_type: SpeechType::Plain.to_string(),
            text: Some(String::from(s)),
            ssml: None,
            play_behavior: None
        }
    }

    pub fn ssml(s: &str) -> Speech {
        Speech {
            speech_type: SpeechType::Ssml.to_string(),
            ssml: Some(String::from(s)),
            text: None,
            play_behavior: None
        }
    }

    pub fn play_behavior(&mut self, behavior: PlayBehavior) {
        self.play_behavior = Some(behavior.to_string());
    }
}

#[allow(dead_code)]
enum CardType {
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
            CardType::AskForPermission => "AskForPermisson"
        };
        write!(f,"{}",s)
    }
}

#[derive(Serialize,Deserialize,Debug,Clone)]
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
}

impl Card {
    pub fn simple(title: &str, text: &str) -> Card {
        Card {
            card_type: CardType::Simple.to_string(),
            title: Some(String::from(title)),
            content: Some(String::from(text)),
            text: None,
            image: None,
        }
    }
}

#[derive(Serialize,Deserialize,Debug,Clone)]
pub struct Reprompt {
    #[serde(rename = "outputSpeech")]
    output_speech: Speech,
}

#[derive(Serialize,Deserialize,Debug,Clone)]
pub struct Image {
    #[serde(rename = "smallImageUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    small_image_url: Option<String>,
    #[serde(rename = "largeImageUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    large_image_url: Option<String>,
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        let r = Response::new_simple("hello, world", "hello, dude");
        assert_eq!(r.version, "1.0")  ; 
    }


    #[test]
    fn test_title() {
        let t = "hello, world";
        let r = Response::new_simple(t, "hello, dude");

        assert_eq!(r.body.card.unwrap().title.unwrap(), t); 
    }

    #[test]
    fn test_text() {
        let t = "hello, dude";
        let r = Response::new_simple("hello,world", t);

        assert_eq!(r.body.card.unwrap().content.unwrap(), t); 
    }

    #[test]
    fn test_should_end() {
        let r = Response::new_simple("foo", "bar");
        assert_eq!(r.body.should_end_session,true);
    }

}
