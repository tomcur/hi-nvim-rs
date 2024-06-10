use std::{
    sync::OnceLock,
    time::{Duration, SystemTime},
};

use axum::{
    extract::Query,
    headers,
    http::{header, HeaderMap, HeaderName},
    response::{Html, IntoResponse},
    routing::{get, post},
    Form, Router,
};
use clap::{arg, command, Parser};
use serde::Deserialize;
use tower_http::{compression::CompressionLayer, timeout::TimeoutLayer};

mod assets;
mod cache_control;
mod colorschemes;
mod error;
mod examples;

use colorschemes::Colorscheme;
use examples::Language;

pub static ASSET_LAST_MODIFIED: OnceLock<headers::LastModified> = OnceLock::new();

#[derive(Clone, Copy)]
pub struct PreviewData<'a> {
    theme: &'a str,
    language: Language,
}

static DESCRIPTION: &'static str = "With this tool you can create Neovim color schemes by defining colors and their transformations in a perceptual color space: Oklab.";

const HEADER_VARIES_HTMX: (HeaderName, &str) = (header::VARY, "hx-request");

markup::define! {
    Index<'a>(style: &'a str, configuration: &'a str, name: &'a str, compiled_colorscheme: &'a str, data: PreviewData<'a>) {
        @markup::doctype()
        html [style=style, lang="en"] {
            head {
                meta[charset="utf-8"];
                meta [name="viewport", content="width=device-width"];
                meta [name="description", content=DESCRIPTION];
                link[rel="stylesheet", href="/style.css"];
                link[rel="icon", type="image/svg+xml", href="/favicon.svg"];
                title { "hi.nvim.rs – (Neo)vim perceptual color scheme compiler" }
            }
            body ["hx-swap"="none"] {
                @Header {}
                main {
                    article {
                        p { @DESCRIPTION }
                        p {
                            "Change the configuration below and hit the \"Compile\" button.
                                Because the color scheme is compiled beforehand, Neovim only needs
                                to load the highlights at startup."
                        }
                        p {
                            "A CLI tool for local compilation is available: see the "
                            a [href="https://github.com/tomcur/hi-nvim-rs"] {
                                "repository"
                            }
                            ". Vim is supported as a secondary target."
                        }
                    }
                    section {
                        @ColorschemeSelector {}
                        @PreviewArticle { data: *data }
                        @Configure { initial_value: configuration }
                        @Install {
                            compilation_target: Target::Neovim,
                            name,
                            compiled_colorscheme,
                        }
                    }
                }
                @Footer {}
                script [src="/htmx.min.js"] {}
                script [src="/main.js"] {}
            }
        }
    }

    ColorschemeSelector() {
        nav {
            header {
                h2 { "Example color schemes" }
            }
            ul ["hx-boost"="true"] {
                li {
                    a [href="/?colorscheme=Highlow"] { "Highlow" }
                }
                li {
                    a [href="/?colorscheme=Twocolor"] { "Twocolor" }
                }
            }
        }
        // form [ method="get" ]
        // {
        //     label {
        //         "Color scheme "
        //         select #"colorscheme-selector" [
        //             name="colorscheme",
        //             autocomplete="off", // reset on page refresh
        //             "hx-get"="/colorscheme",
        //             "hx-params"="colorscheme",
        //             "hx-swap"="none",
        //             "hx-confirm"="bla"
        //         ] {
        //             option [value=Colorscheme::Highlow, selected="selected"] { @Colorscheme::Highlow }
        //             option [value=Colorscheme::Twocolor] { @Colorscheme::Twocolor }
        //         }
        //     }
        // }
    }

    ColorschemeConfig<'a>(initial_value: &'a str) {
        label #"colorscheme-configuration" [
            "hx-swap-oob"="true",
        ] {
            "Color scheme configuration"
            textarea [
            rows="17",
            name="configuration",
            spellcheck="false",
        ] {
                @initial_value
            }
        }
    }

    Configure<'a>(initial_value: &'a str) {
        article {
            h2 { "Configure" }
            form [
                method="post",
                "hx-post"="/colorscheme",
                "hx-swap"="none",
            ]
            {
                label {
                    "Preview language "
                    select [
                        name="language",
                        autocomplete="off", // reset on page refresh
                        "hx-get"="/language",
                        "hx-params"="language",
                        "hx-swap"="innerHTML",
                        "hx-target"="#preview-container",
                    ] {
                        option [value=Language::C] { @Language::C }
                        option [value=Language::Rust, selected="selected"] { @Language::Rust }
                        option [value=Language::Typescript] { @Language::Typescript }
                        option [value=Language::Tsx] { @Language::Tsx }
                    }
                }
                fieldset {
                    label {
                        "Compilation target "
                        select [
                            name="compilation_target",
                            autocomplete="off", // reset on page refresh
                            "hx-post"="/colorscheme",
                        ] {
                            option [value="Neovim", selected="selected"] { "Neovim" }
                            option [value="Vim"] { "Vim" }
                        }
                    }
                    @ColorschemeConfig { initial_value }
                }
                input [type="submit", value="Compile"];
            }
            section #error {}
        }
    }

    Preview(language: Language) {
        section .buffer {
            pre .linenr {
                @language.code_example().lines
            }
            article {
                code {
                    pre {
                        @markup::raw(language.code_example().code)
                    }
                }
            }
        }
        footer .statusline {
            span {
                @language.code_example().file_name
            }
            span {
                "["
                @language.code_example().language_name
                "]"
            }
        }
        section .msgarea {
            ":colorscheme example"
        }
    }

    PreviewContainerInner(language: Language) {
        article #preview1 .preview {
            h2 { "Normal" }
            div { @Preview { language: *language } }
        }

        article #preview2 .preview .inverse {
            h2 { "Inverse" }
            div { @Preview { language: *language } }
        }
    }

    PreviewContainer<'a>(data: PreviewData<'a>) {
        div #"preview-container" [
            style=data.theme,
            "hx-swap-oob"="true",
            "hx-on::oob-before-swap"="setColorscheme(event)",
        ] {
            @PreviewContainerInner { language: data.language }
        }
    }

    EmptyPreviewContainer<'a>(data: PreviewData<'a>) {
        div #"preview-container" [style=data.theme, "hx-swap-oob"="true"] { }
    }

    PreviewArticle<'a>(data: PreviewData<'a>) {
        article {
            @PreviewContainer { data: *data }
        }
    }

    Install<'a>(compilation_target: Target, name: &'a str, compiled_colorscheme: &'a str) {
        article #install ["hx-swap-oob"="true"] {
            h2 { "Install" }
            @match compilation_target {
                Target::Neovim => {
                    p { "Create a file called "
                        code { @name ".vim" }
                        "  in "
                        code { "~/.config/nvim/colors" }
                        " with the following content. This is where Neovim looks for color schemes."
                    }
                }
                Target::Vim => {
                    p { "Create a file called "
                        code { @name ".vim" }
                        "  in "
                        code { "~/.vim/colors" }
                        " with the following content. This is where Vim looks for color schemes."
                    }
                }
            }
            label {
                "Compiled color scheme"
                textarea [readonly, rows="15"] { @compiled_colorscheme }
            }
        }
    }

    Header() {
        header ."site-header" {
            h1 {
                "hi.nvim.rs"
            }
            p {
                "Neovim perceptual color scheme compiler"
            }
        }
    }

    Footer() {
        footer {
            a [href="https://github.com/tomcur/hi.nvim.rs"] {
                { assets::Github {} }
                " hi.nvim.rs "
                { env!("CARGO_PKG_VERSION") }
            }
        }
    }
}

