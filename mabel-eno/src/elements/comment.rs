use std::ops::Range;
use std::rc::Rc;

use crate::elements::DocumentInternals;
use crate::Printer;

#[derive(Debug)]
pub struct Comment {
    document_internals: Rc<DocumentInternals>,
    line_begin_index: usize,
    pub line_number: u32,
    operator_index: usize,
    value_range: Option<Range<usize>>,
}

#[allow(dead_code)]
pub trait CommentImpl {
    fn get_operator_index(&self) -> usize;
    fn get_value_range(&self) -> &Option<Range<usize>>;
    #[allow(clippy::new_ret_no_self)]
    fn new(
        document_internals: Rc<DocumentInternals>,
        line_begin_index: usize,
        line_number: u32,
        operator_index: usize,
        value_range: Option<Range<usize>>,
    ) -> Comment;
    fn snippet_with_options(&self, printer: &dyn Printer, gutter: bool) -> String;
}

impl CommentImpl for Comment {
    fn get_operator_index(&self) -> usize {
        self.operator_index
    }

    fn get_value_range(&self) -> &Option<Range<usize>> {
        &self.value_range
    }

    fn new(
        document_internals: Rc<DocumentInternals>,
        line_begin_index: usize,
        line_number: u32,
        operator_index: usize,
        value_range: Option<Range<usize>>,
    ) -> Comment {
        Comment {
            document_internals,
            line_begin_index,
            line_number,
            operator_index,
            value_range,
        }
    }

    fn snippet_with_options(&self, printer: &dyn Printer, gutter: bool) -> String {
        let mut out = String::new();

        if gutter {
            out.push_str(&printer.gutter(self.line_number));
        }

        if let Some(value_range) = &self.value_range {
            out.push_str(
                &printer.comment(
                    &self.document_internals.content[self.line_begin_index..value_range.end],
                ),
            );
        } else {
            out.push_str(&printer.comment(
                &self.document_internals.content[self.line_begin_index..(self.operator_index + 1)],
            ));
        }

        out
    }
}
