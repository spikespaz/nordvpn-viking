use build_helpers::gresources::*;
use once_cell::sync::Lazy;
use std::fmt::Debug;
use strong_xml::{XmlRead, XmlWrite};
use test_case::test_case;

static EXAMPLE_FILES: Lazy<[(&'static str, File); 6]> = Lazy::new(|| {
    [
        (
            r#"<file>foo/bar/baz_1.png</file>"#,
            File::new("foo/bar/baz_1.png".to_owned(), None, None, None),
        ),
        (
            r#"<file alias="image_2.png">foo/bar/baz_2.png</file>"#,
            File::new(
                "foo/bar/baz_2.png".to_owned(),
                Some("image_2.png".to_owned()),
                None,
                None,
            ),
        ),
        (
            r#"<file compressed="true">foo/bar/baz_3.png</file>"#,
            File::new("foo/bar/baz_3.png".to_owned(), None, Some(true), None),
        ),
        (
            r#"<file preprocess="to-pixdata">foo/bar/baz_4.png</file>"#,
            File::new(
                "foo/bar/baz_4.png".to_owned(),
                None,
                None,
                Some(Preprocess::ToPixData),
            ),
        ),
        (
            r#"<file alias="image_5.png" compressed="true" preprocess="to-pixdata">foo/bar/baz_5.png</file>"#,
            File::new(
                "foo/bar/baz_5.png".to_owned(),
                Some("image_5.png".to_owned()),
                Some(true),
                Some(Preprocess::ToPixData),
            ),
        ),
        (
            r#"<file alias="icon.svg" compressed="true" preprocess="xml-stripblanks">foo/bar/baz_6.svg</file>"#,
            File::new(
                "foo/bar/baz_6.svg".to_owned(),
                Some("icon.svg".to_owned()),
                Some(true),
                Some(Preprocess::XmlStripBlanks),
            ),
        ),
    ]
});

static EXAMPLE_GRESOURCE: Lazy<(&'static str, GResource)> = Lazy::new(|| {
    (
        Box::leak(
            format!(
                r#"<gresource prefix="/com/example/project/res">{}</gresource>"#,
                EXAMPLE_FILES
                    .iter()
                    .fold(String::new(), |xml_a, (xml_b, _)| xml_a + xml_b)
            )
            .into_boxed_str(),
        ),
        GResource::from_iter(
            "/com/example/project/res".to_owned(),
            EXAMPLE_FILES.iter().map(|(_, file)| file.clone()),
        ),
    )
});

static EXAMPLE_GRESOURCES: Lazy<(&'static str, GResources)> = Lazy::new(|| {
    (
        Box::leak(format!("<gresources>{}</gresources>", EXAMPLE_GRESOURCE.0).into_boxed_str()),
        GResources::from_iter([EXAMPLE_GRESOURCE.1.clone()]),
    )
});

#[test_case(EXAMPLE_FILES[0].0, &EXAMPLE_FILES[0].1 ; "test deserialize file 1")]
#[test_case(EXAMPLE_FILES[1].0, &EXAMPLE_FILES[1].1 ; "test deserialize file 2")]
#[test_case(EXAMPLE_FILES[2].0, &EXAMPLE_FILES[2].1 ; "test deserialize file 3")]
#[test_case(EXAMPLE_FILES[3].0, &EXAMPLE_FILES[3].1 ; "test deserialize file 4")]
#[test_case(EXAMPLE_FILES[4].0, &EXAMPLE_FILES[4].1 ; "test deserialize file 5")]
#[test_case(EXAMPLE_FILES[5].0, &EXAMPLE_FILES[5].1 ; "test deserialize file 6")]
#[test_case(EXAMPLE_GRESOURCE.0, &EXAMPLE_GRESOURCE.1 ; "test deserialze gresource")]
#[test_case(EXAMPLE_GRESOURCES.0, &EXAMPLE_GRESOURCES.1 ; "test deserialze gresources")]
fn test_deserialize<'a, T>(xml: &'a str, expected: &T)
where
    T: XmlRead<'a> + Debug + PartialEq,
{
    let result = T::from_str(xml).unwrap();
    assert_eq!(expected, &result);
}

#[test_case(&EXAMPLE_FILES[0].1, EXAMPLE_FILES[0].0 ; "test serialize file 1")]
#[test_case(&EXAMPLE_FILES[1].1, EXAMPLE_FILES[1].0 ; "test serialize file 2")]
#[test_case(&EXAMPLE_FILES[2].1, EXAMPLE_FILES[2].0 ; "test serialize file 3")]
#[test_case(&EXAMPLE_FILES[3].1, EXAMPLE_FILES[3].0 ; "test serialize file 4")]
#[test_case(&EXAMPLE_FILES[4].1, EXAMPLE_FILES[4].0 ; "test serialize file 5")]
#[test_case(&EXAMPLE_FILES[5].1, EXAMPLE_FILES[5].0 ; "test serialize file 6")]
#[test_case(&EXAMPLE_GRESOURCE.1, EXAMPLE_GRESOURCE.0 ; "test serialze gresource")]
#[test_case(&EXAMPLE_GRESOURCES.1, EXAMPLE_GRESOURCES.0 ; "test serialze gresources")]
fn test_serialize<T>(data: &T, expected: &str)
where
    T: XmlWrite + PartialEq,
{
    let xml = data.to_string().unwrap();
    assert_eq!(expected, xml);
}
