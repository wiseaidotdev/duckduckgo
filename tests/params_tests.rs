use duckduckgo::params::{
    AddressBar, Favicons, Font, HeaderBehavior, PageNumbers, Placement, Region, SafeSearch,
    SearchParams, Size, Theme, Toggle, UnitsMeasure, VideoPlayback, Width,
};

#[test]
fn test_default_params_produces_no_pairs() {
    let params = SearchParams::default();
    assert!(
        params.to_query_pairs().is_empty(),
        "Default SearchParams should produce no query pairs"
    );
}

#[test]
fn test_new_params_produces_no_pairs() {
    let params = SearchParams::new();
    assert!(
        params.to_query_pairs().is_empty(),
        "SearchParams::new() should produce no query pairs"
    );
}

#[test]
fn test_region_us_en() {
    let pairs = SearchParams::new().region(Region::UsEn).to_query_pairs();
    assert!(pairs.iter().any(|(k, v)| *k == "kl" && v == "us-en"));
}

#[test]
fn test_region_wt_wt() {
    let pairs = SearchParams::new().region(Region::WtWt).to_query_pairs();
    assert!(pairs.iter().any(|(k, v)| *k == "kl" && v == "wt-wt"));
}

#[test]
fn test_region_fr_fr() {
    let pairs = SearchParams::new().region(Region::FrFr).to_query_pairs();
    assert!(pairs.iter().any(|(k, v)| *k == "kl" && v == "fr-fr"));
}

#[test]
fn test_region_de_de() {
    let pairs = SearchParams::new().region(Region::DeDe).to_query_pairs();
    assert!(pairs.iter().any(|(k, v)| *k == "kl" && v == "de-de"));
}

#[test]
fn test_region_jp_jp() {
    let pairs = SearchParams::new().region(Region::JpJp).to_query_pairs();
    assert!(pairs.iter().any(|(k, v)| *k == "kl" && v == "jp-jp"));
}

#[test]
fn test_safe_search_on() {
    let pairs = SearchParams::new()
        .safe_search(SafeSearch::On)
        .to_query_pairs();
    assert!(pairs.iter().any(|(k, v)| *k == "kp" && v == "1"));
}

#[test]
fn test_safe_search_moderate() {
    let pairs = SearchParams::new()
        .safe_search(SafeSearch::Moderate)
        .to_query_pairs();
    assert!(pairs.iter().any(|(k, v)| *k == "kp" && v == "-1"));
}

#[test]
fn test_safe_search_off() {
    let pairs = SearchParams::new()
        .safe_search(SafeSearch::Off)
        .to_query_pairs();
    assert!(pairs.iter().any(|(k, v)| *k == "kp" && v == "-2"));
}

#[test]
fn test_open_instant_answers_on() {
    let pairs = SearchParams::new()
        .open_instant_answers(Toggle::On)
        .to_query_pairs();
    assert!(pairs.iter().any(|(k, v)| *k == "kz" && v == "1"));
}

#[test]
fn test_open_instant_answers_off() {
    let pairs = SearchParams::new()
        .open_instant_answers(Toggle::Off)
        .to_query_pairs();
    assert!(pairs.iter().any(|(k, v)| *k == "kz" && v == "-1"));
}

#[test]
fn test_auto_load_images() {
    let pairs = SearchParams::new()
        .auto_load_images(Toggle::On)
        .to_query_pairs();
    assert!(pairs.iter().any(|(k, v)| *k == "kc" && v == "1"));
}

#[test]
fn test_auto_load_results() {
    let pairs = SearchParams::new()
        .auto_load_results(Toggle::Off)
        .to_query_pairs();
    assert!(pairs.iter().any(|(k, v)| *k == "kav" && v == "-1"));
}

#[test]
fn test_new_window() {
    let pairs = SearchParams::new().new_window(Toggle::On).to_query_pairs();
    assert!(pairs.iter().any(|(k, v)| *k == "kn" && v == "1"));
}

#[test]
fn test_full_urls() {
    let pairs = SearchParams::new().full_urls(Toggle::On).to_query_pairs();
    assert!(pairs.iter().any(|(k, v)| *k == "kaf" && v == "1"));
}

#[test]
fn test_auto_suggest() {
    let pairs = SearchParams::new()
        .auto_suggest(Toggle::Off)
        .to_query_pairs();
    assert!(pairs.iter().any(|(k, v)| *k == "kac" && v == "-1"));
}

#[test]
fn test_redirect_on() {
    let pairs = SearchParams::new().redirect(Toggle::On).to_query_pairs();
    assert!(pairs.iter().any(|(k, v)| *k == "kd" && v == "1"));
}

