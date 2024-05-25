use crate::elements::{DocumentImpl, FieldContent, FieldImpl, SectionImpl};
use crate::{Document, Element, SectionElement};

/// `line` parameter is 1-indexed
pub fn lookup_line(document: &Document, line: u32) -> Result<Option<&dyn Element>, String> {
    if line > document.get_number_of_lines() {
        return Err(format!(
            "Line {} is outside the line range of the document ({} lines)",
            line,
            document.get_number_of_lines()
        ));
    }

    Ok(lookup_in_section_elements(document.get_elements(), line))
}

pub fn lookup_in_section_elements(
    elements: &[Box<dyn SectionElement>],
    line: u32,
) -> Option<&dyn Element> {
    for element in elements {
        if element.line_number() == line {
            return Some(element.as_element());
        }

        if let Some(field) = element.as_field() {
            match &field.get_content() {
                FieldContent::Attributes(attributes) => {
                    for attribute in attributes {
                        if attribute.line_number == line {
                            return Some(attribute);
                        }
                    }
                }
                FieldContent::Items(items) => {
                    for item in items {
                        if item.line_number == line {
                            return Some(item);
                        }
                    }
                }
                _ => (),
            }
        } else if let Some(section) = element.as_section() {
            if let Some(element) = lookup_in_section_elements(section.get_elements(), line) {
                return Some(element);
            }
        }
    }

    None
}
