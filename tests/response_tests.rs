#![recursion_limit = "512"]

use duckduckgo::response::{Developer, Maintainer, Meta, Response, SrcOptions};
use serde_json::json;

fn deserialize_response(value: serde_json::Value) -> Response {
    serde_json::from_value(value).expect("Failed to deserialize Response")
}

fn deserialize_meta(value: serde_json::Value) -> Meta {
    serde_json::from_value(value).expect("Failed to deserialize Meta")
}

#[test]
fn test_response_minimal_deserialization() {
    let json = json!({
        "Abstract": "",
        "AbstractSource": "",
        "AbstractText": "",
        "AbstractURL": "",
        "Answer": "",
        "AnswerType": "",
        "Definition": "",
        "DefinitionSource": "",
        "DefinitionURL": "",
        "Entity": "",
        "Heading": "",
        "Image": "",
        "ImageHeight": 0,
        "ImageIsLogo": 0,
        "ImageWidth": 0,
        "Infobox": "",
        "Redirect": "",
        "RelatedTopics": [],
        "Results": [],
        "Type": "D",
        "meta": null
    });

    let response = deserialize_response(json);
    assert_eq!(response.r#type, "D");
    assert!(response.related_topics.is_empty());
    assert!(response.results.is_empty());
    assert!(response.meta.is_none());
}

#[test]
fn test_response_heading_field() {
    let json = json!({
        "Abstract": "",
        "AbstractSource": "",
        "AbstractText": "",
        "AbstractURL": "",
        "Answer": "",
        "AnswerType": "",
        "Definition": "",
        "DefinitionSource": "",
        "DefinitionURL": "",
        "Entity": "",
        "Heading": "Rust",
        "Image": "",
        "ImageHeight": 0,
        "ImageIsLogo": 0,
        "ImageWidth": 0,
        "Infobox": "",
        "Redirect": "",
        "RelatedTopics": [],
        "Results": [],
        "Type": "D",
        "meta": null
    });

    let response = deserialize_response(json);
    assert_eq!(response.heading.as_deref(), Some("Rust"));
}

#[test]
fn test_response_abstract_fields() {
    let json = json!({
        "Abstract": "Short abstract.",
        "AbstractSource": "Wikipedia",
        "AbstractText": "Detailed abstract text.",
        "AbstractURL": "https://en.wikipedia.org/wiki/Rust",
        "Answer": "",
        "AnswerType": "",
        "Definition": "",
        "DefinitionSource": "",
        "DefinitionURL": "",
        "Entity": "programming language",
        "Heading": "Rust",
        "Image": "/i/rust.png",
        "ImageHeight": 100,
        "ImageIsLogo": 0,
        "ImageWidth": 100,
        "Infobox": "",
        "Redirect": "",
        "RelatedTopics": [],
        "Results": [],
        "Type": "A",
        "meta": null
    });

    let response = deserialize_response(json);
    assert_eq!(response.r#abstract.as_deref(), Some("Short abstract."));
    assert_eq!(response.abstract_source.as_deref(), Some("Wikipedia"));
    assert_eq!(
        response.abstract_text.as_deref(),
        Some("Detailed abstract text.")
    );
    assert_eq!(
        response.abstract_url.as_deref(),
        Some("https://en.wikipedia.org/wiki/Rust")
    );
    assert_eq!(response.entity.as_deref(), Some("programming language"));
    assert_eq!(response.image.as_deref(), Some("/i/rust.png"));
    assert_eq!(response.r#type, "A");
}

#[test]
fn test_response_answer_fields() {
    let json = json!({
        "Abstract": "",
        "AbstractSource": "",
        "AbstractText": "",
        "AbstractURL": "",
        "Answer": "42",
        "AnswerType": "calc",
        "Definition": "A number.",
        "DefinitionSource": "Math",
        "DefinitionURL": "https://example.com/42",
        "Entity": "",
        "Heading": "",
        "Image": "",
        "ImageHeight": 0,
        "ImageIsLogo": 0,
        "ImageWidth": 0,
        "Infobox": "",
        "Redirect": "",
        "RelatedTopics": [],
        "Results": [],
        "Type": "A",
        "meta": null
    });

    let response = deserialize_response(json);
    assert_eq!(response.answer.as_deref(), Some("42"));
    assert_eq!(response.answer_type.as_deref(), Some("calc"));
    assert_eq!(response.definition.as_deref(), Some("A number."));
    assert_eq!(response.definition_source.as_deref(), Some("Math"));
    assert_eq!(
        response.definition_url.as_deref(),
        Some("https://example.com/42")
    );
}

#[test]
fn test_response_redirect_field() {
    let json = json!({
        "Abstract": "",
        "AbstractSource": "",
        "AbstractText": "",
        "AbstractURL": "",
        "Answer": "",
        "AnswerType": "",
        "Definition": "",
        "DefinitionSource": "",
        "DefinitionURL": "",
        "Entity": "",
        "Heading": "",
        "Image": "",
        "ImageHeight": 0,
        "ImageIsLogo": 0,
        "ImageWidth": 0,
        "Infobox": "",
        "Redirect": "https://duckduckgo.com/rust",
        "RelatedTopics": [],
        "Results": [],
        "Type": "R",
        "meta": null
    });

    let response = deserialize_response(json);
    assert_eq!(
        response.redirect.as_deref(),
        Some("https://duckduckgo.com/rust")
    );
    assert_eq!(response.r#type, "R");
}

#[test]
fn test_response_related_topics_deserialization() {
    let json = json!({
        "Abstract": "",
        "AbstractSource": "",
        "AbstractText": "",
        "AbstractURL": "",
        "Answer": "",
        "AnswerType": "",
        "Definition": "",
        "DefinitionSource": "",
        "DefinitionURL": "",
        "Entity": "",
        "Heading": "Rust",
        "Image": "",
        "ImageHeight": 0,
        "ImageIsLogo": 0,
        "ImageWidth": 0,
        "Infobox": "",
        "Redirect": "",
        "RelatedTopics": [
            {
                "FirstURL": "https://duckduckgo.com/Rust",
                "Icon": { "Height": "", "URL": "/i/rust.jpg", "Width": "" },
                "Result": "<a href=\"https://duckduckgo.com/Rust\">Rust</a>",
                "Text": "Rust An iron oxide."
            }
        ],
        "Results": [],
        "Type": "D",
        "meta": null
    });

    let response = deserialize_response(json);
    assert_eq!(response.related_topics.len(), 1);

    let topic = &response.related_topics[0];
    assert_eq!(
        topic.first_url.as_deref(),
        Some("https://duckduckgo.com/Rust")
    );
    assert_eq!(topic.text.as_deref(), Some("Rust An iron oxide."));

    let icon = topic.icon.as_ref().expect("Expected icon");
    assert_eq!(icon.url, "/i/rust.jpg");
}

#[test]
fn test_meta_basic_fields() {
    let json = json!({
        "description": "Wikipedia",
        "dev_milestone": "live",
        "example_query": "nikola tesla",
        "id": "wikipedia_fathead",
        "js_callback_name": "wikipedia",
        "name": "Wikipedia",
        "perl_module": "DDG::Fathead::Wikipedia",
        "production_state": "online",
        "repo": "fathead",
        "signal_from": "wikipedia_fathead",
        "src_domain": "en.wikipedia.org",
        "src_id": 1,
        "src_name": "Wikipedia",
        "status": "live",
        "tab": "About",
        "topic": ["productivity"],
        "unsafe": 0
    });

    let meta: Meta = deserialize_meta(json);
    assert_eq!(meta.description.as_deref(), Some("Wikipedia"));
    assert_eq!(meta.dev_milestone.as_deref(), Some("live"));
    assert_eq!(meta.example_query.as_deref(), Some("nikola tesla"));
    assert_eq!(meta.id.as_deref(), Some("wikipedia_fathead"));
    assert_eq!(meta.name.as_deref(), Some("Wikipedia"));
    assert_eq!(meta.production_state.as_deref(), Some("online"));
    assert_eq!(meta.repo.as_deref(), Some("fathead"));
    assert_eq!(meta.src_domain.as_deref(), Some("en.wikipedia.org"));
    assert_eq!(meta.src_id, Some(1));
    assert_eq!(meta.src_name.as_deref(), Some("Wikipedia"));
    assert_eq!(meta.status.as_deref(), Some("live"));
    assert_eq!(meta.tab.as_deref(), Some("About"));
    assert!(meta.unsafe_flag.is_some());
    let topics = meta.topic.expect("Expected topics");
    assert_eq!(topics, vec!["productivity"]);
}

#[test]
fn test_meta_with_null_optional_fields() {
    let json = json!({
        "attribution": null,
        "blockgroup": null,
        "created_date": null,
        "description": "Wikipedia",
        "designer": null,
        "dev_date": null,
        "dev_milestone": "live",
        "developer": null,
        "example_query": "nikola tesla",
        "id": "wikipedia_fathead",
        "is_stackexchange": null,
        "js_callback_name": "wikipedia",
        "live_date": null,
        "maintainer": null,
        "name": "Wikipedia",
        "perl_module": "DDG::Fathead::Wikipedia",
        "producer": null,
        "production_state": "online",
        "repo": "fathead",
        "signal_from": "wikipedia_fathead",
        "src_domain": "en.wikipedia.org",
        "src_id": 1,
        "src_name": "Wikipedia",
        "src_options": null,
        "src_url": null,
        "status": "live",
        "tab": "About",
        "topic": ["productivity"],
        "unsafe": 0
    });

    let meta: Meta = deserialize_meta(json);
    assert!(meta.attribution.is_none());
    assert!(meta.blockgroup.is_none());
    assert!(meta.designer.is_none());
    assert!(meta.developer.is_none());
    assert!(meta.maintainer.is_none());
    assert!(meta.src_options.is_none());
    assert!(meta.src_url.is_none());
}

#[test]
fn test_meta_developer_deserialization() {
    let json = json!({
        "name": "DDG Team",
        "type": "ddg",
        "url": "http://www.duckduckhack.com"
    });

    let dev: Developer = serde_json::from_value(json).expect("Failed to deserialize Developer");
    assert_eq!(dev.name.as_deref(), Some("DDG Team"));
    assert_eq!(dev.developer_type.as_deref(), Some("ddg"));
    assert_eq!(dev.url.as_deref(), Some("http://www.duckduckhack.com"));
}

#[test]
fn test_meta_maintainer_deserialization() {
    let json = json!({ "github": "duckduckgo" });

    let maintainer: Maintainer =
        serde_json::from_value(json).expect("Failed to deserialize Maintainer");
    assert_eq!(maintainer.github.as_deref(), Some("duckduckgo"));
}

#[test]
fn test_src_options_deserialization() {
    let json = json!({
        "directory": "",
        "is_fanon": 0,
        "is_mediawiki": 1,
        "is_wikipedia": 1,
        "language": "en",
        "min_abstract_length": "20",
        "skip_abstract": 0,
        "skip_abstract_paren": 0,
        "skip_end": "0",
        "skip_icon": 0,
        "skip_image_name": 0,
        "skip_qr": "",
        "source_skip": "",
        "src_info": ""
    });

    let opts: SrcOptions = serde_json::from_value(json).expect("Failed to deserialize SrcOptions");
    assert_eq!(opts.language.as_deref(), Some("en"));
    assert_eq!(opts.min_abstract_length.as_deref(), Some("20"));
}

#[test]
fn test_response_full_meta_integration() {
    let json = json!({
        "Abstract": "",
        "AbstractSource": "Wikipedia",
        "AbstractText": "",
        "AbstractURL": "https://en.wikipedia.org/wiki/Rust_(disambiguation)",
        "Answer": "",
        "AnswerType": "",
        "Definition": "",
        "DefinitionSource": "",
        "DefinitionURL": "",
        "Entity": "",
        "Heading": "Rust",
        "Image": "",
        "ImageHeight": 0,
        "ImageIsLogo": 0,
        "ImageWidth": 0,
        "Infobox": "",
        "Redirect": "",
        "RelatedTopics": [],
        "Results": [],
        "Type": "D",
        "meta": {
            "attribution": null,
            "blockgroup": null,
            "created_date": null,
            "description": "Wikipedia",
            "designer": null,
            "dev_date": null,
            "dev_milestone": "live",
            "developer": [
                {
                    "name": "DDG Team",
                    "type": "ddg",
                    "url": "http://www.duckduckhack.com"
                }
            ],
            "example_query": "nikola tesla",
            "id": "wikipedia_fathead",
            "is_stackexchange": null,
            "js_callback_name": "wikipedia",
            "live_date": null,
            "maintainer": { "github": "duckduckgo" },
            "name": "Wikipedia",
            "perl_module": "DDG::Fathead::Wikipedia",
            "producer": null,
            "production_state": "online",
            "repo": "fathead",
            "signal_from": "wikipedia_fathead",
            "src_domain": "en.wikipedia.org",
            "src_id": 1,
            "src_name": "Wikipedia",
            "src_options": {
                "directory": "",
                "is_fanon": 0,
                "is_mediawiki": 1,
                "is_wikipedia": 1,
                "language": "en",
                "min_abstract_length": "20",
                "skip_abstract": 0,
                "skip_abstract_paren": 0,
                "skip_end": "0",
                "skip_icon": 0,
                "skip_image_name": 0,
                "skip_qr": "",
                "source_skip": "",
                "src_info": ""
            },
            "src_url": null,
            "status": "live",
            "tab": "About",
            "topic": ["productivity"],
            "unsafe": 0
        }
    });

    let response = deserialize_response(json);
    assert_eq!(response.heading.as_deref(), Some("Rust"));
    assert_eq!(response.abstract_source.as_deref(), Some("Wikipedia"));

    let meta = response.meta.expect("Expected meta field");
    assert_eq!(meta.description.as_deref(), Some("Wikipedia"));
    assert_eq!(meta.src_id, Some(1));

    let developers = meta.developer.expect("Expected developers");
    assert_eq!(developers.len(), 1);
    assert_eq!(developers[0].name.as_deref(), Some("DDG Team"));

    let maintainer = meta.maintainer.expect("Expected maintainer");
    assert_eq!(maintainer.github.as_deref(), Some("duckduckgo"));

    let src_opts = meta.src_options.expect("Expected src_options");
    assert_eq!(src_opts.language.as_deref(), Some("en"));
}