#[test]
fn test_https_on() {
    let pairs = SearchParams::new().https(Toggle::On).to_query_pairs();
    assert!(pairs.iter().any(|(k, v)| *k == "kh" && v == "1"));
}

#[test]
fn test_address_bar_get() {
    let pairs = SearchParams::new()
        .address_bar(AddressBar::Get)
        .to_query_pairs();
    assert!(pairs.iter().any(|(k, v)| *k == "kg" && v == "g"));
}

#[test]
fn test_address_bar_post() {
    let pairs = SearchParams::new()
        .address_bar(AddressBar::Post)
        .to_query_pairs();
    assert!(pairs.iter().any(|(k, v)| *k == "kg" && v == "p"));
}

#[test]
fn test_video_playback_always_on_ddg() {
    let pairs = SearchParams::new()
        .video_playback(VideoPlayback::AlwaysOnDdg)
        .to_query_pairs();
    assert!(pairs.iter().any(|(k, v)| *k == "k5" && v == "1"));
}

#[test]
fn test_video_playback_third_party() {
    let pairs = SearchParams::new()
        .video_playback(VideoPlayback::ThirdPartySite)
        .to_query_pairs();
    assert!(pairs.iter().any(|(k, v)| *k == "k5" && v == "2"));
}

#[test]
fn test_video_playback_prompt() {
    let pairs = SearchParams::new()
        .video_playback(VideoPlayback::Prompt)
        .to_query_pairs();
    assert!(pairs.iter().any(|(k, v)| *k == "k5" && v == "-1"));
}

#[test]
fn test_header_color_preset() {
    let pairs = SearchParams::new().header_color("r3").to_query_pairs();
    assert!(pairs.iter().any(|(k, v)| *k == "kj" && v == "r3"));
}

#[test]
fn test_background_color_hex() {
    let pairs = SearchParams::new()
        .background_color("395323")
        .to_query_pairs();
    assert!(pairs.iter().any(|(k, v)| *k == "k7" && v == "395323"));
}

#[test]
fn test_url_color() {
    let pairs = SearchParams::new().url_color("g").to_query_pairs();
    assert!(pairs.iter().any(|(k, v)| *k == "kx" && v == "g"));
}

#[test]
fn test_text_color() {
    let pairs = SearchParams::new().text_color("g").to_query_pairs();
    assert!(pairs.iter().any(|(k, v)| *k == "k8" && v == "g"));
}

#[test]
fn test_link_color() {
    let pairs = SearchParams::new().link_color("b").to_query_pairs();
    assert!(pairs.iter().any(|(k, v)| *k == "k9" && v == "b"));
}

#[test]
fn test_visited_link_color() {
    let pairs = SearchParams::new().visited_link_color("p").to_query_pairs();
    assert!(pairs.iter().any(|(k, v)| *k == "kaa" && v == "p"));
}

#[test]
fn test_theme_dark() {
    let pairs = SearchParams::new().theme(Theme::Dark).to_query_pairs();
    assert!(pairs.iter().any(|(k, v)| *k == "kae" && v == "d"));
}

#[test]
fn test_theme_terminal() {
    let pairs = SearchParams::new().theme(Theme::Terminal).to_query_pairs();
    assert!(pairs.iter().any(|(k, v)| *k == "kae" && v == "t"));
}

#[test]
fn test_theme_custom() {
    let pairs = SearchParams::new()
        .theme(Theme::Custom("1a2b3c".to_string()))
        .to_query_pairs();
    assert!(pairs.iter().any(|(k, v)| *k == "kae" && v == "1a2b3c"));
}

#[test]
fn test_size_small() {
    let pairs = SearchParams::new().size(Size::Small).to_query_pairs();
    assert!(pairs.iter().any(|(k, v)| *k == "ks" && v == "s"));
}

#[test]
fn test_width_wide() {
    let pairs = SearchParams::new().width(Width::Wide).to_query_pairs();
    assert!(pairs.iter().any(|(k, v)| *k == "kw" && v == "w"));
}

#[test]
fn test_placement_left() {
    let pairs = SearchParams::new()
        .placement(Placement::Left)
        .to_query_pairs();
    assert!(pairs.iter().any(|(k, v)| *k == "km" && v == "l"));
}

#[test]
fn test_link_font_verdana() {
    let pairs = SearchParams::new()
        .link_font(Font::Verdana)
        .to_query_pairs();
    assert!(pairs.iter().any(|(k, v)| *k == "ka" && v == "v"));
}

#[test]
fn test_text_font_custom() {
    let pairs = SearchParams::new()
        .text_font(Font::Custom("Roboto".to_string()))
        .to_query_pairs();
    assert!(pairs.iter().any(|(k, v)| *k == "kt" && v == "Roboto"));
}

