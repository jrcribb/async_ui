use crate::events::{EmitEvent, EventFutureStream};
use web_sys::{HtmlElement, HtmlInputElement, HtmlSelectElement, HtmlTextAreaElement};

macro_rules! make_event_impl {
    ($ev_name:literal, $func_name:ident, $ty:ty, $link:tt) => {
        #[must_use("the returned object is a Future+Stream that does nothing unless polled")]
        #[doc = "Like [until_event][EmitEvent::until_event] for the `"]
        #[doc = $ev_name]
        #[doc = "` event."]
        #[doc = "See"]
        #[doc = $link]
        #[doc = "."]
        fn $func_name(&self) -> EventFutureStream<$ty> {
            self.as_ref().until_event($ev_name.into())
        }
    };
}

/// Subscribe to common events emitted by HTML elements such as `click` or `scroll`.
#[rustfmt::skip]
pub trait EmitElementEvent: AsRef<HtmlElement> {
    make_event_impl!("cancel", until_cancel, web_sys::Event, "[MDN documentation for this event](https://developer.mozilla.org/en-US/docs/Web/API/Element/mousemove_event)");
    make_event_impl!("error", until_error, web_sys::Event, "[MDN documentation for this event](https://developer.mozilla.org/en-US/docs/Web/API/Element/error_event)");
    make_event_impl!("scroll", until_scroll, web_sys::Event, "[MDN documentation for this event](https://developer.mozilla.org/en-US/docs/Web/API/Element/scroll_event)");
    make_event_impl!("securitypolicyviolation", until_securitypolicyviolation, web_sys::Event, "[MDN documentation for this event](https://developer.mozilla.org/en-US/docs/Web/API/Element/securitypolicyviolation_event)");
    make_event_impl!("select", until_select, web_sys::Event, "[MDN documentation for this event](https://developer.mozilla.org/en-US/docs/Web/API/Element/select_event)");
    make_event_impl!("wheel", until_wheel, web_sys::WheelEvent, "[MDN documentation for this event](https://developer.mozilla.org/en-US/docs/Web/API/Element/wheel_event)");

    make_event_impl!("compositionend", until_compositionend, web_sys::CompositionEvent, "[MDN documentation for this event](https://developer.mozilla.org/en-US/docs/Web/API/Element/compositionend_event)");
    make_event_impl!("compositionstart", until_compositionstart, web_sys::CompositionEvent, "[MDN documentation for this event](https://developer.mozilla.org/en-US/docs/Web/API/Element/compositionstart_event)");
    make_event_impl!("compositionupdate", until_compositionupdate, web_sys::CompositionEvent, "[MDN documentation for this event](https://developer.mozilla.org/en-US/docs/Web/API/Element/compositionupdate_event)");

    make_event_impl!("blur", until_blur, web_sys::UiEvent, "[MDN documentation for this event](https://developer.mozilla.org/en-US/docs/Web/API/Element/blur_event)");
    make_event_impl!("focus", until_focus, web_sys::UiEvent, "[MDN documentation for this event](https://developer.mozilla.org/en-US/docs/Web/API/Element/focus_event)");
    make_event_impl!("focusin", until_focusin, web_sys::UiEvent, "[MDN documentation for this event](https://developer.mozilla.org/en-US/docs/Web/API/Element/focusin_event)");
    make_event_impl!("focusout", until_focusout, web_sys::UiEvent, "[MDN documentation for this event](https://developer.mozilla.org/en-US/docs/Web/API/Element/focusout_event)");

    make_event_impl!("fullscreenchange", until_fullscreenchange, web_sys::Event, "[MDN documentation for this event](https://developer.mozilla.org/en-US/docs/Web/API/Element/fullscreenchange_event)");
    make_event_impl!("fullscreenerror", until_fullscreenerror, web_sys::Event, "[MDN documentation for this event](https://developer.mozilla.org/en-US/docs/Web/API/Element/fullscreenerror_event)");

    make_event_impl!("keydown", until_keydown, web_sys::KeyboardEvent, "[MDN documentation for this event](https://developer.mozilla.org/en-US/docs/Web/API/Element/keydown_event)");
    make_event_impl!("keyup", until_keyup, web_sys::KeyboardEvent, "[MDN documentation for this event](https://developer.mozilla.org/en-US/docs/Web/API/Element/keyup_event)");

