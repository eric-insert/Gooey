pub struct WindowConfig<'a> {
    pub title: &'a str,
    pub width: Option<i32>,
    pub height: Option<i32>,
    pub resizable: Option<bool>,
    pub visible: bool,
    pub decorations: Option<bool>,
    pub fullscreen: Option<bool>,
    pub min_size: Option<(u32, u32)>,
    pub max_size: Option<(u32, u32)>,
    pub screen_position: Option<(i32, i32)>,
    pub gpu_access: Option<bool>,
    pub uncloseable: Option<bool>,
    pub dbl_click_detect: Option<bool>,
}

impl<'a> Default for WindowConfig<'a> {
    fn default() -> Self {
        Self {
            title: "Gooey Window",
            width: Some(800),
            height: Some(600),
            resizable: Some(true),
            visible: true,
            decorations: Some(true),
            fullscreen: Some(false),
            min_size: None,
            max_size: None,
            screen_position: None,
            gpu_access: Some(false),
            uncloseable: Some(false),
            dbl_click_detect: Some(true),
        }
    }
}