
slint::slint! {
    import { VerticalBox, Button, HorizontalBox, ScrollView } from "std-widgets.slint";

    export component AppWindow inherits Window {
        background: transparent;
        title: "LFR: Longhorn First Rebirth v0.7.0 Global";
        width: 700px;
        height: 600px;

        in-out property <string> welcome_text: "Aero Glass Mode: ON"; 
        in-out property <bool> night-mode: true;

        Rectangle {
            width: 100%;
            height: 100%;
            background: night-mode ? rgba(30, 30, 35, 0.85) : rgba(240, 240, 255, 0.7);
            border-radius: 20px;
            border-width: 1.5px;
            border-color: night-mode ? rgba(255, 255, 255, 0.25) : rgba(0, 0, 0, 0.2);
            
            animate background { duration: 500ms; easing: ease-in-out; }

            VerticalBox {
                padding: 30px;
                spacing: 20px;
                
                Text {
                    text: "Liquid Glass Engine v0.6.5";
                    color: #00bfff;
                    font-size: 32px;
                    font-weight: 800;
                    horizontal-alignment: center;
                }

                Text {
                    text: root.welcome_text;
                    color: night-mode ? white : black;
                    font-size: 22px;
                    horizontal-alignment: center;
                }

                VerticalBox {
                    alignment: center;
                    spacing: 12px;
                    
                    HorizontalBox {
                        alignment: center;
                        spacing: 8px;
                        Button { text: "UA"; clicked => { root.welcome_text = "Аеро Скло: Увімкнено"; } }
                        Button { text: "PL"; clicked => { root.welcome_text = "Szkło Aero: Włączone"; } }
                        Button { text: "EN"; clicked => { root.welcome_text = "Aero Glass Mode: ON"; } }
                        Button { text: "IT"; clicked => { root.welcome_text = "Modalità Vetro Aero: ON"; } }
                        Button { text: "FR"; clicked => { root.welcome_text = "Mode Verre Aero: Activé"; } }
                    }

                    HorizontalBox {
                        alignment: center;
                        spacing: 8px;
                        Button { text: "CN"; clicked => { root.welcome_text = "航空玻璃模式：開啟"; } }
                        Button { text: "AR"; clicked => { root.welcome_text = "وضع الزجاج الهوائي: قيد التشغيل"; } }
                        Button { text: "JP"; clicked => { root.welcome_text = "エアログラスモード：オン"; } }
                    }
                }

                HorizontalBox {
                    alignment: center;
                    spacing: 20px;
                    Button { 
                        text: "☀️ Light Mode"; 
                        clicked => { root.night-mode = false; }
                    }
                    Button { 
                        text: "🌙 Night Mode"; 
                        clicked => { root.night-mode = true; }
                    }
                }
            }
        }
    }
}

fn main() -> Result<(), slint::PlatformError> {
    unsafe {
        std::env::set_var("SLINT_BACKEND", "software");
    }
    
    let ui = AppWindow::new()?;
    
    println!("LFR v0.6.5 запущено успішно на базі Liquid Glass Engine.");
    
    ui.run()
}
