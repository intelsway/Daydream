use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{window, Notification, NotificationOptions, NotificationPermission, Window};

#[derive(Clone)]
pub(crate) struct Notifications {
    window: Option<Window>,
    avatar: Option<String>,
    displayname: String,
    content: String,
}

impl Notifications {
    pub fn new(avatar: Option<String>, displayname: String, content: String) -> Self {
        Notifications {
            window: None,
            avatar,
            displayname,
            content,
        }
    }
    fn browser_support(&mut self) -> bool {
        return match &self.window {
            None => match window() {
                Some(v) => {
                    self.window = window();
                    match v.get("Notification") {
                        Some(_) => true,
                        _ => false,
                    }
                }

                _ => false,
            },
            Some(v) => match v.get("Notification") {
                Some(_) => true,
                _ => false,
            },
        };
    }

    fn notifications_allowed(&self) -> bool {
        match Notification::permission() {
            NotificationPermission::Granted => true,
            _ => false,
        }
    }

    pub fn show(&self) {
        if !self.notifications_allowed() {
            let self_clone = self.clone();
            let cb = Closure::wrap(Box::new(move || {
                if self_clone.notifications_allowed() {
                    self_clone.clone().show_actual();
                }
            }) as Box<dyn FnMut()>);

            Notification::request_permission_with_permission_callback(cb.as_ref().unchecked_ref());
            cb.forget();
        } else {
            self.show_actual();
        }
    }

    fn show_actual(&self) {
        let mut options = NotificationOptions::new() as NotificationOptions;
        let mut options = options.body(&self.content).tag("daydream") as &mut NotificationOptions;
        let options = if self.avatar.is_some() {
            options.icon(self.avatar.as_ref().unwrap())
        } else {
            options
        };
        Notification::new_with_options(&self.displayname, &options);
    }
}
