use yew::prelude::*;
use yew_icons::{Icon, IconId};

#[function_component(Navbar)]
pub fn navbar() -> Html {
    html! {
        <>
        <ybc::Navbar
        classes={classes!("is-light")}
        padded=true
        navbrand={html!{
            <>
                <ybc::NavbarItem>
                    <ybc::ButtonAnchor
                    classes={classes!("is-black", "is-outlined")}
                    href="/">
                        <Icon icon_id={IconId::BootstrapPersonCircle}/>
                    </ybc::ButtonAnchor>
                </ybc::NavbarItem>
                <ybc::NavbarItem>
                    <ybc::ButtonAnchor
                    classes={classes!("is-black", "is-outlined")}
                    rel={String::from("noopener noreferrer")}
                    target={String::from("_blank")}
                    href="/static/resume.pdf">
                        <Icon icon_id={IconId::HeroiconsMiniSolidDocumentText}/>
                    </ybc::ButtonAnchor>
                </ybc::NavbarItem>
            </>
        }}
        navend={html!{
            <>
            <ybc::NavbarItem>
                <ybc::ButtonAnchor
                classes={classes!("is-black", "is-outlined")}
                rel={String::from("noopener noreferrer")}
                target={String::from("_blank")}
                href="https://www.linkedin.com/in/jason-chong-14962328b">
                    <Icon icon_id={IconId::BootstrapLinkedin}/>
                </ybc::ButtonAnchor>
            </ybc::NavbarItem>
            <ybc::NavbarItem>
                <ybc::ButtonAnchor
                classes={classes!("is-black", "is-outlined")}
                rel={String::from("noopener noreferrer")}
                target={String::from("_blank")}
                href="https://github.com/reaovyd">
                    <Icon icon_id={IconId::BootstrapGithub}/>
                </ybc::ButtonAnchor>
            </ybc::NavbarItem>
            <ybc::NavbarItem>
                <ybc::ButtonAnchor
                classes={classes!("is-black", "is-outlined")}
                rel={String::from("noopener noreferrer")}
                target={String::from("_blank")}
                href="https://twitter.com/">
                    <Icon icon_id={IconId::BootstrapTwitter}/>
                </ybc::ButtonAnchor>
            </ybc::NavbarItem>
            <ybc::NavbarItem>
                <ybc::ButtonAnchor
                classes={classes!("is-black", "is-outlined")}
                rel={String::from("noopener noreferrer")}
                target={String::from("_blank")}
                href="mailto:zackbellzokobell@gmail.com">
                    <Icon icon_id={IconId::LucideMail}/>
                </ybc::ButtonAnchor>
            </ybc::NavbarItem>
            </>
        }}
        />
        </>
    }
}
