# 🔝 Yew Scroll

[![Crates.io](https://img.shields.io/crates/v/yew-scroll)](https://crates.io/crates/yew-scroll)
[![Crates.io Downloads](https://img.shields.io/crates/d/yew-scroll)](https://crates.io/crates/yew-scroll)
![Crates.io License](https://img.shields.io/crates/l/yew-scroll)
![Rust](https://img.shields.io/badge/rust-stable-orange)

![demo](https://dev-to-uploads.s3.amazonaws.com/uploads/articles/3wyypvi0bgxwsr0i146j.gif)

## 📜 Introduction

This component is designed to provide a convenient and customizable solution for implementing a "scroll to top" button in your Yew applications. Enhance user experience by allowing them to easily navigate to the top of the page with a single click.

## 🤔 Why is this Component Useful?

This component comes with several benefits that make it an essential addition to your Yew projects:

1. 🔄 Scroll Navigation: Simplify user navigation by adding a convenient button to scroll smoothly to the top of the page.

1. 🎨 Customization: Tailor the appearance of the "scroll to top" button using custom CSS classes, and provide a personalized SVG icon.

1. 📏 Customizable Offset: Define a custom vertical offset (Y position) to trigger the visibility of the button, ensuring a seamless user experience.

## ⚙️ Installation

Integrating this component into your Yew project is a straightforward process. Follow these simple steps:

1. Make sure you have Yew set up in your project. If not, refer to the [Yew documentation](https://yew.rs/docs/getting-started/introduction) for installation instructions.

2. Install the component package using your preferred package manager:

   ```bash
   $ cargo add yew-scroll
   ```

3. Import the component into your Yew application and start using it to improve user navigation.

## 🛠️ Usage

Incorporating this component into your application is easy. Follow these steps:

1. Import the component and its required dependencies:

   ```rust
   use yew-scroll::{ScrollToTop, ScrollToTopProps};
   ```

1. Set up the props for the `ScrollToTop` component:

   ```rust
   // Custom SVG content for the scroll-to-top button (an arrow).
   fn custom_svg() -> Html {
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

   // Custom SVG content for the scroll-to-top button (an arrow).
   #[function_component(MyComponent)]
   pub fn my_component() -> Html {

       // Set props for the `ScrollToTop` component
       let custom_props = ScrollToTopProps {
           css: "custom-css",         // Add any custom CSS classes
           top_offset: 0.0,           // Set the desired top offset value to show the button
           svg_content: custom_svg(), // Provide custom SVG content
       };

       // Render the `ScrollToTop` component with the specified props
       html! {
           <>
               // Other content in your component
               <p>{"Scroll down to see the button"}</p>

               // Use the scroll_to_top component
               <ScrollToTop ..custom_props />

               // Default Usage
               <ScrollToTop />
           </>
       }
   }
   ```

1. Customize the appearance and behavior of the "scroll to top" button based on your project requirements.

## 🔧 Props

| Name | Type | Description | Example | Default Value |
| --- | --- | --- | --- | --- |
| `css` | `&'static str` | Custom CSS classes for styling the button. | "custom-scroll-button", "highlight-button". | "fixed bottom-4 right-4 bg-blue-500 text-white p-3 rounded-full cursor-pointer hover:bg-blue-600 transition duration-300 ease-in-out" |
| `top_offset` | `f64` | The vertical offset value (Y position) to trigger button visibility. | 200.0, 300.0 | 500.0 |
| `svg_content` | `Html` | Custom SVG content for the button. | `custom_svg()` | `default_svg()` |

## 🤝 Contribution

We encourage contributions from the community to enhance this Yew component. Feel free to open issues, submit pull requests, or provide feedback. Let's collaborate and make this component even more powerful!

## 📜 License

The Scroll To Top Yew component is licensed under the `MIT` License, allowing you to use, modify, and distribute it freely. Refer to the [`LICENSE`](LICENSE) file for more details.