    make_event_impl!("auxclick", until_auxclick, web_sys::MouseEvent, "[MDN documentation for this event](https://developer.mozilla.org/en-US/docs/Web/API/Element/auxclick_event)");
    make_event_impl!("click", until_click, web_sys::MouseEvent, "[MDN documentation for this event](https://developer.mozilla.org/en-US/docs/Web/API/Element/click_event)");
    make_event_impl!("contextmenu", until_contextmenu, web_sys::MouseEvent, "[MDN documentation for this event](https://developer.mozilla.org/en-US/docs/Web/API/Element/contextmenu_event)");
    make_event_impl!("dblclick", until_dblclick, web_sys::MouseEvent, "[MDN documentation for this event](https://developer.mozilla.org/en-US/docs/Web/API/Element/dblclick_event)");
    make_event_impl!("mousedown", until_mousedown, web_sys::MouseEvent, "[MDN documentation for this event](https://developer.mozilla.org/en-US/docs/Web/API/Element/mousedown_event)");
    make_event_impl!("mouseenter", until_mouseenter, web_sys::MouseEvent, "[MDN documentation for this event](https://developer.mozilla.org/en-US/docs/Web/API/Element/mouseenter_event)");
    make_event_impl!("mouseleave", until_mouseleave, web_sys::MouseEvent, "[MDN documentation for this event](https://developer.mozilla.org/en-US/docs/Web/API/Element/mouseleave_event)");
    make_event_impl!("mousemove", until_mousemove, web_sys::MouseEvent, "[MDN documentation for this event](https://developer.mozilla.org/en-US/docs/Web/API/Element/mousemove_event)");
    make_event_impl!("mouseout", until_mouseout, web_sys::MouseEvent, "[MDN documentation for this event](https://developer.mozilla.org/en-US/docs/Web/API/Element/mouseout_event)");
    make_event_impl!("mouseover", until_mouseover, web_sys::MouseEvent, "[MDN documentation for this event](https://developer.mozilla.org/en-US/docs/Web/API/Element/mouseover_event)");
    make_event_impl!("mouseup", until_mouseup, web_sys::MouseEvent, "[MDN documentation for this event](https://developer.mozilla.org/en-US/docs/Web/API/Element/mouseup_event)");

    make_event_impl!("touchcancel", until_touchcancel, web_sys::TouchEvent, "[MDN documentation for this event](https://developer.mozilla.org/en-US/docs/Web/API/Element/touchcancel_event)");
    make_event_impl!("touchend", until_touchend, web_sys::TouchEvent, "[MDN documentation for this event](https://developer.mozilla.org/en-US/docs/Web/API/Element/touchend_event)");
    make_event_impl!("touchmove", until_touchmove, web_sys::TouchEvent, "[MDN documentation for this event](https://developer.mozilla.org/en-US/docs/Web/API/Element/touchmove_event)");
    make_event_impl!("touchstart", until_touchstart, web_sys::TouchEvent, "[MDN documentation for this event](https://developer.mozilla.org/en-US/docs/Web/API/Element/touchstart_event)");

    // make_event_impl!("copy", event_copy, web_sys::ClipboardEvent, "[MDN documentation for this event](https://developer.mozilla.org/en-US/docs/Web/API/Element/copy_event)");
    // make_event_impl!("cut", event_cut, web_sys::ClipboardEvent, "[MDN documentation for this event](https://developer.mozilla.org/en-US/docs/Web/API/Element/cut_event)");
    // make_event_impl!("paste", event_paste, web_sys::ClipboardEvent, "[MDN documentation for this event](https://developer.mozilla.org/en-US/docs/Web/API/Element/paste_event)");
}

impl EmitElementEvent for HtmlElement {}

/// Subscribe to the `input` and `change` events emitted by HTML "editing" elements.
pub trait EmitEditEvent: AsRef<HtmlElement> {
    make_event_impl!("input", until_input, web_sys::Event, "[MDN documentation for this event](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/input_event)");
    make_event_impl!("change", until_change, web_sys::Event, "[MDN documentation for this event](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/change_event)");
}

impl EmitEditEvent for HtmlInputElement {}
impl EmitEditEvent for HtmlTextAreaElement {}
impl EmitEditEvent for HtmlSelectElement {}
