use crate::components::navbar;
use crate::statics;
use actix_web::HttpResponse;

#[derive(PartialEq, Eq)]
pub enum SideBar {
    HighInterestEvents,
    SpaceObjects,
    Conjunctions,
    TrackingDataMessages,
}

markup::define! {
    Header <'a>(title: &'a str) {

        head {
            meta [ charset="utf-8" ] {}
            meta [ "http-equiv"="X-UA-Compatible", content="IE=edge"] {}
            meta [ name="viewport", content="width=device-width, initial-scale=1" ] {}
            title { {title} }

            script [ src = statics::get_index_js(),
                type="text/javascript", async=""] {}

            link [ rel = "stylesheet", type="text/css" , href = statics::get_index_css()] {}
            // Lock everything down.
            //meta ["http-equiv"="Content-Security-Policy", content="default-src 'self'"] {}

            // Favicons.
            link [rel="icon", type="image/svg+xml", href=statics::get_observatory_svg()] {}
            link [rel="alternate icon", href=statics::get_observatory_ico()] {}
        }
    }
    SvgSideMenuItem<'a>(side_bar: SideBar, name: &'a str, link: &'a str,
        svg: &'a str, selected_sidebar: &'a SideBar) {
        @if *selected_sidebar == side_bar {
            li.selected {
                img[alt="Satellite", width = "24px", src = svg] { }
                a[href=link] { {name} }
            }
        } else {
            li {
                img[alt="Satellite", width = "24px", src = svg] { }
                a[href=link] { {name} }
            }
        }
    }
    ApplicationLayout<'a>(content: &'a str, title: &'a str, selected_sidebar: SideBar,
        admin: bool, messaging: bool, show_mockup_screens: bool)
     {
        {markup::doctype()}
        html {
            @Header {
                title,
            }
            body["data-controller" = "upload requests
                orbdetails trackingdetails srfilter 
                cofilter orbfilter trackingfilter usersfilter usersdetails
                deltaflyout messageflyout trajectoryflyout"] {
                @crate::data_upload::DataUploadFlyOut { show_mockup_screens: *show_mockup_screens }
                @crate::mockup::high_interest_events::AttachCCSDSDataFlyout {}
                @crate::mockup::high_interest_events::DeltaNegotiationFlyout {}
                @crate::mockup::high_interest_events::SendMessageFlyout {}
                @crate::mockup::high_interest_events::TrajectoryFlyout {}
                @crate::mockup::service_requests::ScreeningRequestFlyout {}
                @crate::mockup::service_requests::DataValidationRequestFlyout {}
                @crate::conjunctions::ConjunctionFilterFlyOut {}
                @crate::mockup::orbit_data::FilterFlyOut {}
                @crate::mockup::orbit_data::DetailsFlyOut {}
                @crate::mockup::tracking_data::FilterFlyOut {}
                @crate::mockup::tracking_data::DetailsFlyOut {}
                @crate::mockup::service_requests::FilterFlyOut {}
                @crate::mockup::users::DetailsFlyOut {}
                @crate::mockup::users::FilterFlyOut {}
                div.l_application["data-upload-target" = "application",
                    "data-requests-target" = "application",
                    "data-srfilter-target" = "application",
                    "data-cofilter-target" = "application",
                    "data-orbfilter-target" = "application",
                    "data-trackingfilter-target" = "application",
                    "data-orbdetails-target" = "application",
                    "data-usersdetails-target" = "application",
                    "data-usersfilter-target" = "application",
                    "data-deltaflyout-target" = "application",
                    "data-messageflyout-target" = "application",
                    "data-trajectoryflyout-target" = "application",
                    "data-trackingdetails-target" = "application"] {
                    header {
                        ul {
                            li {
                                button.a_button.small["id"="ccsds-data-upload-button"] {
                                    "Upload CCSDS Data"
                                }
                            }

                        }
                        ul {
                            li {
                                a[href=crate::SIGN_OUT_URL] {
                                    img[src=statics::get_logout_svg()] {}
                                }
                            }
                        }
                    }
                    aside.sidenav {
                        h1 {
                            a[href="/app"] { "CREAM" }
                        }
                        h2 {
                            "Space Objects"
                        }
                        ul {


                            @if *show_mockup_screens {
                                { SideMenuItem { side_bar: SideBar::Dashboard, name: "Dashboard",
                                    link: crate::mockup::dashboard::INDEX_URL,
                                    letters: "D", selected_sidebar  } }
                            }

                            { SvgSideMenuItem { side_bar: SideBar::SpaceObjects, name: "Space Objects",
                                link: crate::space_objects::SPACE_OBJECTS_URL,
                                svg: &statics::get_space_object_svg(), selected_sidebar  } }

                            @if *show_mockup_screens {

                                { SideMenuItem { side_bar: SideBar::HighInterestEvents, name: "High Interest Events",
                                    link: crate::mockup::high_interest_events::NEGOTIATIONS_URL,
                                    letters: "HIE", selected_sidebar  } }

                                { SideMenuItem { side_bar: SideBar::ServiceRequests, name: "Service Requests",
                                    link: crate::mockup::service_requests::INDEX_URL,
                                    letters: "N", selected_sidebar  } }

                                { SideMenuItem { side_bar: SideBar::PhoneBook, name: "Phone Book",
                                    link: crate::mockup::phonebook::INDEX_URL,
                                    letters: "PB", selected_sidebar  } }

                                { SideMenuItem { side_bar: SideBar::Wallet, name: "Wallet",
                                    link: crate::mockup::wallet::INDEX_URL,
                                    letters: "N", selected_sidebar  } }
                            }
                        }
                        h2 {
                            "CCSDS Data"
                        }
                        ul {

                            { SvgSideMenuItem { side_bar: SideBar::Conjunctions, name: "Conjunctions",
                                link: crate::conjunctions::CONJUNCTIONS_URL,
                                svg: &statics::get_conjunctions_svg(), selected_sidebar  } }

                            @if *show_mockup_screens {
                                { SideMenuItem { side_bar: SideBar::OrbitDataMessages, name: "Orbit Data",
                                    link: crate::mockup::orbit_data::INDEX_URL,
                                    letters: "OD", selected_sidebar  } }

                                { SideMenuItem { side_bar: SideBar::TrackingDataMessages, name: "Tracking Data",
                                    link: crate::mockup::tracking_data::INDEX_URL,
                                    letters: "NDM", selected_sidebar  } }
                            }
                        }
                        @if *admin {

                            h2 {
                                "Administrator"
                            }
                            ul {
                                { SideMenuItem { side_bar: SideBar::RegistrationRequests, name: "Registration Requests",
                                    link: crate::mockup::registration_requests::INDEX_URL,
                                    letters: "ANU", selected_sidebar  } }
                                { SideMenuItem { side_bar: SideBar::Users, name: "User Admin",
                                    link: crate::mockup::users::INDEX_URL,
                                    letters: "UA", selected_sidebar  } }
                            }
                        } else {
                                h2 {
                                    "Settings"
                                }
                                ul {
                                    { SvgSideMenuItem { side_bar: SideBar::Profile, name: "Your Keys",
                                        link: crate::encryption_keys::INDEX_URL,
                                        svg: &statics::get_encryption_keys_svg(), selected_sidebar  } }

                                    @if *show_mockup_screens {
                                        { SideMenuItem { side_bar: SideBar::Team, name: "Your Delegations",
                                            link: crate::mockup::team::INDEX_URL,
                                            letters: "T", selected_sidebar  } }


                                        { SideMenuItem { side_bar: SideBar::ApiKeys, name: "API Keys",
                                            link: crate::mockup::api_keys::INDEX_URL,
                                            letters: "PO", selected_sidebar  } }
                                    }
                                }
                        }
                    }
                    main.container {
                        @if *messaging {
                            section.messaging {
                                div {
                                    {markup::raw(content)}
                                }
                            }
                        } else {
                            section.content {
                                div {
                                    {markup::raw(content)}
                                }
                            }
                        }
                    }
                    @SnackBar {}
                }
            }
        }
    }
}

pub fn application_layout(title: &str, content: &str, selected_sidebar: SideBar) -> HttpResponse {
    let l = ApplicationLayout {
        content,
        title,
        selected_sidebar,
        admin: false,
        messaging: false,
        show_mockup_screens: true,
    };
    HttpResponse::Ok()
        .content_type("text/html")
        .body(l.to_string())
}