#[test]
fn test_underline_on() {
    let pairs = SearchParams::new().underline(Toggle::On).to_query_pairs();
    assert!(pairs.iter().any(|(k, v)| *k == "ku" && v == "1"));
}

#[test]
fn test_header_behavior_off() {
    let pairs = SearchParams::new()
        .header_behavior(HeaderBehavior::Off)
        .to_query_pairs();
    assert!(pairs.iter().any(|(k, v)| *k == "ko" && v == "-2"));
}

#[test]
fn test_advertisements_off() {
    let pairs = SearchParams::new()
        .advertisements(Toggle::Off)
        .to_query_pairs();
    assert!(pairs.iter().any(|(k, v)| *k == "k1" && v == "-1"));
}

#[test]
fn test_page_numbers_on_no_numbers() {
    let pairs = SearchParams::new()
        .page_numbers(PageNumbers::OnNoNumbers)
        .to_query_pairs();
    assert!(pairs.iter().any(|(k, v)| *k == "kv" && v == "n"));
}

#[test]
fn test_units_measure_off() {
    let pairs = SearchParams::new()
        .units_measure(UnitsMeasure::Off)
        .to_query_pairs();
    assert!(pairs.iter().any(|(k, v)| *k == "kaj" && v == "-1"));
}

#[test]
fn test_source_identifier() {
    let pairs = SearchParams::new().source("my_app").to_query_pairs();
    assert!(pairs.iter().any(|(k, v)| *k == "t" && v == "my_app"));
}

#[test]
fn test_favicons_just_favicons() {
    let pairs = SearchParams::new()
        .favicons(Favicons::JustFavicons)
        .to_query_pairs();
    assert!(pairs.iter().any(|(k, v)| *k == "kf" && v == "1"));
}

#[test]
fn test_favicons_wot_and_favicons() {
    let pairs = SearchParams::new()
        .favicons(Favicons::WotAndFavicons)
        .to_query_pairs();
    assert!(pairs.iter().any(|(k, v)| *k == "kf" && v == "fw"));
}

#[test]
fn test_multiple_params_combined() {
    let params = SearchParams::new()
        .region(Region::UsEn)
        .safe_search(SafeSearch::On)
        .theme(Theme::Dark)
        .source("test_suite");

    let pairs = params.to_query_pairs();
    assert_eq!(pairs.len(), 4);
    assert!(pairs.iter().any(|(k, v)| *k == "kl" && v == "us-en"));
    assert!(pairs.iter().any(|(k, v)| *k == "kp" && v == "1"));
    assert!(pairs.iter().any(|(k, v)| *k == "kae" && v == "d"));
    assert!(pairs.iter().any(|(k, v)| *k == "t" && v == "test_suite"));
}

#[test]
fn test_all_region_as_str_values_are_non_empty() {
    let regions = [
        Region::XaAr,
        Region::XaEn,
        Region::ArEs,
        Region::AuEn,
        Region::AtDe,
        Region::BeFr,
        Region::BeNl,
        Region::BrPt,
        Region::BgBg,
        Region::CaEn,
        Region::CaFr,
        Region::CtCa,
        Region::ClEs,
        Region::CnZh,
        Region::CoEs,
        Region::HrHr,
        Region::CzCs,
        Region::DkDa,
        Region::EeEt,
        Region::FiFi,
        Region::FrFr,
        Region::DeDe,
        Region::GrEl,
        Region::HkTzh,
        Region::HuHu,
        Region::InEn,
        Region::IdId,
        Region::IdEn,
        Region::IeEn,
        Region::IlHe,
        Region::ItIt,
        Region::JpJp,
        Region::KrKr,
        Region::LvLv,
        Region::LtLt,
        Region::XlEs,
        Region::MyMs,
        Region::MyEn,
        Region::MxEs,
        Region::NlNl,
        Region::NzEn,
        Region::NoNo,
        Region::PeEs,
        Region::PhEn,
        Region::PhTl,
        Region::PlPl,
        Region::PtPt,
        Region::RoRo,
        Region::RuRu,
        Region::SgEn,
        Region::SkSk,
        Region::SlSl,
        Region::ZaEn,
        Region::EsEs,
        Region::SeSv,
        Region::ChDe,
        Region::ChFr,
        Region::ChIt,
        Region::TwTzh,
        Region::ThTh,
        Region::TrTr,
        Region::UaUk,
        Region::UkEn,
        Region::UsEn,
        Region::UeEs,
        Region::VeEs,
        Region::VnVi,
        Region::WtWt,
    ];

    for region in &regions {
        assert!(
            !region.as_str().is_empty(),
            "Region {:?} has an empty as_str()",
            region
        );
    }
}