#[derive(Clone, Copy, Deserialize)]
pub enum Target {
    Neovim,
    Vim,
}

impl From<Target> for hi_nvim_rs_web_styler::Target {
    fn from(value: Target) -> Self {
        match value {
            Target::Neovim => hi_nvim_rs_web_styler::Target::Neovim,
            Target::Vim => hi_nvim_rs_web_styler::Target::Vim,
        }
    }
}

#[derive(Deserialize)]
struct LanguagePayload {
    language: Language,
}

async fn language(Form(payload): Form<LanguagePayload>) -> impl IntoResponse {
    let template = PreviewContainerInner {
        language: payload.language,
    };
    Html(template.to_string())
}

#[derive(Deserialize)]
struct ColorschemePayload {
    configuration: String,
    language: Language,
    compilation_target: Target,
}

async fn colorscheme(Form(payload): Form<ColorschemePayload>) -> impl IntoResponse {
    let ColorschemePayload {
        configuration,
        language,
        compilation_target,
    } = payload;

    match hi_nvim_rs_web_styler::style(&configuration, compilation_target.into()) {
        Ok(hi_nvim_rs_web_styler::Compiled {
            name,
            css_variables,
            compiled_colorscheme,
            ..
        }) => {
            let data = PreviewData {
                theme: &css_variables,
                language,
            };

            let template = markup::new! {
                @PreviewContainer { data }
                section #error {}
                @Install {
                    compilation_target: compilation_target,
                    name,
                    compiled_colorscheme: &compiled_colorscheme,
                }
            };

            Html(template.to_string())
        }
        Err(err) => {
            let template = markup::new! {
                section #error ["hx-swap-oob"="true"] {
                    p { @err.to_string() }
                }
            };

            Html(template.to_string())
        }
    }
}

