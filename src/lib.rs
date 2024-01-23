use gloo::events::EventListener;
use gloo::utils::window;
use yew::prelude::*;

/// Constant defining default Tailwind CSS classes for the scroll-to-top button.
const SCROLL_TO_TOP_CLASSES: &'static str =
    "fixed bottom-4 right-4 bg-blue-500 text-white p-3 rounded-full cursor-pointer hover:bg-blue-600 transition duration-300 ease-in-out";

#[derive(Properties, Clone, PartialEq)]
pub struct ScrollToTopProps {
    /// Custom CSS classes for styling the scroll-to-top button.
    #[prop_or(SCROLL_TO_TOP_CLASSES)]
    pub css: &'static str,

    /// The vertical offset value (Y position) to show the button.
    #[prop_or(500.0)]
    pub top_offset: f64,

    /// Custom SVG content for the scroll-to-top button.
    #[prop_or_else(default_svg)]
    pub svg_content: Html,
}

/// scroll_to_top
/// A Yew component that provides a button to scroll to the top of the page when clicked.
///
/// # Arguments
/// * `props` - The properties of the component.
///   - `css` - Custom CSS classes for styling the scroll-to-top button. Defaults to predefined Tailwind classes.
///   - `top_offset` - The vertical offset value (Y position) to show the button. Defaults to 500.0 pixels.
///   - `svg_content` - Custom SVG content for the scroll-to-top button. Defaults to a default arrow SVG.
///
/// # Returns
/// (Html): An HTML representation of the scroll-to-top button.
///
/// # Examples
/// ```
/// // Example of using the scroll_to_top component
/// use yew::prelude::*;
/// use yew_scroll::{ScrollToTop, ScrollToTopProps};
///
/// // Custom SVG content for the scroll-to-top button (an arrow).
/// fn custom_svg() -> Html {
///     html! {
///         <svg
///             class="w-6 h-6"
///             fill="none"
///             stroke="currentColor"
///             viewBox="0 0 24 24"
///             xmlns="http://www.w3.org/2000/svg"
///         >
///             <path
///                 stroke-linecap="round"
///                 stroke-linejoin="round"
///                 stroke-width="2"
///                 d="M5 10l7-7m0 0l7 7m-7-7v18"
///             />
///         </svg>
///     }
/// }
///
/// #[function_component(MyComponent)]
/// pub fn my_component() -> Html {
///     // Set props for the scroll_to_top component
///     let scroll_to_top_props = ScrollToTopProps {
///         css: "custom-css",         // Add any custom CSS classes
///         top_offset: 0.0,           // Set the desired top offset value to show the button
///         svg_content: custom_svg(), // Provide custom SVG content
///     };
///
///     // Render the scroll_to_top component with the specified props
///     html! {
///         <>
///             // Other content in your component
///             <p>{"Scroll down to see the button"}</p>
///
///             // Use the scroll_to_top component
///             <ScrollToTop ..scroll_to_top_props />
///
///             // Default Usage
///             <ScrollToTop />
///         </>
///     }
/// }
/// ```
#[function_component(ScrollToTop)]
pub fn scroll_to_top(props: &ScrollToTopProps) -> Html {
    // State handle to track the visibility of the scroll-to-top button.
    let visible_handle = use_state(|| false);
    let visible = *visible_handle;
    let top_offset = props.top_offset.clone();

    // Effect to add a scroll event listener and update the visibility state.
    use_effect(move || {
        let listener = EventListener::new(&window(), "scroll", move |_| {
            let scroll_position = window().scroll_y().unwrap_or_default();
            visible_handle.set(scroll_position > top_offset);
        });

        // Cleanup when the component is unmounted.
        move || {
            drop(listener);
        }
    });

    // Callback for the button click event to scroll to the top.
    let on_click = Callback::from(|_| {
        let win = window();
        win.scroll_to_with_x_and_y(0.0, 0.0);
    });

    html! {
        if visible {
            <div class={SCROLL_TO_TOP_CLASSES} onclick={on_click}>{ props.svg_content.clone() }</div>
        }
    }
}

/// Default SVG content for the scroll-to-top button (an arrow).
fn default_svg() -> Html {
    html! {
        <svg
            class="w-6 h-6"
            fill="none"
            stroke="currentColor"
            viewBox="0 0 24 24"
            xmlns="http://www.w3.org/2000/svg"
        >
            <path
                stroke-linecap="round"
                stroke-linejoin="round"
                stroke-width="2"
                d="M5 10l7-7m0 0l7 7m-7-7v18"
            />
        </svg>
    }
}
