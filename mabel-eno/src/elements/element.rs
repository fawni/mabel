use std::ops::RangeInclusive;

use crate::{Attribute, Embed, Field, Flag, Item, Printer, Section};

pub trait Element: std::fmt::Debug {
    fn as_attribute(&self) -> Option<&Attribute> {
        None
    }
    fn as_embed(&self) -> Option<&Embed> {
        None
    }
    fn as_field(&self) -> Option<&Field> {
        None
    }
    fn as_flag(&self) -> Option<&Flag> {
        None
    }
    fn as_item(&self) -> Option<&Item> {
        None
    }
    fn as_section(&self) -> Option<&Section> {
        None
    }
    fn is_attribute(&self) -> bool {
        false
    }
    fn is_embed(&self) -> bool {
        false
    }
    fn is_field(&self) -> bool {
        false
    }
    fn is_flag(&self) -> bool {
        false
    }
    fn is_item(&self) -> bool {
        false
    }
    fn is_section(&self) -> bool {
        false
    }
    fn line_number(&self) -> u32;
    fn snippet(&self) -> String;
    fn snippet_with_options(&self, printer: &dyn Printer, gutter: bool) -> String;
    fn touch(&self);
}

pub trait ElementImpl {
    fn line_range(&self) -> RangeInclusive<u32>;
    fn touched(&self) -> bool;
}
