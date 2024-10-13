use anpa::core::{ParserExt, ParserInto, StrParser};
use anpa::parsers::{item_if, item_while, seq, until_seq};
use anpa::combinators::{get_parsed, left, many, many_to_vec, middle, no_separator, right, separator};
use anpa::{create_parser, item, right, tuplify, variadic};
use crossterm::event::KeyCode;
use crate::parser::jetbrains::low_level_parser::{attribute, attribute_value, eat};

#[derive(Debug, PartialEq)]
struct Attribute<'a> {
    key: &'a str,
    value: &'a str,
}

impl<'a> Attribute<'a> {
    fn new(key: &'a str, value: &'a str) -> Self {
        Self { key, value }
    }
}

#[derive(Debug, PartialEq)]
struct XmlTag<'a> {
    name: &'a str,
    attributes: Vec<Attribute<'a>>,
    children: Vec<Xml<'a>>,
}

impl XmlTag<'_> {
    fn new<'a>(name: &'a str, attributes: Vec<Attribute<'a>>, children: Vec<Xml<'a>>) -> XmlTag<'a> {
        XmlTag { name, attributes, children }
    }
}

#[derive(Debug, PartialEq)]
enum Xml<'a> {
    Element(XmlTag<'a>),
    Attribute(Attribute<'a>),
    Comment(&'a str),
    OpenXmlTag(&'a str, Vec<Attribute<'a>>),
    CloseXmlTag(&'a str),
    SelfContainedXmlTag(&'a str, Vec<Attribute<'a>>),
}

// example:
// <keymap version="1" name="Eclipse copy" parent="Eclipse">
//     <action id="$Copy">
//         <keyboard-shortcut first-keystroke="ctrl c" />
//     </action>
//     <action id="$Redo">
//         <keyboard-shortcut first-keystroke="shift ctrl z" />
//     </action>
//     <action id=":cursive.repl.actions/jump-to-repl">
//         <keyboard-shortcut first-keystroke="ctrl 2" />
//     </action>
//     <action id=":cursive.repl.actions/run-last-sexp">
//         <keyboard-shortcut first-keystroke="ctrl 3" />
//     </action>
//     <action id=":cursive.repl.actions/sync-files">
//         <keyboard-shortcut first-keystroke="shift ctrl r" />
//     </action>
//     <action id="ActivateMavenProjectsToolWindow">
//         <keyboard-shortcut first-keystroke="f2" />
//     </action>
//     <action id="Build">
//         <keyboard-shortcut first-keystroke="ctrl f9" />
//     </action>
//     <action id="BuildProject">
//         <keyboard-shortcut first-keystroke="ctrl b" />
//     </action>
//     <action id="ChooseDebugConfiguration">
//         <keyboard-shortcut first-keystroke="alt d" />
//     </action>
//     <action id="ChooseRunConfiguration">
//         <keyboard-shortcut first-keystroke="alt r" />
//     </action>
//     <action id="CloseActiveTab" />
//     <action id="CloseContent">
//         <keyboard-shortcut first-keystroke="ctrl w" />
//     </action>
//     <action id="CollapseAll">
//         <keyboard-shortcut first-keystroke="ctrl subtract" />
//     </action>
//     <action id="CollapseAllRegions">
//         <!-- this is a comment -->
//         <keyboard-shortcut first-keystroke="shift ctrl divide" />
//         <keyboard-shortcut first-keystroke="<![CDATA[ctrl minus]]>" />
//     </action>
// </keymap>

struct KeyMap {
    version: String,
    name: String,
    parent: String,
    actions: Vec<Action>,
}

struct Action {
    id: String,
    shortcuts: Vec<Shortcut>,
}

struct Shortcut {
    first_keystroke: String,
}

struct Element<'a> {
    name: String,
    attributes: Vec<(String, String)>,
    children: Vec<Xml<'a>>,
}


mod low_level_parser {
    use anpa::combinators::{many, middle, no_separator, right, succeed};
    use anpa::parsers::{item_if, item_while};
    use anpa::core::ParserInto;
    use anpa::item;
    use super::*;

    fn whitespace<'a>() -> impl StrParser<'a, ()> {
        item_while(|c: char| c.is_whitespace()).map(|_| ())
    }

    pub(super) fn attribute_value<'a>() -> impl StrParser<'a, &'a str> {
        let valid_char = item_if(|c: char| c != '"');
        middle(item!('"'), many(valid_char, true, no_separator()), item!('"')).into_type()
    }

    pub(super) fn cdata<'a>() -> impl StrParser<'a, &'a str> {
        let valid_char = item_if(|c: char| c != ']');
        middle(seq("<![CDATA["), many(valid_char, true, no_separator()), seq("]]>"))
            .into_type()
            .map(|s: &str| s.trim())
    }

    pub(super) fn eat<'a, O>(p: impl StrParser<'a, O>) -> impl StrParser<'a, O> {
        right(succeed(item_while(|c: char| c.is_whitespace())), p)
    }

    pub(super) fn attribute<'a>() -> impl StrParser<'a, Attribute<'a>> {
        let name = item_while(|c: char| c.is_alphabetic() || c.is_numeric() || c == '_');

        tuplify!(
            left(eat(name), eat(item!('='))),
            eat(attribute_value()),
        ).map(|(key, value)| Attribute::new(key, value))
    }
}

