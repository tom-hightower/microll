
pub struct Ids {
    pub master: conrod::widget::Id,
    pub menu_bar: MenuBarIds,
    pub url_bar: UrlBarIds,
    pub body: conrod::widget::Id,
    pub file_load_btn: conrod::widget::Id,
    pub page_load_btn: conrod::widget::Id,
    pub line_breaks: conrod::widget::id::List,
    pub code_elements: conrod::widget::id::List,
    pub text_elements: conrod::widget::id::List,
    pub link_elements: conrod::widget::id::List,
}

impl Ids {
    pub fn new(mut generator: conrod::widget::id::Generator) -> Self {
        Ids {
            master: generator.next(),
            menu_bar: MenuBarIds::new(&mut generator),
            url_bar: UrlBarIds::new(&mut generator),
            body: generator.next(),
            file_load_btn: generator.next(),
            page_load_btn: generator.next(),
            line_breaks: conrod::widget::id::List::new(),
            code_elements: conrod::widget::id::List::new(),
            text_elements: conrod::widget::id::List::new(),
            link_elements: conrod::widget::id::List::new(),
        }
    }
}

pub struct MenuBarIds {
    pub canvas: conrod::widget::Id,
    pub tabs: conrod::widget::Id,
    pub file_menu: ListButton,
    pub view_menu: ListButton,
    pub link_menu: ListButton,
    pub help_menu: ListButton,
}

impl MenuBarIds {
    fn new(mut generator: &mut conrod::widget::id::Generator) -> Self {
        MenuBarIds {
            canvas: generator.next(),
            tabs: generator.next(),
            file_menu: ListButton::new(&mut generator),
            view_menu: ListButton::new(&mut generator),
            link_menu: ListButton::new(&mut generator),
            help_menu: ListButton::new(&mut generator),
        }
    }
}

pub struct ListButton {
    pub button: conrod::widget::Id,
    pub list: conrod::widget::id::List,
}

impl ListButton {
    pub fn new(generator: &mut conrod::widget::id::Generator) -> Self {
        ListButton {
            button: generator.next(),
            list: conrod::widget::id::List::new(),
        }
    }
}

pub struct UrlBarIds {
    pub canvas: conrod::widget::Id,
    pub title_text: conrod::widget::Id,
    pub input_box: conrod::widget::Id,
    pub go_button: conrod::widget::Id,
}

impl UrlBarIds {
    fn new(generator: &mut conrod::widget::id::Generator) -> Self {
        UrlBarIds {
            canvas: generator.next(),
            title_text: generator.next(),
            input_box: generator.next(),
            go_button: generator.next(),
        }
    }
}