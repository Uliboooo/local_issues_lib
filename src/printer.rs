use std::fmt::{self, Display};

use crate::StatusT;

pub trait AltDisplay {
    fn fmt_only_open(&self) -> String;
    fn fmt_only_prop(&self) -> String;
}

pub struct IssuePrinter<T: Display> {
    content: T,
    only_opened: bool,
    // only_property: bool,
    contain_message: bool,
}

impl<T: Display> IssuePrinter<T> {
    fn new(content: T) -> Self {
        Self {
            content,
            only_opened: false,
            contain_message: false,
        }
    }
    fn opened(&mut self, opened: bool) {
        self.only_opened = opened
    }
    // fn property(&mut self, prop: bool) {
    //     self.only_property = prop
    // }
    fn contain_messages(&mut self, con: bool) {
        self.contain_message = con
    }
}

impl<T: Display + AltDisplay + StatusT + Iterator> Display for IssuePrinter<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let b = self.content;
        // let a = match (self.only_opened, self.contain_message) {
        //     // opened & contain messages
        //     (true, true) => {
        //         if self.content.is_opened() {
        //             format!("{}", self.content)
        //         } else {
        //             String::new()
        //         }
        //     }
        //     // opened & don't contain messages
        //     (true, false) => todo!(),
        //     // contain closed & messages
        //     (false, true) => todo!(),
        //     // contain closed & don't contain messages
        //     (false, false) => todo!(),
        // };
        write!(f, "{}", a)
    }
}
