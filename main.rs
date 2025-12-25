slint::slint! {
    import { VerticalBox, Button, HorizontalBox } from "std-widgets.slint";

    export component AppWindow inherits Window {
        background: night-mode ? #1c1c22 : #edeff5;
        title: "Longhorn First Rebirth v0.6.5 (Classic Stable)";
        width: 500px;
        height: 450px;

        in-out property <bool> night-mode: false;
        in-out property <string> welcome_text: "Simple, less transparent Aero";

        VerticalBox {
            padding: 25px;
            spacing: 20px;

            Text {
                text: "Liquid Glass Engine";
                color: #00a2ed;
                font-size: 26px;
                horizontal-alignment: center;
            }

            Text {
                text: root.welcome_text;
                color: night-mode ? white : black;
                font-size: 18px;
                horizontal-alignment: center;
            }

            Rectangle {
                background: night-mode ? rgba(255, 255, 255, 0.05) : rgba(0, 0, 0, 0.03);
                border-radius: 12px;
                padding: 15px;

                VerticalBox {
                    spacing: 15px;
                    HorizontalBox {
                        alignment: center;
                        spacing: 10px;
                        Button { text: "UA"; clicked => { root.welcome_text = "Просте, менш прозоре Аеро"; } }
                        Button { text: "PL"; clicked => { root.welcome_text = "Proste, mniej przezroczyste Aero"; } }
                        Button { text: "EN"; clicked => { root.welcome_text = "Simple, less transparent Aero"; } }
                    }

                    HorizontalBox {
                        alignment: center;
                        spacing: 10px;
                        Button { 
                            text: "Light Mode"; 
                            clicked => { root.night-mode = false; } 
                        }
                        Button { 
                            text: "Night Mode"; 
                            clicked => { root.night-mode = true; } 
                        }
                    }
                }
            }

            Text {
                text: "v0.6.5 Build (Based on v0.6.0 Stability)";
                color: gray;
                font-size: 11px;
                horizontal-alignment: center;
            }
        }
    }
}

fn main() -> Result<(), slint::PlatformError> {
    unsafe { std::env::set_var("SLINT_BACKEND", "software"); }
    let ui = AppWindow::new()?;
    ui.run()
}
