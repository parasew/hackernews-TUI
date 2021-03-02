use super::comment_view::CommentView;
use crate::prelude::*;

/// Construct a new Event view from a LinearLayout by adding
/// event handlers for a key pressed
pub fn construct_event_view<T: ListEventView>(view: T) -> OnEventView<T> {
    // add "j" and "k" for moving down and up the story list
    OnEventView::new(view)
        .on_pre_event_inner('k', |s, _| s.focus_up())
        .on_pre_event_inner('j', |s, _| s.focus_down())
}

/// ListEventView is a trait that implements method interfaces
/// to interact with a List View
pub trait ListEventView {
    fn focus_up(&mut self) -> Option<EventResult> {
        None
    }
    fn focus_down(&mut self) -> Option<EventResult> {
        None
    }
}

impl ListEventView for LinearLayout {
    fn focus_up(&mut self) -> Option<EventResult> {
        let id = self.get_focus_index();
        if id > 0 {
            match self.set_focus_index(id - 1) {
                Ok(_) => Some(EventResult::Consumed(None)),
                Err(_) => Some(EventResult::Ignored),
            }
        } else {
            Some(EventResult::Ignored)
        }
    }

    fn focus_down(&mut self) -> Option<EventResult> {
        let id = self.get_focus_index();
        if id + 1 < self.len() {
            match self.set_focus_index(id + 1) {
                Ok(_) => Some(EventResult::Consumed(None)),
                Err(_) => Some(EventResult::Ignored),
            }
        } else {
            Some(EventResult::Ignored)
        }
    }
}

impl ListEventView for CommentView {
    fn focus_up(&mut self) -> Option<EventResult> {
        self.get_inner_mut().focus_up()
    }
    fn focus_down(&mut self) -> Option<EventResult> {
        self.get_inner_mut().focus_down()
    }
}
