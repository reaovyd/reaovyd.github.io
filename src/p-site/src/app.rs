use yew::prelude::*;

use crate::navbar::Navbar;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <>
            <>
            <Navbar>
            </Navbar>
            </>
            <>
                    <div id={"wrapper"}>
                        <ybc::Section
                        classes={classes!("hero", "is-small", "is-bold", "has-text-centered", "is-size-3")}
                        >
                            <p class={"is-family-code is-size-2"}>
                                {"Jason Chong"}
                            </p>
                            <p class={"is-family-code is-size-7"}>
                                {"Software Engineer"}
                            </p>
                            <ybc::Section
                            classes={classes!("hero", "is-small", "is-bold", "has-text-left")}
                            >
                                <ybc::Box>
                                    <article class={"media"}>
                                        <div class="media-left">
                                            <img src={"/static/meself.png"} alt={"me"} style={"border-radius: 50%; margin: 0 auto; width: 5em; height: 5em;"} />
                                        </div>
                                        <div class={"media-content"}>
                                            <div class={"content is-size-6"}>
                                                <p style={"overflow-wrap: break-word; "} >
                                                    {"Hello there! My name is Jason, and I'm currently an"}
                                                    {" Undergraduate in my 4th year pursuing a Bachelor's Degree in Computer Science at "}
                                                    <a href={"https://www.stonybrook.edu/"}>{"Stony Brook University"}</a>
                                                    {". Previously, I worked on "}
                                                    <a href={"https://www.amazon.com/"}>{"Amazon"}</a>
                                                    {"'s Seller Wallet product and "}
                                                    <a href={"https://www.global-infra.com/"}>{"GIP"}</a>
                                                    {"'s internal core infrastructure as a Software Engineer Intern."}
                                                    {" Currently, my interests lie in operating systems, Rust, Go, distributed systems, modern web and cloud technologies that I want to explore and learn "}
                                                    {"more about. "}
                                                </p>
                                            </div>
                                        </div>
                                    </article>
                                    <article style={"hyphens: auto; font-size: 12px;"}>
                                    </article>
                                </ybc::Box>
                            </ybc::Section>
                        </ybc::Section>
                    </div>
            </>
            <>
                <ybc::Footer
                classes={classes!("footer")}
                >
                    <div
                    class={"content has-text-centered mt-auto"}>
                        <p
                        class={"has-text-weight-light"}
                        style={"font-size: 8px;"}
                        >
                            {"Icons by "} <a href={"https://finnbear.github.io/yew_icons/"}>{"yew_icons"}</a><br/>
                            {"Frankensteined with "} <a href={"https://bulma.io"}>{"Bulma"}</a><br/>
                        </p>
                    </div>
                </ybc::Footer>
            </>
        </>
    }
}
