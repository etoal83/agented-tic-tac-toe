#![allow(non_snake_case)]
use dioxus::prelude::*;

fn Square(cx: Scope) -> Element {
    cx.render(rsx!(
        button { class: "square",
        }
    ))
}

fn Board(cx: Scope) -> Element {
    #[derive(Props, PartialEq)]
    struct RenderSquareProps {
        nth: i32,
    }

    fn RenderSquare(cx: Scope<RenderSquareProps>) -> Element {
        cx.render(rsx!{
            Square()
        })
    }
    
    let status = "Next Player: X";

    cx.render(rsx!(
        div { class: "status", "{status}" },
        div { class: "board-row",
            RenderSquare { nth: 0 },
            RenderSquare { nth: 1 },
            RenderSquare { nth: 2 },
        },
        div { class: "board-row",
            RenderSquare { nth: 3 },
            RenderSquare { nth: 4 },
            RenderSquare { nth: 5 },
        },
        div { class: "board-row",
            RenderSquare { nth: 6 },
            RenderSquare { nth: 7 },
            RenderSquare { nth: 8 },
        },
    ))
}

fn app(cx: Scope) -> Element {
    cx.render(rsx!{
        div { class: "game",
            div { class: "game-borad",
                Board(),
            },
            div { class: "game-info",
                div {},
                ol {},
            }
        }
    })
}

fn main() {
    dioxus::web::launch(app);
}