async fn index_post(Form(payload): Form<ColorschemePayload>) -> impl IntoResponse {
    let hi_nvim_rs_web_styler::Compiled {
        name,
        css_variables,
        compiled_colorscheme,
        ..
    } = hi_nvim_rs_web_styler::style(&payload.configuration, payload.compilation_target.into())
        .unwrap();

    let data = PreviewData {
        theme: &css_variables,
        language: payload.language,
    };

    let style = include_str!(concat!(env!("OUT_DIR"), "/site_color_scheme.txt"));
    let template = Index {
        style,
        configuration: &payload.configuration,
        name,
        compiled_colorscheme: &compiled_colorscheme,
        data,
    };

    Html(template.to_string())
}

#[derive(Deserialize)]
struct DefaultColorschemePayload {
    colorscheme: Colorscheme,
}

async fn default_colorscheme(Form(payload): Form<DefaultColorschemePayload>) -> impl IntoResponse {
    let colorscheme = payload.colorscheme.colorscheme();

    let data = PreviewData {
        theme: &colorscheme.css_variables,
        language: Language::default(),
    };

    let template = markup::new! {
        @EmptyPreviewContainer { data }
        section #error {}
        @ColorschemeConfig { initial_value: colorscheme.config }
        @Install {
            compilation_target: Target::Neovim,
            name: colorscheme.name,
            compiled_colorscheme: colorscheme.neovim_config,
        }
    };

    Html(template.to_string())
}

#[derive(Deserialize)]
struct IndexQuery {
    colorscheme: Option<Colorscheme>,
}

async fn index(Query(query): Query<IndexQuery>, headers: HeaderMap) -> impl IntoResponse {
    let is_htmx = headers
        .get("hx-request")
        .map(|v| v == "true")
        .unwrap_or(false);
    let colorscheme = query.colorscheme.unwrap_or_default().colorscheme();

    let data = PreviewData {
        theme: colorscheme.css_variables,
        language: Language::Rust,
    };

    if is_htmx {
        let template = markup::new! {
            @EmptyPreviewContainer { data }
            section #error {}
            @ColorschemeConfig { initial_value: colorscheme.config }
            @Install {
                compilation_target: Target::Neovim,
                name: colorscheme.name,
                compiled_colorscheme: colorscheme.neovim_config,
            }
        };
        ([HEADER_VARIES_HTMX], Html(template.to_string()))
    } else {
        let style = include_str!(concat!(env!("OUT_DIR"), "/site_color_scheme.txt"));
        let template = Index {
            style,
            configuration: colorscheme.config,
            name: "hl",
            compiled_colorscheme: colorscheme.neovim_config,
            data,
        };
        ([HEADER_VARIES_HTMX], Html(template.to_string()))
    }
}

/// Start the hi-nvim-rs web server.
#[derive(Parser, Debug)]
#[command(version)]
struct Cli {
    /// The web server listen address.
    #[arg(short, long, value_name = "target", default_value = "127.0.0.1")]
    address: std::net::IpAddr,

    /// The web server listen port.
    #[arg(short, long, value_name = "target", default_value = "3000")]
    port: u16,
}

#[tokio::main]
async fn main() {
    ASSET_LAST_MODIFIED
        .set(headers::LastModified::from(SystemTime::now()))
        .unwrap();

    let cli = Cli::parse();

    let static_assets = Router::new()
        .route("/", get(index))
        .route("/favicon.svg", get(assets::favicon))
        .route("/style.css", get(assets::css))
        .route("/htmx.min.js", get(assets::htmx))
        .route("/main.js", get(assets::js))
        .route("/language", get(language))
        .route("/colorscheme", get(default_colorscheme))
        .layer(cache_control::CacheControlLayer::new());

    let app = Router::new()
        .merge(static_assets)
        .route("/", post(index_post))
        .route("/colorscheme", post(colorscheme))
        .layer(TimeoutLayer::new(Duration::from_secs(30)))
        .layer(CompressionLayer::new());

    axum::Server::bind(&std::net::SocketAddr::new(cli.address, cli.port))
        .serve(app.into_make_service())
        .await
        .unwrap();
}