fn comment<'a>() -> impl StrParser<'a, Xml<'a>> {
    right!(seq("<!--"), until_seq("-->"))
        .map(|s: &str| Xml::Comment(s.trim()))
}

fn self_contained_tag<'a>() -> impl StrParser<'a, Xml<'a>> {
    let name = item_while(|c: char| c != ' ' && c != '/');
    let attributes = many_to_vec(attribute(), true, no_separator());

    tuplify!(
    right(eat(item!('<')), name),
    left(attributes, eat(seq("/>")))
).map(|(name, attributes)| Xml::SelfContainedXmlTag(name, attributes))
}

fn tag_open<'a>() -> impl StrParser<'a, Xml<'a>> {
    let name = item_while(|c: char| c != ' ' && c != '/');
    let attributes = many_to_vec(attribute(), true, no_separator());

    tuplify!(
        right(eat(item!('<')), name),
        left(attributes, eat(item!('>')))
    ).map(|(name, attributes)| Xml::OpenXmlTag(name, attributes))
}

fn tag_close<'a>() -> impl StrParser<'a, Xml<'a>> {
    right!(seq("</"), item_while(|c: char| c != '>'))
        .map(Xml::CloseXmlTag)
}

mod tests {
    use anpa::core::parse;
    use crate::parser::jetbrains::low_level_parser::cdata;
    use crate::parser::jetbrains::{comment, self_contained_tag, tag_open};
    use super::*;

    #[test]
    fn test_comment() {
        [
            "<!-- This is a comment -->",
            "<!--This is a comment-->",
            r#"<!--

                This is a comment

            -->"#,
        ].iter().for_each(|input| {
            let p = comment();
            let result = parse(p, input);

            assert_eq!(result.state, "");
            assert_eq!(result.result, Some(Xml::Comment("This is a comment")));
        });
    }

    #[test]
    fn test_cdata() {
        [
            "<![CDATA[This is a CDATA]]>",
            r#"<![CDATA[

                This is a CDATA

            ]]>"#,
        ].iter().for_each(|input| {
            let p = cdata();
            let result = parse(p, input);

            assert_eq!(result.state, "");
            assert_eq!(result.result, Some("This is a CDATA"));
        });
    }

    #[test]
    fn parse_attribute_value() {
        let p = attribute_value();
        let result = parse(p, r#""This is a value" "#);

        assert_eq!(result.state, " ");
        assert_eq!(result.result, Some("This is a value"));
    }

    #[test]
    fn parse_attribute() {
        [
            r#"name="value" "#,
            r#"name =  "value" "#,
        ].iter().for_each(|s|{
            let p = attribute();
            let result = parse(p, s);
            assert_eq!(result.state, " ");
            assert_eq!(result.result, Some(Attribute::new("name", "value")));
        });
    }

    #[test]
    fn parse_open_tag() {
        let p = tag_open();
        let result = parse(p, "<tag key=\"value\">");

        assert_eq!(result.state, "");
        assert_eq!(result.result, Some(Xml::OpenXmlTag("tag", vec![Attribute::new("key", "value")])));

        let result = parse(p, "<tag key=\"value\" key2=\"value2\">");

        assert_eq!(result.state, "");
        assert_eq!(result.result, Some(Xml::OpenXmlTag("tag", vec![Attribute::new("key", "value"), Attribute::new("key2", "value2")])));

        let result = parse(p, "<tag key=\"value\" key2=\"value2\" key3=\"value3\">");

        assert_eq!(result.state, "");
        assert_eq!(result.result, Some(Xml::OpenXmlTag("tag", vec![Attribute::new("key", "value"), Attribute::new("key2", "value2"), Attribute::new("key3", "value3")])));
    }

    #[test]
    fn parse_self_contained_tag() {
        let p = self_contained_tag();
        let result = parse(p, "<tag key=\"value\"/>");

        assert_eq!(result.state, "");
        assert_eq!(result.result, Some(Xml::SelfContainedXmlTag("tag", vec![Attribute::new("key", "value")])));
    }
}