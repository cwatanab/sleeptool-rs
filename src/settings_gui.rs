use std::sync::Arc;
use crate::config::Config;
use crate::state::SharedState;
use crate::platform_win32::WindowsPlatform;
use crate::platform::Platform;

slint::slint! {
    import { Button, CheckBox, ComboBox, LineEdit, Slider, ScrollView, GroupBox } from "std-widgets.slint";

    component SidebarButton inherits Rectangle {
        in-out property <string> text;
        in-out property <bool> active;
        in-out property <image> icon;
        callback clicked();

        border-radius: 6px;
        background: active ? rgb(62, 62, 74) : (touch.has-hover ? rgb(48, 48, 58) : transparent);
        height: 36px;

        HorizontalLayout {
            padding-left: 12px;
            padding-right: 12px;
            spacing: 10px;

            VerticalLayout {
                alignment: center;
                Image {
                    source: root.icon;
                    width: 18px;
                    height: 18px;
                }
            }

            VerticalLayout {
                alignment: center;
                Text {
                    text: root.text;
                    color: root.active ? rgb(255, 255, 255) : rgb(192, 192, 192);
                    font-weight: root.active ? 600 : 400;
                }
            }
        }

        touch := TouchArea {
            clicked => { root.clicked(); }
        }
    }

    component Bar inherits Rectangle {
        in property <float> val: 0.0;
        in property <float> max-val: 100.0;
        in property <length> chart-height: 18px;
        in property <bool> show-threshold: false;
        in property <float> threshold-ratio: 0.0;
        width: 6px;
        height: root.chart-height * clamp(root.val / root.max-val, 0.04, 1.0);
        border-radius: 2px;
        background: root.show-threshold && root.val / root.max-val > root.threshold-ratio
            ? @linear-gradient(180deg, rgb(249, 115, 22) 0%, rgb(251, 146, 60) 50%, rgb(253, 186, 116) 100%)
            : @linear-gradient(180deg, rgb(99, 102, 241) 0%, rgb(129, 140, 248) 50%, rgb(165, 180, 252) 100%);
    }

    component Sparkline inherits Rectangle {
        in property <float> max-val: 100.0;
        in property <float> val0: 0.0;
        in property <float> val1: 0.0;
        in property <float> val2: 0.0;
        in property <float> val3: 0.0;
        in property <float> val4: 0.0;
        in property <float> val5: 0.0;
        in property <float> val6: 0.0;
        in property <float> val7: 0.0;
        in property <float> val8: 0.0;
        in property <float> val9: 0.0;
        in property <float> val10: 0.0;
        in property <float> val11: 0.0;
        in property <float> val12: 0.0;
        in property <float> val13: 0.0;
        in property <float> val14: 0.0;
        in property <float> val15: 0.0;
        in property <float> val16: 0.0;
        in property <float> val17: 0.0;
        in property <float> val18: 0.0;
        in property <float> val19: 0.0;
        in property <float> val20: 0.0;
        in property <float> val21: 0.0;
        in property <float> val22: 0.0;
        in property <float> val23: 0.0;
        in property <float> val24: 0.0;
        in property <float> val25: 0.0;
        in property <float> val26: 0.0;
        in property <float> val27: 0.0;
        in property <float> val28: 0.0;
        in property <float> val29: 0.0;

        in property <length> chart-height: 18px;
        in property <bool> show-threshold: false;
        in property <float> threshold-ratio: 0.0;

        height: 20px;
        width: 100px;
        background: rgb(14, 14, 20);
        border-radius: 6px;
        border-width: 1px;
        border-color: rgb(38, 38, 50);

        HorizontalLayout {
            padding-left: 3px;
            padding-right: 3px;
            padding-top: 1px;
            padding-bottom: 1px;
            spacing: 1px;

            VerticalLayout { alignment: end; Bar { val: root.val14; max-val: root.max-val; chart-height: root.chart-height; show-threshold: root.show-threshold; threshold-ratio: root.threshold-ratio; } }
            VerticalLayout { alignment: end; Bar { val: root.val13; max-val: root.max-val; chart-height: root.chart-height; show-threshold: root.show-threshold; threshold-ratio: root.threshold-ratio; } }
            VerticalLayout { alignment: end; Bar { val: root.val12; max-val: root.max-val; chart-height: root.chart-height; show-threshold: root.show-threshold; threshold-ratio: root.threshold-ratio; } }
            VerticalLayout { alignment: end; Bar { val: root.val11; max-val: root.max-val; chart-height: root.chart-height; show-threshold: root.show-threshold; threshold-ratio: root.threshold-ratio; } }
            VerticalLayout { alignment: end; Bar { val: root.val10; max-val: root.max-val; chart-height: root.chart-height; show-threshold: root.show-threshold; threshold-ratio: root.threshold-ratio; } }
            VerticalLayout { alignment: end; Bar { val: root.val9; max-val: root.max-val; chart-height: root.chart-height; show-threshold: root.show-threshold; threshold-ratio: root.threshold-ratio; } }
            VerticalLayout { alignment: end; Bar { val: root.val8; max-val: root.max-val; chart-height: root.chart-height; show-threshold: root.show-threshold; threshold-ratio: root.threshold-ratio; } }
            VerticalLayout { alignment: end; Bar { val: root.val7; max-val: root.max-val; chart-height: root.chart-height; show-threshold: root.show-threshold; threshold-ratio: root.threshold-ratio; } }
            VerticalLayout { alignment: end; Bar { val: root.val6; max-val: root.max-val; chart-height: root.chart-height; show-threshold: root.show-threshold; threshold-ratio: root.threshold-ratio; } }
            VerticalLayout { alignment: end; Bar { val: root.val5; max-val: root.max-val; chart-height: root.chart-height; show-threshold: root.show-threshold; threshold-ratio: root.threshold-ratio; } }
            VerticalLayout { alignment: end; Bar { val: root.val4; max-val: root.max-val; chart-height: root.chart-height; show-threshold: root.show-threshold; threshold-ratio: root.threshold-ratio; } }
            VerticalLayout { alignment: end; Bar { val: root.val3; max-val: root.max-val; chart-height: root.chart-height; show-threshold: root.show-threshold; threshold-ratio: root.threshold-ratio; } }
            VerticalLayout { alignment: end; Bar { val: root.val2; max-val: root.max-val; chart-height: root.chart-height; show-threshold: root.show-threshold; threshold-ratio: root.threshold-ratio; } }
            VerticalLayout { alignment: end; Bar { val: root.val1; max-val: root.max-val; chart-height: root.chart-height; show-threshold: root.show-threshold; threshold-ratio: root.threshold-ratio; } }
            VerticalLayout { alignment: end; Bar { val: root.val0; max-val: root.max-val; chart-height: root.chart-height; show-threshold: root.show-threshold; threshold-ratio: root.threshold-ratio; } }
            VerticalLayout { alignment: end; Bar { val: root.val15; max-val: root.max-val; chart-height: root.chart-height; show-threshold: root.show-threshold; threshold-ratio: root.threshold-ratio; } }
            VerticalLayout { alignment: end; Bar { val: root.val16; max-val: root.max-val; chart-height: root.chart-height; show-threshold: root.show-threshold; threshold-ratio: root.threshold-ratio; } }
            VerticalLayout { alignment: end; Bar { val: root.val17; max-val: root.max-val; chart-height: root.chart-height; show-threshold: root.show-threshold; threshold-ratio: root.threshold-ratio; } }
            VerticalLayout { alignment: end; Bar { val: root.val18; max-val: root.max-val; chart-height: root.chart-height; show-threshold: root.show-threshold; threshold-ratio: root.threshold-ratio; } }
            VerticalLayout { alignment: end; Bar { val: root.val19; max-val: root.max-val; chart-height: root.chart-height; show-threshold: root.show-threshold; threshold-ratio: root.threshold-ratio; } }
            VerticalLayout { alignment: end; Bar { val: root.val20; max-val: root.max-val; chart-height: root.chart-height; show-threshold: root.show-threshold; threshold-ratio: root.threshold-ratio; } }
            VerticalLayout { alignment: end; Bar { val: root.val21; max-val: root.max-val; chart-height: root.chart-height; show-threshold: root.show-threshold; threshold-ratio: root.threshold-ratio; } }
            VerticalLayout { alignment: end; Bar { val: root.val22; max-val: root.max-val; chart-height: root.chart-height; show-threshold: root.show-threshold; threshold-ratio: root.threshold-ratio; } }
            VerticalLayout { alignment: end; Bar { val: root.val23; max-val: root.max-val; chart-height: root.chart-height; show-threshold: root.show-threshold; threshold-ratio: root.threshold-ratio; } }
            VerticalLayout { alignment: end; Bar { val: root.val24; max-val: root.max-val; chart-height: root.chart-height; show-threshold: root.show-threshold; threshold-ratio: root.threshold-ratio; } }
            VerticalLayout { alignment: end; Bar { val: root.val25; max-val: root.max-val; chart-height: root.chart-height; show-threshold: root.show-threshold; threshold-ratio: root.threshold-ratio; } }
            VerticalLayout { alignment: end; Bar { val: root.val26; max-val: root.max-val; chart-height: root.chart-height; show-threshold: root.show-threshold; threshold-ratio: root.threshold-ratio; } }
            VerticalLayout { alignment: end; Bar { val: root.val27; max-val: root.max-val; chart-height: root.chart-height; show-threshold: root.show-threshold; threshold-ratio: root.threshold-ratio; } }
            VerticalLayout { alignment: end; Bar { val: root.val28; max-val: root.max-val; chart-height: root.chart-height; show-threshold: root.show-threshold; threshold-ratio: root.threshold-ratio; } }
            VerticalLayout { alignment: end; Bar { val: root.val29; max-val: root.max-val; chart-height: root.chart-height; show-threshold: root.show-threshold; threshold-ratio: root.threshold-ratio; } }
        }

        // Guide lines
        Rectangle {
            x: 2px;
            y: root.height - 1px - root.chart-height * 0.5;
            width: root.width - 4px;
            height: 1px;
            background: rgb(38, 38, 50);
        }

        // Threshold line
        if (root.show-threshold) : Rectangle {
            x: 0px;
            y: root.height - 1px - root.threshold-ratio * root.chart-height;
            width: root.width;
            height: 1px;
            background: rgb(251, 146, 60);
        }
    }

    component AutocompleteInput inherits HorizontalLayout {
        in property <string> placeholder-text: "";
        in property <[string]> suggestions: [];
        in-out property <string> text <=> input.text;
        callback accepted(string);

        spacing: 4px;
        
        input := LineEdit {
            placeholder-text: root.placeholder-text;
            accepted => {
                if (self.text != "") {
                    root.accepted(self.text);
                    self.text = "";
                }
            }
        }

        if (root.suggestions.length > 0) : Rectangle {
            width: 30px;
            
            btn := Button {
                text: "▼";
                width: 30px;
                clicked => {
                    popup.show();
                }
            }
            
            popup := PopupWindow {
                x: -input.width - 4px;
                y: btn.height;
                width: input.width + 34px;
                height: clamp(root.suggestions.length * 28px + 8px, 28px, 150px);
                close-policy: close-on-click;
                
                Rectangle {
                    background: rgb(43, 43, 54);
                    border-color: rgb(62, 62, 74);
                    border-width: 1px;
                    border-radius: 4px;
                    
                    ScrollView {
                        VerticalLayout {
                            padding: 4px;
                            spacing: 2px;
                            
                            for item in root.suggestions : ta := TouchArea {
                                height: 26px;
                                clicked => {
                                    input.text = item;
                                    popup.close();
                                }
                                
                                Rectangle {
                                    background: ta.has-hover ? rgb(62, 62, 74) : transparent;
                                    border-radius: 2px;
                                    
                                    Text {
                                        text: item;
                                        color: rgb(224, 224, 224);
                                        font-size: 13px;
                                        x: 8px;
                                        vertical-alignment: center;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    component ChipInput inherits VerticalLayout {
        in property <string> label: "";
        in property <[string]> suggestions: [];
        in-out property <string> items: "";
        in-out property <int> tag-count: 0;
        in-out property <string> tag-0: "";
        in-out property <string> tag-1: "";
        in-out property <string> tag-2: "";
        in-out property <string> tag-3: "";
        in-out property <string> tag-4: "";
        in-out property <string> tag-5: "";
        in-out property <string> tag-6: "";
        in-out property <string> tag-7: "";
        callback add-item(string);
        callback remove-item(int);

        spacing: 6px;
        alignment: start;

        if (root.label != "") : Text {
            text: root.label;
            font-size: 12px;
            color: rgb(180, 180, 190);
            font-weight: 500;
        }

        if (root.tag-count > 0) : HorizontalLayout {
            spacing: 4px;
            alignment: start;

            if (root.tag-count > 0 && root.tag-0 != "") : Rectangle {
                background: rgb(55, 65, 81);
                border-radius: 4px;
                height: 22px;
                HorizontalLayout {
                    padding-left: 8px;
                    padding-right: 4px;
                    spacing: 2px;
                    Text { text: root.tag-0; color: rgb(226, 232, 240); font-size: 12px; vertical-alignment: center; }
                    t0 := TouchArea { clicked => { root.remove-item(0); }
                        Text { text: "×"; color: rgb(160, 170, 190); font-size: 12px; vertical-alignment: center; }
                    }
                }
            }
            if (root.tag-count > 1 && root.tag-1 != "") : Rectangle {
                background: rgb(55, 65, 81);
                border-radius: 4px;
                height: 22px;
                HorizontalLayout {
                    padding-left: 8px;
                    padding-right: 4px;
                    spacing: 2px;
                    Text { text: root.tag-1; color: rgb(226, 232, 240); font-size: 12px; vertical-alignment: center; }
                    t1 := TouchArea { clicked => { root.remove-item(1); }
                        Text { text: "×"; color: rgb(160, 170, 190); font-size: 12px; vertical-alignment: center; }
                    }
                }
            }
            if (root.tag-count > 2 && root.tag-2 != "") : Rectangle {
                background: rgb(55, 65, 81);
                border-radius: 4px;
                height: 22px;
                HorizontalLayout {
                    padding-left: 8px;
                    padding-right: 4px;
                    spacing: 2px;
                    Text { text: root.tag-2; color: rgb(226, 232, 240); font-size: 12px; vertical-alignment: center; }
                    t2 := TouchArea { clicked => { root.remove-item(2); }
                        Text { text: "×"; color: rgb(160, 170, 190); font-size: 12px; vertical-alignment: center; }
                    }
                }
            }
            if (root.tag-count > 3 && root.tag-3 != "") : Rectangle {
                background: rgb(55, 65, 81);
                border-radius: 4px;
                height: 22px;
                HorizontalLayout {
                    padding-left: 8px;
                    padding-right: 4px;
                    spacing: 2px;
                    Text { text: root.tag-3; color: rgb(226, 232, 240); font-size: 12px; vertical-alignment: center; }
                    t3 := TouchArea { clicked => { root.remove-item(3); }
                        Text { text: "×"; color: rgb(160, 170, 190); font-size: 12px; vertical-alignment: center; }
                    }
                }
            }
            if (root.tag-count > 4 && root.tag-4 != "") : Rectangle {
                background: rgb(55, 65, 81);
                border-radius: 4px;
                height: 22px;
                HorizontalLayout {
                    padding-left: 8px;
                    padding-right: 4px;
                    spacing: 2px;
                    Text { text: root.tag-4; color: rgb(226, 232, 240); font-size: 12px; vertical-alignment: center; }
                    t4 := TouchArea { clicked => { root.remove-item(4); }
                        Text { text: "×"; color: rgb(160, 170, 190); font-size: 12px; vertical-alignment: center; }
                    }
                }
            }
            if (root.tag-count > 5 && root.tag-5 != "") : Rectangle {
                background: rgb(55, 65, 81);
                border-radius: 4px;
                height: 22px;
                HorizontalLayout {
                    padding-left: 8px;
                    padding-right: 4px;
                    spacing: 2px;
                    Text { text: root.tag-5; color: rgb(226, 232, 240); font-size: 12px; vertical-alignment: center; }
                    t5 := TouchArea { clicked => { root.remove-item(5); }
                        Text { text: "×"; color: rgb(160, 170, 190); font-size: 12px; vertical-alignment: center; }
                    }
                }
            }
            if (root.tag-count > 6 && root.tag-6 != "") : Rectangle {
                background: rgb(55, 65, 81);
                border-radius: 4px;
                height: 22px;
                HorizontalLayout {
                    padding-left: 8px;
                    padding-right: 4px;
                    spacing: 2px;
                    Text { text: root.tag-6; color: rgb(226, 232, 240); font-size: 12px; vertical-alignment: center; }
                    t6 := TouchArea { clicked => { root.remove-item(6); }
                        Text { text: "×"; color: rgb(160, 170, 190); font-size: 12px; vertical-alignment: center; }
                    }
                }
            }
            if (root.tag-count > 7 && root.tag-7 != "") : Rectangle {
                background: rgb(55, 65, 81);
                border-radius: 4px;
                height: 22px;
                HorizontalLayout {
                    padding-left: 8px;
                    padding-right: 4px;
                    spacing: 2px;
                    Text { text: root.tag-7; color: rgb(226, 232, 240); font-size: 12px; vertical-alignment: center; }
                    t7 := TouchArea { clicked => { root.remove-item(7); }
                        Text { text: "×"; color: rgb(160, 170, 190); font-size: 12px; vertical-alignment: center; }
                    }
                }
            }
        }

        HorizontalLayout {
            spacing: 6px;

            input := AutocompleteInput {
                placeholder-text: "直接入力、またはリストから選択...";
                suggestions: root.suggestions;
                accepted(text) => {
                    root.add-item(text);
                }
            }

            Button {
                text: "追加";
                clicked => {
                    if (input.text != "") {
                        root.add-item(input.text);
                        input.text = "";
                    }
                }
            }
        }
    }

    component SensorConfigPanel inherits GroupBox {
        in-out property <bool> sensor-enabled: true;
        in-out property <string> threshold-text: "";
        in-out property <float> threshold-val: 1.0;
        in-out property <float> threshold-min: 1.0;
        in-out property <float> threshold-max: 100.0;
        in-out property <string> current-val-text: "";

        // Chart properties
        in property <float> max-chart-val: 100.0;
        in property <float> val0: 0.0;
        in property <float> val1: 0.0;
        in property <float> val2: 0.0;
        in property <float> val3: 0.0;
        in property <float> val4: 0.0;
        in property <float> val5: 0.0;
        in property <float> val6: 0.0;
        in property <float> val7: 0.0;
        in property <float> val8: 0.0;
        in property <float> val9: 0.0;
        in property <float> val10: 0.0;
        in property <float> val11: 0.0;
        in property <float> val12: 0.0;
        in property <float> val13: 0.0;
        in property <float> val14: 0.0;
        in property <float> val15: 0.0;
        in property <float> val16: 0.0;
        in property <float> val17: 0.0;
        in property <float> val18: 0.0;
        in property <float> val19: 0.0;
        in property <float> val20: 0.0;
        in property <float> val21: 0.0;
        in property <float> val22: 0.0;
        in property <float> val23: 0.0;
        in property <float> val24: 0.0;
        in property <float> val25: 0.0;
        in property <float> val26: 0.0;
        in property <float> val27: 0.0;
        in property <float> val28: 0.0;
        in property <float> val29: 0.0;

        in property <length> chart-width: 90px;
        in property <length> chart-height: 20px;
        in property <length> chart-bar-height: 18px;
        in property <bool> show-threshold: false;
        in property <float> threshold-ratio: 0.0;

        callback toggle-enabled();
        callback threshold-changed(float);

        VerticalLayout {
            spacing: 8px;
            CheckBox {
                text: "このセンサーを有効にする";
                checked: root.sensor-enabled;
                toggled => { root.toggle-enabled(); }
            }

            if (root.sensor-enabled) : VerticalLayout {
                spacing: 8px;
                HorizontalLayout {
                    spacing: 8px;
                    Text {
                        text: root.current-val-text;
                        color: rgb(129, 140, 248); // Indigo 400 for better visibility in dark theme
                        font-weight: 600;
                        vertical-alignment: center;
                    }
                    Rectangle {} // Spacer
                    Sparkline {
                        max-val: root.max-chart-val;
                        val0: root.val0;
                        val1: root.val1;
                        val2: root.val2;
                        val3: root.val3;
                        val4: root.val4;
                        val5: root.val5;
                        val6: root.val6;
                        val7: root.val7;
                        val8: root.val8;
                        val9: root.val9;
                        val10: root.val10;
                        val11: root.val11;
                        val12: root.val12;
                        val13: root.val13;
                        val14: root.val14;
                        val15: root.val15;
                        val16: root.val16;
                        val17: root.val17;
                        val18: root.val18;
                        val19: root.val19;
                        val20: root.val20;
                        val21: root.val21;
                        val22: root.val22;
                        val23: root.val23;
                        val24: root.val24;
                        val25: root.val25;
                        val26: root.val26;
                        val27: root.val27;
                        val28: root.val28;
                        val29: root.val29;
                        width: root.chart-width;
                        height: root.chart-height;
                        chart-height: root.chart-bar-height;
                        show-threshold: root.show-threshold;
                        threshold-ratio: root.threshold-ratio;
                    }
                }
                HorizontalLayout {
                    Text {
                        text: root.threshold-text;
                        color: rgb(224, 224, 224);
                    }
                }
                Slider {
                    minimum: root.threshold-min;
                    maximum: root.threshold-max;
                    value: root.threshold-val;
                    changed(val) => { root.threshold-changed(val); }
                }
            }
        }
    }

    component MultimediaConfigPanel inherits GroupBox {
        in-out property <bool> sensor-enabled: true;
        in-out property <string> current-val-text: "";
        in-out property <bool> current-sound-active: false;

        // Chart properties
        in property <float> val0: 0.0;
        in property <float> val1: 0.0;
        in property <float> val2: 0.0;
        in property <float> val3: 0.0;
        in property <float> val4: 0.0;
        in property <float> val5: 0.0;
        in property <float> val6: 0.0;
        in property <float> val7: 0.0;
        in property <float> val8: 0.0;
        in property <float> val9: 0.0;
        in property <float> val10: 0.0;
        in property <float> val11: 0.0;
        in property <float> val12: 0.0;
        in property <float> val13: 0.0;
        in property <float> val14: 0.0;
        in property <float> val15: 0.0;
        in property <float> val16: 0.0;
        in property <float> val17: 0.0;
        in property <float> val18: 0.0;
        in property <float> val19: 0.0;
        in property <float> val20: 0.0;
        in property <float> val21: 0.0;
        in property <float> val22: 0.0;
        in property <float> val23: 0.0;
        in property <float> val24: 0.0;
        in property <float> val25: 0.0;
        in property <float> val26: 0.0;
        in property <float> val27: 0.0;
        in property <float> val28: 0.0;
        in property <float> val29: 0.0;

        in property <length> chart-width: 90px;
        in property <length> chart-height: 20px;
        in property <length> chart-bar-height: 18px;
        in property <bool> show-threshold: false;
        in property <float> threshold-ratio: 0.0;

        callback toggle-enabled();

        VerticalLayout {
            spacing: 8px;
            CheckBox {
                text: "音声再生中はスリープを抑止する";
                checked: root.sensor-enabled;
                toggled => { root.toggle-enabled(); }
            }

            if (root.sensor-enabled) : VerticalLayout {
                spacing: 8px;
                HorizontalLayout {
                    spacing: 8px;
                    Text {
                        text: root.current-val-text;
                        color: root.current-sound-active ? rgb(129, 140, 248) : rgb(136, 136, 136);
                        font-weight: 600;
                        vertical-alignment: center;
                    }
                    Rectangle {} // Spacer
                    Sparkline {
                        max-val: 0.05;
                        val0: root.val0;
                        val1: root.val1;
                        val2: root.val2;
                        val3: root.val3;
                        val4: root.val4;
                        val5: root.val5;
                        val6: root.val6;
                        val7: root.val7;
                        val8: root.val8;
                        val9: root.val9;
                        val10: root.val10;
                        val11: root.val11;
                        val12: root.val12;
                        val13: root.val13;
                        val14: root.val14;
                        val15: root.val15;
                        val16: root.val16;
                        val17: root.val17;
                        val18: root.val18;
                        val19: root.val19;
                        val20: root.val20;
                        val21: root.val21;
                        val22: root.val22;
                        val23: root.val23;
                        val24: root.val24;
                        val25: root.val25;
                        val26: root.val26;
                        val27: root.val27;
                        val28: root.val28;
                        val29: root.val29;
                        width: root.chart-width;
                        height: root.chart-height;
                        chart-height: root.chart-bar-height;
                        show-threshold: root.show-threshold;
                        threshold-ratio: root.threshold-ratio;
                    }
                }
                Rectangle {
                    height: 58px;
                    Text {
                        text: "音声出力（サウンドカードの出力振幅レベル）を検知して、メディア再生中の自動スリープ移行を抑止します。";
                        font-size: 11px;
                        color: rgb(160, 160, 160);
                        wrap: word-wrap;
                    }
                }
            }
        }
    }

    export component SettingsWindow inherits Window {
        title: "SleepTool 設定";
        icon: @image-url("../assets/icons/default.png");
        width: 720px;
        height: 550px;
        background: rgb(30, 30, 36);
        default-font-size: 14px;

        // State properties
        in-out property <int> active-tab: 0;

        // Config properties
        in-out property <float> sleep-delay-minutes: 10.0;
        in-out property <bool> hibernate: false;
        in-out property <bool> sound-enabled: true;
        in-out property <bool> auto-start: false;
        in-out property <bool> display-off-on-sleep: true;
        in-out property <bool> warn-before-sleep: true;
        in-out property <bool> warn-sound-enabled: true;
        in-out property <bool> display-state-by-icon: true;

        // Sensors
        in-out property <bool> cpu-enabled: true;
        in-out property <float> cpu-threshold: 1.0;

        in-out property <bool> network-enabled: true;
        in-out property <float> network-threshold-kb: 10.0;

        in-out property <bool> disk-write-enabled: true;
        in-out property <float> disk-write-threshold-kb: 10.0;

        // Lists
        in-out property <string> excluded-processes: "";
        in-out property <string> watched-processes: "";
        in-out property <string> watched-tags: "";
        in-out property <string> excluded-tags: "";
        in property <[string]> process-suggestions: [];

        // Tag slot properties for watched
        in-out property <int> watched-tag-count: 0;
        in-out property <string> watched-tag-0: "";
        in-out property <string> watched-tag-1: "";
        in-out property <string> watched-tag-2: "";
        in-out property <string> watched-tag-3: "";
        in-out property <string> watched-tag-4: "";
        in-out property <string> watched-tag-5: "";
        in-out property <string> watched-tag-6: "";
        in-out property <string> watched-tag-7: "";

        // Tag slot properties for excluded
        in-out property <int> excluded-tag-count: 0;
        in-out property <string> excluded-tag-0: "";
        in-out property <string> excluded-tag-1: "";
        in-out property <string> excluded-tag-2: "";
        in-out property <string> excluded-tag-3: "";
        in-out property <string> excluded-tag-4: "";
        in-out property <string> excluded-tag-5: "";
        in-out property <string> excluded-tag-6: "";
        in-out property <string> excluded-tag-7: "";



        // Real-time monitored values
        in-out property <float> current-cpu: 0.0;
        in-out property <float> current-network-kb: 0.0;
        in-out property <float> current-disk-write-kb: 0.0;
        in-out property <bool> current-sound-active: false;

        // CPU History
        in-out property <float> cpu-history-0: 0.0;
        in-out property <float> cpu-history-1: 0.0;
        in-out property <float> cpu-history-2: 0.0;
        in-out property <float> cpu-history-3: 0.0;
        in-out property <float> cpu-history-4: 0.0;
        in-out property <float> cpu-history-5: 0.0;
        in-out property <float> cpu-history-6: 0.0;
        in-out property <float> cpu-history-7: 0.0;
        in-out property <float> cpu-history-8: 0.0;
        in-out property <float> cpu-history-9: 0.0;
        in-out property <float> cpu-history-10: 0.0;
        in-out property <float> cpu-history-11: 0.0;
        in-out property <float> cpu-history-12: 0.0;
        in-out property <float> cpu-history-13: 0.0;
        in-out property <float> cpu-history-14: 0.0;
        in-out property <float> cpu-history-15: 0.0;
        in-out property <float> cpu-history-16: 0.0;
        in-out property <float> cpu-history-17: 0.0;
        in-out property <float> cpu-history-18: 0.0;
        in-out property <float> cpu-history-19: 0.0;
        in-out property <float> cpu-history-20: 0.0;
        in-out property <float> cpu-history-21: 0.0;
        in-out property <float> cpu-history-22: 0.0;
        in-out property <float> cpu-history-23: 0.0;
        in-out property <float> cpu-history-24: 0.0;
        in-out property <float> cpu-history-25: 0.0;
        in-out property <float> cpu-history-26: 0.0;
        in-out property <float> cpu-history-27: 0.0;
        in-out property <float> cpu-history-28: 0.0;
        in-out property <float> cpu-history-29: 0.0;

        // Network History
        in-out property <float> network-history-0: 0.0;
        in-out property <float> network-history-1: 0.0;
        in-out property <float> network-history-2: 0.0;
        in-out property <float> network-history-3: 0.0;
        in-out property <float> network-history-4: 0.0;
        in-out property <float> network-history-5: 0.0;
        in-out property <float> network-history-6: 0.0;
        in-out property <float> network-history-7: 0.0;
        in-out property <float> network-history-8: 0.0;
        in-out property <float> network-history-9: 0.0;
        in-out property <float> network-history-10: 0.0;
        in-out property <float> network-history-11: 0.0;
        in-out property <float> network-history-12: 0.0;
        in-out property <float> network-history-13: 0.0;
        in-out property <float> network-history-14: 0.0;
        in-out property <float> network-history-15: 0.0;
        in-out property <float> network-history-16: 0.0;
        in-out property <float> network-history-17: 0.0;
        in-out property <float> network-history-18: 0.0;
        in-out property <float> network-history-19: 0.0;
        in-out property <float> network-history-20: 0.0;
        in-out property <float> network-history-21: 0.0;
        in-out property <float> network-history-22: 0.0;
        in-out property <float> network-history-23: 0.0;
        in-out property <float> network-history-24: 0.0;
        in-out property <float> network-history-25: 0.0;
        in-out property <float> network-history-26: 0.0;
        in-out property <float> network-history-27: 0.0;
        in-out property <float> network-history-28: 0.0;
        in-out property <float> network-history-29: 0.0;

        // Disk Write History
        in-out property <float> disk-write-history-0: 0.0;
        in-out property <float> disk-write-history-1: 0.0;
        in-out property <float> disk-write-history-2: 0.0;
        in-out property <float> disk-write-history-3: 0.0;
        in-out property <float> disk-write-history-4: 0.0;
        in-out property <float> disk-write-history-5: 0.0;
        in-out property <float> disk-write-history-6: 0.0;
        in-out property <float> disk-write-history-7: 0.0;
        in-out property <float> disk-write-history-8: 0.0;
        in-out property <float> disk-write-history-9: 0.0;
        in-out property <float> disk-write-history-10: 0.0;
        in-out property <float> disk-write-history-11: 0.0;
        in-out property <float> disk-write-history-12: 0.0;
        in-out property <float> disk-write-history-13: 0.0;
        in-out property <float> disk-write-history-14: 0.0;
        in-out property <float> disk-write-history-15: 0.0;
        in-out property <float> disk-write-history-16: 0.0;
        in-out property <float> disk-write-history-17: 0.0;
        in-out property <float> disk-write-history-18: 0.0;
        in-out property <float> disk-write-history-19: 0.0;
        in-out property <float> disk-write-history-20: 0.0;
        in-out property <float> disk-write-history-21: 0.0;
        in-out property <float> disk-write-history-22: 0.0;
        in-out property <float> disk-write-history-23: 0.0;
        in-out property <float> disk-write-history-24: 0.0;
        in-out property <float> disk-write-history-25: 0.0;
        in-out property <float> disk-write-history-26: 0.0;
        in-out property <float> disk-write-history-27: 0.0;
        in-out property <float> disk-write-history-28: 0.0;
        in-out property <float> disk-write-history-29: 0.0;

        // Sound History
        in-out property <float> sound-history-0: 0.0;
        in-out property <float> sound-history-1: 0.0;
        in-out property <float> sound-history-2: 0.0;
        in-out property <float> sound-history-3: 0.0;
        in-out property <float> sound-history-4: 0.0;
        in-out property <float> sound-history-5: 0.0;
        in-out property <float> sound-history-6: 0.0;
        in-out property <float> sound-history-7: 0.0;
        in-out property <float> sound-history-8: 0.0;
        in-out property <float> sound-history-9: 0.0;
        in-out property <float> sound-history-10: 0.0;
        in-out property <float> sound-history-11: 0.0;
        in-out property <float> sound-history-12: 0.0;
        in-out property <float> sound-history-13: 0.0;
        in-out property <float> sound-history-14: 0.0;
        in-out property <float> sound-history-15: 0.0;
        in-out property <float> sound-history-16: 0.0;
        in-out property <float> sound-history-17: 0.0;
        in-out property <float> sound-history-18: 0.0;
        in-out property <float> sound-history-19: 0.0;
        in-out property <float> sound-history-20: 0.0;
        in-out property <float> sound-history-21: 0.0;
        in-out property <float> sound-history-22: 0.0;
        in-out property <float> sound-history-23: 0.0;
        in-out property <float> sound-history-24: 0.0;
        in-out property <float> sound-history-25: 0.0;
        in-out property <float> sound-history-26: 0.0;
        in-out property <float> sound-history-27: 0.0;
        in-out property <float> sound-history-28: 0.0;
        in-out property <float> sound-history-29: 0.0;

        callback save-clicked();
        callback cancel-clicked();
        callback watched-add(string);
        callback watched-pop();
        callback excluded-add(string);
        callback excluded-pop();
        callback watched-remove(int);
        callback excluded-remove(int);

        HorizontalLayout {
            // Sidebar
            Rectangle {
                width: 180px;
                background: rgb(37, 37, 46);

                VerticalLayout {
                    padding: 16px;
                    spacing: 12px;
                    alignment: start;

                    Text {
                        text: "SleepTool 設定";
                        font-size: 18px;
                        font-weight: 700;
                        color: rgb(255, 255, 255);
                        horizontal-alignment: center;
                        height: 40px;
                    }

                    SidebarButton {
                        text: "基本設定";
                        icon: @image-url("../assets/icons/default.png");
                        active: root.active-tab == 0;
                        clicked => { root.active-tab = 0; }
                    }
                    SidebarButton {
                        text: "CPU使用率";
                        icon: @image-url("../assets/icons/cpu.png");
                        active: root.active-tab == 1;
                        clicked => { root.active-tab = 1; }
                    }
                    SidebarButton {
                        text: "ネットワーク";
                        icon: @image-url("../assets/icons/network.png");
                        active: root.active-tab == 2;
                        clicked => { root.active-tab = 2; }
                    }
                    SidebarButton {
                        text: "ディスク書き込み";
                        icon: @image-url("../assets/icons/disk.png");
                        active: root.active-tab == 3;
                        clicked => { root.active-tab = 3; }
                    }
                    SidebarButton {
                        text: "マルチメディア";
                        icon: @image-url("../assets/icons/sound.png");
                        active: root.active-tab == 4;
                        clicked => { root.active-tab = 4; }
                    }
                    SidebarButton {
                        text: "プロセス";
                        icon: @image-url("../assets/icons/process.png");
                        active: root.active-tab == 5;
                        clicked => { root.active-tab = 5; }
                    }
                }
            }

            // Main content area
            VerticalLayout {
                padding: 20px;
                spacing: 16px;

                Text {
                    text: root.active-tab == 0 ? "基本設定" :
                          root.active-tab == 1 ? "CPU使用率監視" :
                          root.active-tab == 2 ? "ネットワーク通信量監視" :
                          root.active-tab == 3 ? "ディスク書き込み量監視" :
                          root.active-tab == 4 ? "マルチメディア監視" : "プロセス監視";
                    font-size: 20px;
                    font-weight: 700;
                    color: rgb(255, 255, 255);
                }

                Rectangle {
                    background: rgb(43, 43, 54);
                    border-radius: 8px;
                    clip: true;

                    if (root.active-tab == 0) : VerticalLayout {
                        padding: 16px;
                        spacing: 10px;
                        alignment: start;

                        GroupBox {
                            title: "スリープ移行時間";
                            VerticalLayout {
                                spacing: 8px;
                                HorizontalLayout {
                                    Text {
                                        text: "無操作時間: " + round(root.sleep-delay-minutes) + " 分";
                                        color: rgb(224, 224, 224);
                                        vertical-alignment: center;
                                    }
                                }
                                Slider {
                                    minimum: 1;
                                    maximum: 180;
                                    value <=> root.sleep-delay-minutes;
                                }
                            }
                        }

                        GroupBox {
                            title: "システム動作設定";
                            VerticalLayout {
                                spacing: 8px;
                                CheckBox {
                                    text: "スリープの代わりに休止状態（ハイバネート）を使用する";
                                    checked <=> root.hibernate;
                                }
                                CheckBox {
                                    text: "スリープ復帰時に自動的に画面をオフにする";
                                    checked <=> root.display-off-on-sleep;
                                }
                                CheckBox {
                                    text: "ログイン時に自動起動する";
                                    checked <=> root.auto-start;
                                }
                            }
                        }

                        GroupBox {
                            title: "警告・通知設定";
                            VerticalLayout {
                                spacing: 8px;
                                CheckBox {
                                    text: "スリープ前にバルーン通知で警告する";
                                    checked <=> root.warn-before-sleep;
                                }
                                CheckBox {
                                    text: "スリープ前に警告サウンドを再生する";
                                    checked <=> root.warn-sound-enabled;
                                }
                                CheckBox {
                                    text: "トレイアイコンの状態表示を有効にする";
                                    checked <=> root.display-state-by-icon;
                                }
                            }
                        }
                    }

                    if (root.active-tab == 1) : VerticalLayout {
                        padding: 16px;
                        alignment: start;
                        SensorConfigPanel {
                            title: "CPU使用率";
                            chart-width: 220px;
                            chart-height: 48px;
                            chart-bar-height: 44px;
                            show-threshold: true;
                            threshold-ratio: root.cpu-threshold / 50.0;
                            sensor-enabled <=> root.cpu-enabled;
                            threshold-text: "閾値: " + round(root.cpu-threshold) + " %";
                            threshold-val <=> root.cpu-threshold;
                            threshold-min: 1.0;
                            threshold-max: 50.0;
                            current-val-text: "現在: " + round(root.current-cpu) + " %";
                            max-chart-val: 50.0;
                            val0: root.cpu-history-0;
                            val1: root.cpu-history-1;
                            val2: root.cpu-history-2;
                            val3: root.cpu-history-3;
                            val4: root.cpu-history-4;
                            val5: root.cpu-history-5;
                            val6: root.cpu-history-6;
                            val7: root.cpu-history-7;
                            val8: root.cpu-history-8;
                            val9: root.cpu-history-9;
                            val10: root.cpu-history-10;
                            val11: root.cpu-history-11;
                            val12: root.cpu-history-12;
                            val13: root.cpu-history-13;
                            val14: root.cpu-history-14;
                            val15: root.cpu-history-15;
                            val16: root.cpu-history-16;
                            val17: root.cpu-history-17;
                            val18: root.cpu-history-18;
                            val19: root.cpu-history-19;
                            val20: root.cpu-history-20;
                            val21: root.cpu-history-21;
                            val22: root.cpu-history-22;
                            val23: root.cpu-history-23;
                            val24: root.cpu-history-24;
                            val25: root.cpu-history-25;
                            val26: root.cpu-history-26;
                            val27: root.cpu-history-27;
                            val28: root.cpu-history-28;
                            val29: root.cpu-history-29;
                            toggle-enabled => { root.cpu-enabled = !root.cpu-enabled; }
                            threshold-changed(val) => { root.cpu-threshold = val; }
                        }
                    }

                    if (root.active-tab == 2) : VerticalLayout {
                        padding: 16px;
                        alignment: start;
                        SensorConfigPanel {
                            title: "ネットワーク通信量";
                            chart-width: 220px;
                            chart-height: 48px;
                            chart-bar-height: 44px;
                            show-threshold: true;
                            threshold-ratio: root.network-threshold-kb / 1000.0;
                            sensor-enabled <=> root.network-enabled;
                            threshold-text: "閾値: " + round(root.network-threshold-kb) + " KB/s";
                            threshold-val <=> root.network-threshold-kb;
                            threshold-min: 1.0;
                            threshold-max: 1000.0;
                            current-val-text: "現在: " + round(root.current-network-kb) + " KB/s";
                            max-chart-val: 1000.0;
                            val0: root.network-history-0;
                            val1: root.network-history-1;
                            val2: root.network-history-2;
                            val3: root.network-history-3;
                            val4: root.network-history-4;
                            val5: root.network-history-5;
                            val6: root.network-history-6;
                            val7: root.network-history-7;
                            val8: root.network-history-8;
                            val9: root.network-history-9;
                            val10: root.network-history-10;
                            val11: root.network-history-11;
                            val12: root.network-history-12;
                            val13: root.network-history-13;
                            val14: root.network-history-14;
                            val15: root.network-history-15;
                            val16: root.network-history-16;
                            val17: root.network-history-17;
                            val18: root.network-history-18;
                            val19: root.network-history-19;
                            val20: root.network-history-20;
                            val21: root.network-history-21;
                            val22: root.network-history-22;
                            val23: root.network-history-23;
                            val24: root.network-history-24;
                            val25: root.network-history-25;
                            val26: root.network-history-26;
                            val27: root.network-history-27;
                            val28: root.network-history-28;
                            val29: root.network-history-29;
                            toggle-enabled => { root.network-enabled = !root.network-enabled; }
                            threshold-changed(val) => { root.network-threshold-kb = val; }
                        }
                    }

                    if (root.active-tab == 3) : VerticalLayout {
                        padding: 16px;
                        alignment: start;
                        SensorConfigPanel {
                            title: "ディスク書き込み量";
                            chart-width: 220px;
                            chart-height: 48px;
                            chart-bar-height: 44px;
                            show-threshold: true;
                            threshold-ratio: root.disk-write-threshold-kb / 1000.0;
                            sensor-enabled <=> root.disk-write-enabled;
                            threshold-text: "閾値: " + round(root.disk-write-threshold-kb) + " KB/s";
                            threshold-val <=> root.disk-write-threshold-kb;
                            threshold-min: 1.0;
                            threshold-max: 1000.0;
                            current-val-text: "現在: " + round(root.current-disk-write-kb) + " KB/s";
                            max-chart-val: 1000.0;
                            val0: root.disk-write-history-0;
                            val1: root.disk-write-history-1;
                            val2: root.disk-write-history-2;
                            val3: root.disk-write-history-3;
                            val4: root.disk-write-history-4;
                            val5: root.disk-write-history-5;
                            val6: root.disk-write-history-6;
                            val7: root.disk-write-history-7;
                            val8: root.disk-write-history-8;
                            val9: root.disk-write-history-9;
                            val10: root.disk-write-history-10;
                            val11: root.disk-write-history-11;
                            val12: root.disk-write-history-12;
                            val13: root.disk-write-history-13;
                            val14: root.disk-write-history-14;
                            val15: root.disk-write-history-15;
                            val16: root.disk-write-history-16;
                            val17: root.disk-write-history-17;
                            val18: root.disk-write-history-18;
                            val19: root.disk-write-history-19;
                            val20: root.disk-write-history-20;
                            val21: root.disk-write-history-21;
                            val22: root.disk-write-history-22;
                            val23: root.disk-write-history-23;
                            val24: root.disk-write-history-24;
                            val25: root.disk-write-history-25;
                            val26: root.disk-write-history-26;
                            val27: root.disk-write-history-27;
                            val28: root.disk-write-history-28;
                            val29: root.disk-write-history-29;
                            toggle-enabled => { root.disk-write-enabled = !root.disk-write-enabled; }
                            threshold-changed(val) => { root.disk-write-threshold-kb = val; }
                        }
                    }

                    if (root.active-tab == 4) : VerticalLayout {
                        padding: 16px;
                        alignment: start;
                        MultimediaConfigPanel {
                            title: "マルチメディア";
                            chart-width: 220px;
                            chart-height: 48px;
                            chart-bar-height: 44px;
                            sensor-enabled <=> root.sound-enabled;
                            current-val-text: root.current-sound-active ? "現在: 音声再生中 🔊" : "現在: 音声なし 🔇";
                            current-sound-active <=> root.current-sound-active;
                            val0: root.sound-history-0;
                            val1: root.sound-history-1;
                            val2: root.sound-history-2;
                            val3: root.sound-history-3;
                            val4: root.sound-history-4;
                            val5: root.sound-history-5;
                            val6: root.sound-history-6;
                            val7: root.sound-history-7;
                            val8: root.sound-history-8;
                            val9: root.sound-history-9;
                            val10: root.sound-history-10;
                            val11: root.sound-history-11;
                            val12: root.sound-history-12;
                            val13: root.sound-history-13;
                            val14: root.sound-history-14;
                            val15: root.sound-history-15;
                            val16: root.sound-history-16;
                            val17: root.sound-history-17;
                            val18: root.sound-history-18;
                            val19: root.sound-history-19;
                            val20: root.sound-history-20;
                            val21: root.sound-history-21;
                            val22: root.sound-history-22;
                            val23: root.sound-history-23;
                            val24: root.sound-history-24;
                            val25: root.sound-history-25;
                            val26: root.sound-history-26;
                            val27: root.sound-history-27;
                            val28: root.sound-history-28;
                            val29: root.sound-history-29;
                            toggle-enabled => { root.sound-enabled = !root.sound-enabled; }
                        }
                    }

                    if (root.active-tab == 5) : VerticalLayout {
                        padding: 16px;
                        spacing: 16px;
                        alignment: start;

                        GroupBox {
                            title: "監視するプロセス名（カンマ区切り）";
                            watched-chip := ChipInput {
                                items <=> root.watched-processes;
                                tag-count <=> root.watched-tag-count;
                                tag-0 <=> root.watched-tag-0;
                                tag-1 <=> root.watched-tag-1;
                                tag-2 <=> root.watched-tag-2;
                                tag-3 <=> root.watched-tag-3;
                                tag-4 <=> root.watched-tag-4;
                                tag-5 <=> root.watched-tag-5;
                                tag-6 <=> root.watched-tag-6;
                                tag-7 <=> root.watched-tag-7;
                                suggestions: root.process-suggestions;
                                add-item(text) => { root.watched-add(text); }
                                remove-item(idx) => { root.watched-remove(idx); }
                            }
                        }

                        GroupBox {
                            title: "除外するプロセス名（カンマ区切り）";
                            excluded-chip := ChipInput {
                                items <=> root.excluded-processes;
                                tag-count <=> root.excluded-tag-count;
                                tag-0 <=> root.excluded-tag-0;
                                tag-1 <=> root.excluded-tag-1;
                                tag-2 <=> root.excluded-tag-2;
                                tag-3 <=> root.excluded-tag-3;
                                tag-4 <=> root.excluded-tag-4;
                                tag-5 <=> root.excluded-tag-5;
                                tag-6 <=> root.excluded-tag-6;
                                tag-7 <=> root.excluded-tag-7;
                                suggestions: root.process-suggestions;
                                add-item(text) => { root.excluded-add(text); }
                                remove-item(idx) => { root.excluded-remove(idx); }
                            }
                        }
                    }
                }

                HorizontalLayout {
                    alignment: end;
                    spacing: 12px;
                    height: 36px;

                    Button {
                        text: "キャンセル";
                        width: 100px;
                        clicked => { root.cancel-clicked(); }
                    }

                    Button {
                        text: "保存";
                        primary: true;
                        width: 100px;
                        clicked => { root.save-clicked(); }
                    }
                }
            }
        }
    }
}

fn populate_settings_window(
    window: &SettingsWindow,
    state: &SharedState,
    platform: &Arc<WindowsPlatform>,
) {
    // Load config values into UI
    let config = {
        let s = state.lock().unwrap();
        s.config.as_ref().clone()
    };

    window.set_sleep_delay_minutes((config.sleep_delay_seconds / 60) as f32);
    window.set_hibernate(config.hibernate);
    window.set_sound_enabled(config.sound_enabled);
    window.set_auto_start(config.auto_start);
    window.set_display_off_on_sleep(config.display_off_on_sleep);
    window.set_warn_before_sleep(config.warn_before_sleep);
    window.set_warn_sound_enabled(config.warn_sound_enabled);
    window.set_display_state_by_icon(config.display_state_by_icon);

    window.set_cpu_enabled(config.cpu.enabled);
    window.set_cpu_threshold(config.cpu.threshold as f32);

    window.set_network_enabled(config.network.enabled);
    window.set_network_threshold_kb((config.network.threshold / 1000.0) as f32);

    window.set_disk_write_enabled(config.disk_write.enabled);
    window.set_disk_write_threshold_kb((config.disk_write.threshold / 1000.0) as f32);

    window.set_excluded_processes(slint::SharedString::from(config.excluded_processes.join(", ")));
    window.set_watched_processes(slint::SharedString::from(config.watched_processes.join(", ")));
    // Initialize tag slots
    sync_tags(window, &config.watched_processes, "watched");
    sync_tags(window, &config.excluded_processes, "excluded");

    // Load process list for autocomplete
    if let Ok(processes) = platform.list_running_processes() {
        let process_model: Vec<slint::SharedString> = processes
            .into_iter()
            .map(|p| slint::SharedString::from(p))
            .collect();
        let model = std::rc::Rc::new(slint::VecModel::from(process_model));
        window.set_process_suggestions(slint::ModelRc::from(model));
    }
}

pub fn show_settings_window(
    state: SharedState,
    platform: Arc<WindowsPlatform>,
    hwnd: Option<isize>,
) {
    {
        let mut s = state.lock().unwrap();
        if s.settings_open {
            if let Some(ref weak) = s.settings_window {
                let _ = weak.upgrade_in_event_loop(|window| {
                    window.show().unwrap();
                });
            }
            return;
        }
        s.settings_open = true;

        if let Some(ref weak) = s.settings_window {
            let state_clone = state.clone();
            let platform_clone = platform.clone();
            let _ = weak.upgrade_in_event_loop(move |window| {
                populate_settings_window(&window, &state_clone, &platform_clone);
                window.show().unwrap();
            });
            return;
        }
    }

    let state_for_cleanup = state.clone();
    let platform_clone = platform.clone();
    std::thread::spawn(move || {
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let window = match SettingsWindow::new() {
                Ok(w) => w,
                Err(e) => {
                    crate::tracing::error!("Failed to create settings window: {}", e);
                    let mut s = state.lock().unwrap();
                    s.settings_open = false;
                    return;
                }
            };

            // Store the weak reference in state
            {
                let mut s = state.lock().unwrap();
                s.settings_window = Some(window.as_weak());
            }

            populate_settings_window(&window, &state, &platform_clone);

            let window_weak = window.as_weak();
            let platform_clone2 = platform_clone.clone();

            // Chip callbacks — watched
            window.on_watched_add({
                let w = window_weak.clone();
                move |text: slint::SharedString| {
                    let win = w.unwrap();
                    let current = win.get_watched_processes();
                    let updated = add_to_list(&current, &text);
                    win.set_watched_processes(slint::SharedString::from(updated.as_str()));
                    sync_tags_str(&win, &updated, "watched");
                }
            });
            window.on_watched_remove({
                let w = window_weak.clone();
                move |idx: i32| {
                    let win = w.unwrap();
                    let current = win.get_watched_processes();
                    let updated = remove_from_list(&current, idx as usize);
                    win.set_watched_processes(slint::SharedString::from(updated.as_str()));
                    sync_tags_str(&win, &updated, "watched");
                }
            });

            // Chip callbacks — excluded
            window.on_excluded_add({
                let w = window_weak.clone();
                move |text: slint::SharedString| {
                    let win = w.unwrap();
                    let current = win.get_excluded_processes();
                    let updated = add_to_list(&current, &text);
                    win.set_excluded_processes(slint::SharedString::from(updated.as_str()));
                    sync_tags_str(&win, &updated, "excluded");
                }
            });
            window.on_excluded_remove({
                let w = window_weak.clone();
                move |idx: i32| {
                    let win = w.unwrap();
                    let current = win.get_excluded_processes();
                    let updated = remove_from_list(&current, idx as usize);
                    win.set_excluded_processes(slint::SharedString::from(updated.as_str()));
                    sync_tags_str(&win, &updated, "excluded");
                }
            });

            let state_clone_save = state.clone();
            window.on_save_clicked(move || {
                let window = window_weak.unwrap();
                let mut s = state_clone_save.lock().unwrap();
                let cfg = Arc::make_mut(&mut s.config);

                cfg.sleep_delay_seconds = (window.get_sleep_delay_minutes().round() as u64) * 60;
                cfg.hibernate = window.get_hibernate();
                cfg.sound_enabled = window.get_sound_enabled();
                cfg.auto_start = window.get_auto_start();
                cfg.display_off_on_sleep = window.get_display_off_on_sleep();
                cfg.warn_before_sleep = window.get_warn_before_sleep();
                cfg.warn_sound_enabled = window.get_warn_sound_enabled();
                cfg.display_state_by_icon = window.get_display_state_by_icon();

                cfg.cpu.enabled = window.get_cpu_enabled();
                cfg.cpu.threshold = window.get_cpu_threshold().round() as f64;
                cfg.cpu.delay_seconds = 180;

                cfg.network.enabled = window.get_network_enabled();
                cfg.network.threshold = (window.get_network_threshold_kb().round() as f64) * 1000.0;
                cfg.network.delay_seconds = 180;

                cfg.disk_write.enabled = window.get_disk_write_enabled();
                cfg.disk_write.threshold = (window.get_disk_write_threshold_kb().round() as f64) * 1000.0;
                cfg.disk_write.delay_seconds = 180;

                let parse_list = |s: slint::SharedString| -> Vec<String> {
                    s.split(',')
                        .map(|item| item.trim().to_string())
                        .filter(|item| !item.is_empty())
                        .collect()
                };

                cfg.excluded_processes = parse_list(window.get_excluded_processes());
                cfg.watched_processes = parse_list(window.get_watched_processes());
                cfg.watched_printers = vec![];

                if let Err(e) = cfg.save(&Config::config_path()) {
                    crate::tracing::error!("Failed to save config: {}", e);
                }

                let _ = platform_clone2.set_auto_start(cfg.auto_start);

                if let Some(hwnd) = hwnd {
                    unsafe {
                        let _ = windows::Win32::UI::WindowsAndMessaging::PostMessageW(
                            windows::Win32::Foundation::HWND(hwnd as *mut std::ffi::c_void),
                            crate::tray::WM_UPDATE_TRAY,
                            windows::Win32::Foundation::WPARAM(0),
                            windows::Win32::Foundation::LPARAM(0),
                        );
                    }
                }

                window.hide().unwrap();
                s.settings_open = false;
            });

            let window_weak2 = window.as_weak();
            let state_clone_cancel = state.clone();
            window.on_cancel_clicked(move || {
                let window = window_weak2.unwrap();
                window.hide().unwrap();
                let mut s = state_clone_cancel.lock().unwrap();
                s.settings_open = false;
            });

            let state_clone_close = state.clone();
            window.window().on_close_requested(move || {
                let mut s = state_clone_close.lock().unwrap();
                s.settings_open = false;
                slint::CloseRequestResponse::HideWindow
            });

            // Setup real-time monitoring timer (every 1 second)
            let window_weak_timer = window.as_weak();
            let platform_timer = platform_clone.clone();
            let timer = slint::Timer::default();
            timer.start(
                slint::TimerMode::Repeated,
                std::time::Duration::from_millis(1000),
                move || {
                    if let Some(window) = window_weak_timer.upgrade() {
                        let mut new_cpu = 0.0;
                        let mut new_network = 0.0;
                        let mut new_disk_write = 0.0;
                        let mut sound_rms = 0.0;

                        if let Ok(perf) = platform_timer.query_performance() {
                            new_cpu = perf.cpu_percent as f32;
                            new_network = (perf.network_bytes_per_sec / 1024.0) as f32;
                            new_disk_write = (perf.disk_write_bytes_per_sec / 1024.0) as f32;

                            window.set_current_cpu(new_cpu);
                            window.set_current_network_kb(new_network);
                            window.set_current_disk_write_kb(new_disk_write);
                        }
                        if let Ok(sound) = platform_timer.current_sound_rms() {
                            sound_rms = sound as f32;
                            window.set_current_sound_active(sound_rms >= 0.01);
                        }

                        // Shift CPU History
                        window.set_cpu_history_29(window.get_cpu_history_28());
                        window.set_cpu_history_28(window.get_cpu_history_27());
                        window.set_cpu_history_27(window.get_cpu_history_26());
                        window.set_cpu_history_26(window.get_cpu_history_25());
                        window.set_cpu_history_25(window.get_cpu_history_24());
                        window.set_cpu_history_24(window.get_cpu_history_23());
                        window.set_cpu_history_23(window.get_cpu_history_22());
                        window.set_cpu_history_22(window.get_cpu_history_21());
                        window.set_cpu_history_21(window.get_cpu_history_20());
                        window.set_cpu_history_20(window.get_cpu_history_19());
                        window.set_cpu_history_19(window.get_cpu_history_18());
                        window.set_cpu_history_18(window.get_cpu_history_17());
                        window.set_cpu_history_17(window.get_cpu_history_16());
                        window.set_cpu_history_16(window.get_cpu_history_15());
                        window.set_cpu_history_15(window.get_cpu_history_14());
                        window.set_cpu_history_14(window.get_cpu_history_13());
                        window.set_cpu_history_13(window.get_cpu_history_12());
                        window.set_cpu_history_12(window.get_cpu_history_11());
                        window.set_cpu_history_11(window.get_cpu_history_10());
                        window.set_cpu_history_10(window.get_cpu_history_9());
                        window.set_cpu_history_9(window.get_cpu_history_8());
                        window.set_cpu_history_8(window.get_cpu_history_7());
                        window.set_cpu_history_7(window.get_cpu_history_6());
                        window.set_cpu_history_6(window.get_cpu_history_5());
                        window.set_cpu_history_5(window.get_cpu_history_4());
                        window.set_cpu_history_4(window.get_cpu_history_3());
                        window.set_cpu_history_3(window.get_cpu_history_2());
                        window.set_cpu_history_2(window.get_cpu_history_1());
                        window.set_cpu_history_1(window.get_cpu_history_0());
                        window.set_cpu_history_0(new_cpu);

                        // Shift Network History
                        window.set_network_history_29(window.get_network_history_28());
                        window.set_network_history_28(window.get_network_history_27());
                        window.set_network_history_27(window.get_network_history_26());
                        window.set_network_history_26(window.get_network_history_25());
                        window.set_network_history_25(window.get_network_history_24());
                        window.set_network_history_24(window.get_network_history_23());
                        window.set_network_history_23(window.get_network_history_22());
                        window.set_network_history_22(window.get_network_history_21());
                        window.set_network_history_21(window.get_network_history_20());
                        window.set_network_history_20(window.get_network_history_19());
                        window.set_network_history_19(window.get_network_history_18());
                        window.set_network_history_18(window.get_network_history_17());
                        window.set_network_history_17(window.get_network_history_16());
                        window.set_network_history_16(window.get_network_history_15());
                        window.set_network_history_15(window.get_network_history_14());
                        window.set_network_history_14(window.get_network_history_13());
                        window.set_network_history_13(window.get_network_history_12());
                        window.set_network_history_12(window.get_network_history_11());
                        window.set_network_history_11(window.get_network_history_10());
                        window.set_network_history_10(window.get_network_history_9());
                        window.set_network_history_9(window.get_network_history_8());
                        window.set_network_history_8(window.get_network_history_7());
                        window.set_network_history_7(window.get_network_history_6());
                        window.set_network_history_6(window.get_network_history_5());
                        window.set_network_history_5(window.get_network_history_4());
                        window.set_network_history_4(window.get_network_history_3());
                        window.set_network_history_3(window.get_network_history_2());
                        window.set_network_history_2(window.get_network_history_1());
                        window.set_network_history_1(window.get_network_history_0());
                        window.set_network_history_0(new_network);

                        // Shift Disk Write History
                        window.set_disk_write_history_29(window.get_disk_write_history_28());
                        window.set_disk_write_history_28(window.get_disk_write_history_27());
                        window.set_disk_write_history_27(window.get_disk_write_history_26());
                        window.set_disk_write_history_26(window.get_disk_write_history_25());
                        window.set_disk_write_history_25(window.get_disk_write_history_24());
                        window.set_disk_write_history_24(window.get_disk_write_history_23());
                        window.set_disk_write_history_23(window.get_disk_write_history_22());
                        window.set_disk_write_history_22(window.get_disk_write_history_21());
                        window.set_disk_write_history_21(window.get_disk_write_history_20());
                        window.set_disk_write_history_20(window.get_disk_write_history_19());
                        window.set_disk_write_history_19(window.get_disk_write_history_18());
                        window.set_disk_write_history_18(window.get_disk_write_history_17());
                        window.set_disk_write_history_17(window.get_disk_write_history_16());
                        window.set_disk_write_history_16(window.get_disk_write_history_15());
                        window.set_disk_write_history_15(window.get_disk_write_history_14());
                        window.set_disk_write_history_14(window.get_disk_write_history_13());
                        window.set_disk_write_history_13(window.get_disk_write_history_12());
                        window.set_disk_write_history_12(window.get_disk_write_history_11());
                        window.set_disk_write_history_11(window.get_disk_write_history_10());
                        window.set_disk_write_history_10(window.get_disk_write_history_9());
                        window.set_disk_write_history_9(window.get_disk_write_history_8());
                        window.set_disk_write_history_8(window.get_disk_write_history_7());
                        window.set_disk_write_history_7(window.get_disk_write_history_6());
                        window.set_disk_write_history_6(window.get_disk_write_history_5());
                        window.set_disk_write_history_5(window.get_disk_write_history_4());
                        window.set_disk_write_history_4(window.get_disk_write_history_3());
                        window.set_disk_write_history_3(window.get_disk_write_history_2());
                        window.set_disk_write_history_2(window.get_disk_write_history_1());
                        window.set_disk_write_history_1(window.get_disk_write_history_0());
                        window.set_disk_write_history_0(new_disk_write);

                        // Shift Sound History
                        window.set_sound_history_29(window.get_sound_history_28());
                        window.set_sound_history_28(window.get_sound_history_27());
                        window.set_sound_history_27(window.get_sound_history_26());
                        window.set_sound_history_26(window.get_sound_history_25());
                        window.set_sound_history_25(window.get_sound_history_24());
                        window.set_sound_history_24(window.get_sound_history_23());
                        window.set_sound_history_23(window.get_sound_history_22());
                        window.set_sound_history_22(window.get_sound_history_21());
                        window.set_sound_history_21(window.get_sound_history_20());
                        window.set_sound_history_20(window.get_sound_history_19());
                        window.set_sound_history_19(window.get_sound_history_18());
                        window.set_sound_history_18(window.get_sound_history_17());
                        window.set_sound_history_17(window.get_sound_history_16());
                        window.set_sound_history_16(window.get_sound_history_15());
                        window.set_sound_history_15(window.get_sound_history_14());
                        window.set_sound_history_14(window.get_sound_history_13());
                        window.set_sound_history_13(window.get_sound_history_12());
                        window.set_sound_history_12(window.get_sound_history_11());
                        window.set_sound_history_11(window.get_sound_history_10());
                        window.set_sound_history_10(window.get_sound_history_9());
                        window.set_sound_history_9(window.get_sound_history_8());
                        window.set_sound_history_8(window.get_sound_history_7());
                        window.set_sound_history_7(window.get_sound_history_6());
                        window.set_sound_history_6(window.get_sound_history_5());
                        window.set_sound_history_5(window.get_sound_history_4());
                        window.set_sound_history_4(window.get_sound_history_3());
                        window.set_sound_history_3(window.get_sound_history_2());
                        window.set_sound_history_2(window.get_sound_history_1());
                        window.set_sound_history_1(window.get_sound_history_0());
                        window.set_sound_history_0(sound_rms);
                    }
                },
            );

            window.show().unwrap();

            let _ = slint::run_event_loop_until_quit();
        }));

        let mut s = state_for_cleanup.lock().unwrap();
        s.settings_window = None;
        s.settings_open = false;
        if let Err(panic_err) = result {
            crate::tracing::error!("Settings window thread panicked: {:?}", panic_err);
        }
    });
}

fn add_to_list(csv: &str, item: &str) -> String {
    let mut items: Vec<String> = csv.split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();
    let trimmed = item.trim().to_string();
    if !trimmed.is_empty() && !items.iter().any(|i| i.eq_ignore_ascii_case(&trimmed)) {
        items.push(trimmed);
    }
    items.join(", ")
}

fn remove_from_list(csv: &str, idx: usize) -> String {
    let mut items: Vec<String> = csv.split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();
    if idx < items.len() {
        items.remove(idx);
    }
    items.join(", ")
}

fn sync_tags(window: &SettingsWindow, items: &[String], prefix: &str) {
    sync_tags_str(window, &items.join(", "), prefix)
}

fn sync_tags_str(window: &SettingsWindow, csv: &str, prefix: &str) {
    let items: Vec<&str> = csv.split(',')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .collect();
    let count = items.len() as i32;
    let set_count = |w: &SettingsWindow, c: i32| match prefix {
        "watched" => w.set_watched_tag_count(c),
        "excluded" => w.set_excluded_tag_count(c),
        _ => {}
    };
    let set_tag = |w: &SettingsWindow, idx: i32, val: slint::SharedString| match prefix {
        "watched" => match idx {
            0 => w.set_watched_tag_0(val), 1 => w.set_watched_tag_1(val),
            2 => w.set_watched_tag_2(val), 3 => w.set_watched_tag_3(val),
            4 => w.set_watched_tag_4(val), 5 => w.set_watched_tag_5(val),
            6 => w.set_watched_tag_6(val), 7 => w.set_watched_tag_7(val),
            _ => {}
        },
        "excluded" => match idx {
            0 => w.set_excluded_tag_0(val), 1 => w.set_excluded_tag_1(val),
            2 => w.set_excluded_tag_2(val), 3 => w.set_excluded_tag_3(val),
            4 => w.set_excluded_tag_4(val), 5 => w.set_excluded_tag_5(val),
            6 => w.set_excluded_tag_6(val), 7 => w.set_excluded_tag_7(val),
            _ => {}
        },
        _ => {}
    };
    set_count(window, count);
    for (i, item) in items.iter().enumerate() {
        set_tag(window, i as i32, slint::SharedString::from(*item));
    }
    // Clear remaining slots
    for i in items.len()..8 {
        set_tag(window, i as i32, slint::SharedString::from(""));
    }
}
