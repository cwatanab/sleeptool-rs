use std::sync::Arc;
use crate::config::Config;
use crate::state::SharedState;
use crate::platform_win32::WindowsPlatform;
use crate::platform::{AudioProbe, PerformanceProbe, Platform, ProcessProbe, StartupControl};

pub mod list_utils;

slint::slint! {
    import { LineEdit, Slider, ScrollView } from "std-widgets.slint";

    component SidebarButton inherits Rectangle {
        in-out property <string> text;
        in-out property <bool> active;
        in-out property <image> icon;
        callback clicked();

        border-radius: 6px;
        background: active ? rgb(99, 102, 241, 0.12) : (touch.has-hover ? rgb(255, 255, 255, 0.04) : transparent);
        height: 38px;

        HorizontalLayout {
            padding-left: 14px;
            padding-right: 14px;
            spacing: 12px;

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
                    color: root.active ? rgb(255, 255, 255) : (touch.has-hover ? rgb(220, 220, 230) : rgb(150, 150, 160));
                    font-weight: root.active ? 600 : 500;
                }
            }
        }

        if (root.active) : Rectangle {
            x: 0px;
            y: 9px;
            width: 3px;
            height: 20px;
            border-radius: 1.5px;
            background: rgb(99, 102, 241);
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
            ? @linear-gradient(180deg, rgb(244, 63, 94) 0%, rgb(225, 29, 72) 100%)
            : @linear-gradient(180deg, rgb(99, 102, 241) 0%, rgb(79, 70, 229) 100%);
    }

    component Sparkline inherits Rectangle {
        in property <[float]> values;
        in property <float> max-val: 100.0;
        in property <length> chart-height: 18px;
        in property <bool> show-threshold: false;
        in property <float> threshold-ratio: 0.0;

        height: 20px;
        width: 100px;
        background: rgb(10, 10, 15);
        border-radius: 6px;
        border-width: 1px;
        border-color: rgb(45, 45, 58);

        HorizontalLayout {
            padding-left: 3px;
            padding-right: 3px;
            padding-top: 1px;
            padding-bottom: 1px;
            spacing: 1px;

            for i in [14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29] : VerticalLayout {
                alignment: end;
                Bar {
                    val: root.values[i];
                    max-val: root.max-val;
                    chart-height: root.chart-height;
                    show-threshold: root.show-threshold;
                    threshold-ratio: root.threshold-ratio;
                }
            }
        }

        Rectangle {
            x: 2px;
            y: root.height - 1px - root.chart-height * 0.5;
            width: root.width - 4px;
            height: 1px;
            background: rgb(45, 45, 58);
        }

        if (root.show-threshold) : Rectangle {
            x: 0px;
            y: root.height - 1px - root.threshold-ratio * root.chart-height;
            width: root.width;
            height: 1px;
            background: rgb(244, 63, 94);
        }
    }

    component CustomButton inherits Rectangle {
        in-out property <string> text;
        in-out property <bool> primary: false;
        in-out property <bool> enabled: true;
        callback clicked();

        border-radius: 6px;
        border-width: 1px;
        
        background: !root.enabled 
            ? rgb(30, 30, 38) 
            : (root.primary 
                ? (touch.pressed ? rgb(67, 56, 202) : (touch.has-hover ? rgb(99, 102, 241) : rgb(79, 70, 229)))
                : (touch.pressed ? rgb(35, 35, 45) : (touch.has-hover ? rgb(45, 45, 58) : rgb(30, 30, 40))));
        
        border-color: !root.enabled
            ? rgb(45, 45, 55)
            : (root.primary
                ? rgb(99, 102, 241)
                : rgb(55, 55, 70));
        
        height: 32px;

        HorizontalLayout {
            alignment: center;
            VerticalLayout {
                alignment: center;
                Text {
                    text: root.text;
                    color: !root.enabled
                        ? rgb(100, 100, 110)
                        : (root.primary ? rgb(255, 255, 255) : rgb(220, 220, 230));
                    font-weight: root.primary ? 600 : 500;
                }
            }
        }

        touch := TouchArea {
            enabled: root.enabled;
            clicked => { root.clicked(); }
        }
    }

    component CustomCheckBox inherits Rectangle {
        in-out property <string> text;
        in-out property <bool> checked: false;
        in-out property <bool> enabled: true;
        callback toggled();

        height: 24px;
        background: transparent;

        HorizontalLayout {
            spacing: 10px;
            alignment: start;

            VerticalLayout {
                alignment: center;
                Rectangle {
                    width: 18px;
                    height: 18px;
                    border-radius: 4px;
                    border-width: 1px;
                    border-color: root.checked ? rgb(99, 102, 241) : rgb(60, 60, 75);
                    background: root.checked ? rgb(99, 102, 241) : rgb(24, 24, 32);
                    
                    if (root.checked) : Rectangle {
                        width: 8px;
                        height: 8px;
                        border-radius: 1px;
                        background: rgb(255, 255, 255);
                    }
                }
            }

            VerticalLayout {
                alignment: center;
                Text {
                    text: root.text;
                    color: root.enabled ? rgb(220, 220, 230) : rgb(120, 120, 130);
                }
            }
        }

        touch := TouchArea {
            enabled: root.enabled;
            clicked => {
                root.checked = !root.checked;
                root.toggled();
            }
        }
    }

    component ConfigCard inherits Rectangle {
        in property <string> title;
        
        background: rgb(22, 22, 30);
        border-radius: 8px;
        border-width: 1px;
        border-color: rgb(35, 35, 48);

        VerticalLayout {
            padding: 16px;
            spacing: 14px;
            
            if (root.title != "") : HorizontalLayout {
                spacing: 8px;
                alignment: start;
                
                VerticalLayout {
                    alignment: center;
                    Rectangle {
                        width: 4px;
                        height: 12px;
                        border-radius: 2px;
                        background: rgb(99, 102, 241);
                    }
                }
                
                Text {
                    text: root.title;
                    font-size: 16px;
                    font-weight: 700;
                    color: rgb(235, 235, 245);
                    vertical-alignment: center;
                }
            }
            
            @children
        }
    }

    component TagChip inherits Rectangle {
        in property <string> text;
        callback remove();

        background: rgb(99, 102, 241, 0.12);
        border-radius: 6px;
        border-width: 1px;
        border-color: rgb(99, 102, 241, 0.35);
        height: 24px;

        HorizontalLayout {
            padding-left: 8px;
            padding-right: 6px;
            spacing: 6px;
            alignment: center;

            Text {
                text: root.text;
                color: rgb(199, 210, 254);
                font-size: 12px;
                vertical-alignment: center;
            }

            t := TouchArea {
                width: 12px;
                height: 12px;
                clicked => { root.remove(); }
                
                Text {
                    text: "×";
                    color: t.has-hover ? rgb(244, 63, 94) : rgb(165, 180, 252);
                    font-size: 13px;
                    font-weight: 700;
                    x: 2px;
                    y: -1px;
                }
            }
        }
    }

    component AutocompleteInput inherits HorizontalLayout {
        in property <string> placeholder-text: "";
        in property <[string]> suggestions: [];
        in-out property <string> text <=> input.text;
        callback accepted(string);

        spacing: 4px;

        input := LineEdit {
            height: 32px;
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

            btn := CustomButton {
                text: "▼";
                width: 30px;
                height: 32px;
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
                    background: rgb(30, 30, 40);
                    border-color: rgb(55, 55, 75);
                    border-width: 1px;
                    border-radius: 6px;

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
                                    background: ta.has-hover ? rgb(99, 102, 241, 0.15) : transparent;
                                    border-radius: 4px;

                                    Text {
                                        text: item;
                                        color: rgb(230, 230, 240);
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
            spacing: 6px;
            alignment: start;

            if (root.tag-count > 0 && root.tag-0 != "") : TagChip { text: root.tag-0; remove => { root.remove-item(0); } }
            if (root.tag-count > 1 && root.tag-1 != "") : TagChip { text: root.tag-1; remove => { root.remove-item(1); } }
            if (root.tag-count > 2 && root.tag-2 != "") : TagChip { text: root.tag-2; remove => { root.remove-item(2); } }
            if (root.tag-count > 3 && root.tag-3 != "") : TagChip { text: root.tag-3; remove => { root.remove-item(3); } }
            if (root.tag-count > 4 && root.tag-4 != "") : TagChip { text: root.tag-4; remove => { root.remove-item(4); } }
            if (root.tag-count > 5 && root.tag-5 != "") : TagChip { text: root.tag-5; remove => { root.remove-item(5); } }
            if (root.tag-count > 6 && root.tag-6 != "") : TagChip { text: root.tag-6; remove => { root.remove-item(6); } }
            if (root.tag-count > 7 && root.tag-7 != "") : TagChip { text: root.tag-7; remove => { root.remove-item(7); } }
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

            CustomButton {
                text: "追加";
                primary: true;
                width: 60px;
                height: 32px;
                clicked => {
                    if (input.text != "") {
                        root.add-item(input.text);
                        input.text = "";
                    }
                }
            }
        }
    }

    component SensorConfigPanel inherits Rectangle {
        in-out property <bool> sensor-enabled: true;
        in-out property <string> threshold-text: "";
        in-out property <float> threshold-val: 1.0;
        in-out property <float> threshold-min: 1.0;
        in-out property <float> threshold-max: 100.0;
        in-out property <string> current-val-text: "";

        in property <[float]> chart-values;
        in property <float> max-chart-val: 100.0;
        in property <length> chart-width: 90px;
        in property <length> chart-height: 20px;
        in property <length> chart-bar-height: 18px;
        in property <bool> show-threshold: false;
        in property <float> threshold-ratio: 0.0;

        in property <string> title: "";

        callback toggle-enabled();
        callback threshold-changed(float);

        background: rgb(22, 22, 30);
        border-radius: 8px;
        border-width: 1px;
        border-color: rgb(35, 35, 48);

        VerticalLayout {
            padding: 16px;
            spacing: 14px;
            
            if (root.title != "") : HorizontalLayout {
                spacing: 8px;
                alignment: start;
                
                VerticalLayout {
                    alignment: center;
                    Rectangle {
                        width: 4px;
                        height: 12px;
                        border-radius: 2px;
                        background: rgb(99, 102, 241);
                    }
                }
                
                Text {
                    text: root.title;
                    font-size: 16px;
                    font-weight: 700;
                    color: rgb(235, 235, 245);
                    vertical-alignment: center;
                }
            }

            CustomCheckBox {
                text: "このセンサーを有効にする";
                checked: root.sensor-enabled;
                toggled => { root.toggle-enabled(); }
            }

            if (root.sensor-enabled) : VerticalLayout {
                spacing: 12px;
                HorizontalLayout {
                    spacing: 12px;
                    VerticalLayout {
                        alignment: center;
                        Text {
                            text: root.current-val-text;
                            color: rgb(129, 140, 248);
                            font-weight: 700;
                            vertical-alignment: center;
                        }
                    }
                    Rectangle {}
                    Sparkline {
                        values: root.chart-values;
                        max-val: root.max-chart-val;
                        width: root.chart-width;
                        height: root.chart-height;
                        chart-height: root.chart-bar-height;
                        show-threshold: root.show-threshold;
                        threshold-ratio: root.threshold-ratio;
                    }
                }
                VerticalLayout {
                    spacing: 6px;
                    Text {
                        text: root.threshold-text;
                        color: rgb(180, 180, 195);
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
    }

    component MultimediaConfigPanel inherits Rectangle {
        in-out property <bool> sensor-enabled: true;
        in-out property <string> current-val-text: "";
        in-out property <bool> current-sound-active: false;

        in property <[float]> chart-values: [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        in property <length> chart-width: 90px;
        in property <length> chart-height: 20px;
        in property <length> chart-bar-height: 18px;
        in property <bool> show-threshold: false;
        in property <float> threshold-ratio: 0.0;

        in property <string> title: "";

        callback toggle-enabled();

        background: rgb(22, 22, 30);
        border-radius: 8px;
        border-width: 1px;
        border-color: rgb(35, 35, 48);

        VerticalLayout {
            padding: 16px;
            spacing: 14px;
            
            if (root.title != "") : HorizontalLayout {
                spacing: 8px;
                alignment: start;
                
                VerticalLayout {
                    alignment: center;
                    Rectangle {
                        width: 4px;
                        height: 12px;
                        border-radius: 2px;
                        background: rgb(99, 102, 241);
                    }
                }
                
                Text {
                    text: root.title;
                    font-size: 16px;
                    font-weight: 700;
                    color: rgb(235, 235, 245);
                    vertical-alignment: center;
                }
            }

            CustomCheckBox {
                text: "音声再生中はスリープを抑止する";
                checked: root.sensor-enabled;
                toggled => { root.toggle-enabled(); }
            }

            if (root.sensor-enabled) : VerticalLayout {
                spacing: 12px;
                HorizontalLayout {
                    spacing: 12px;
                    VerticalLayout {
                        alignment: center;
                        Text {
                            text: root.current-val-text;
                            color: root.current-sound-active ? rgb(129, 140, 248) : rgb(140, 140, 150);
                            font-weight: 700;
                            vertical-alignment: center;
                        }
                    }
                    Rectangle {}
                    Sparkline {
                        values: root.chart-values;
                        max-val: 0.05;
                        width: root.chart-width;
                        height: root.chart-height;
                        chart-height: root.chart-bar-height;
                        show-threshold: root.show-threshold;
                        threshold-ratio: root.threshold-ratio;
                    }
                }
                warn_box := Rectangle {
                    background: rgb(15, 15, 20);
                    border-radius: 6px;
                    border-width: 1px;
                    border-color: rgb(28, 28, 38);
                    height: 54px;
                    
                    Text {
                        text: "音声出力（サウンドカードの出力振幅レベル）を検知して、メディア再生中の自動スリープ移行を抑止します。";
                        font-size: 11px;
                        color: rgb(150, 150, 160);
                        wrap: word-wrap;
                        x: 12px;
                        y: 8px;
                        width: warn_box.width - 24px;
                        height: warn_box.height - 16px;
                    }
                }
            }
        }
    }

    export component SettingsWindow inherits Window {
        title: "SleepTool 設定";
        icon: @image-url("../../assets/icons/default.png");
        width: 640px;
        height: 480px;
        background: rgb(13, 13, 17);
        default-font-size: 14px;
        default-font-family: "";

        in-out property <int> active-tab: 0;

        in-out property <float> sleep-delay-minutes: 10.0;
        in-out property <bool> hibernate: false;
        in-out property <bool> sound-enabled: true;
        in-out property <bool> auto-start: false;
        in-out property <bool> display-off-on-sleep: true;
        in-out property <bool> warn-before-sleep: true;
        in-out property <bool> warn-sound-enabled: true;
        in-out property <bool> display-state-by-icon: true;

        in-out property <bool> cpu-enabled: true;
        in-out property <float> cpu-threshold: 1.0;

        in-out property <bool> network-enabled: true;
        in-out property <float> network-threshold-kb: 10.0;

        in-out property <bool> disk-write-enabled: true;
        in-out property <float> disk-write-threshold-kb: 10.0;

        in-out property <string> excluded-processes: "";
        in-out property <string> watched-processes: "";
        in-out property <string> watched-tags: "";
        in-out property <string> excluded-tags: "";
        in property <[string]> process-suggestions: [];

        in-out property <int> watched-tag-count: 0;
        in-out property <string> watched-tag-0: "";
        in-out property <string> watched-tag-1: "";
        in-out property <string> watched-tag-2: "";
        in-out property <string> watched-tag-3: "";
        in-out property <string> watched-tag-4: "";
        in-out property <string> watched-tag-5: "";
        in-out property <string> watched-tag-6: "";
        in-out property <string> watched-tag-7: "";

        in-out property <int> excluded-tag-count: 0;
        in-out property <string> excluded-tag-0: "";
        in-out property <string> excluded-tag-1: "";
        in-out property <string> excluded-tag-2: "";
        in-out property <string> excluded-tag-3: "";
        in-out property <string> excluded-tag-4: "";
        in-out property <string> excluded-tag-5: "";
        in-out property <string> excluded-tag-6: "";
        in-out property <string> excluded-tag-7: "";

        in-out property <float> current-cpu: 0.0;
        in-out property <float> current-network-kb: 0.0;
        in-out property <float> current-disk-write-kb: 0.0;
        in-out property <bool> current-sound-active: false;

        in-out property <[float]> cpu-history;
        in-out property <[float]> network-history;
        in-out property <[float]> disk-write-history;
        in-out property <[float]> sound-history;

        callback save-clicked();
        callback cancel-clicked();
        callback watched-add(string);
        callback watched-pop();
        callback excluded-add(string);
        callback excluded-pop();
        callback watched-remove(int);
        callback excluded-remove(int);

        HorizontalLayout {
            Rectangle {
                width: 180px;
                background: rgb(18, 18, 24);

                Rectangle {
                    x: parent.width - 1px;
                    width: 1px;
                    height: parent.height;
                    background: rgb(30, 30, 40);
                }

                VerticalLayout {
                    padding-left: 12px;
                    padding-right: 12px;
                    padding-top: 24px;
                    padding-bottom: 24px;
                    spacing: 8px;
                    alignment: start;

                    SidebarButton {
                        text: "基本設定";
        icon: @image-url("../../assets/icons/default.png");
                        active: root.active-tab == 0;
                        clicked => { root.active-tab = 0; }
                    }
                    SidebarButton {
                        text: "CPU使用率";
                        icon: @image-url("../../assets/icons/cpu.png");
                        active: root.active-tab == 1;
                        clicked => { root.active-tab = 1; }
                    }
                    SidebarButton {
                        text: "ネットワーク";
                        icon: @image-url("../../assets/icons/network.png");
                        active: root.active-tab == 2;
                        clicked => { root.active-tab = 2; }
                    }
                    SidebarButton {
                        text: "ディスク書き込み";
                        icon: @image-url("../../assets/icons/disk.png");
                        active: root.active-tab == 3;
                        clicked => { root.active-tab = 3; }
                    }
                    SidebarButton {
                        text: "マルチメディア";
                        icon: @image-url("../../assets/icons/sound.png");
                        active: root.active-tab == 4;
                        clicked => { root.active-tab = 4; }
                    }
                    SidebarButton {
                        text: "プロセス";
                        icon: @image-url("../../assets/icons/process.png");
                        active: root.active-tab == 5;
                        clicked => { root.active-tab = 5; }
                    }
                }
            }

            VerticalLayout {
                padding: 16px;
                spacing: 16px;

                Rectangle {
                    background: rgb(18, 18, 24);
                    border-radius: 8px;
                    border-width: 1px;
                    border-color: rgb(29, 29, 39);
                    clip: true;

                    if (root.active-tab == 0) : ScrollView {
                        VerticalLayout {
                            padding: 16px;
                            spacing: 16px;
                            alignment: start;

                            ConfigCard {
                                title: "スリープ移行時間";
                                VerticalLayout {
                                    spacing: 10px;
                                    HorizontalLayout {
                                        Text {
                                            text: "無操作時間: " + round(root.sleep-delay-minutes) + " 分";
                                            color: rgb(220, 220, 230);
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

                            ConfigCard {
                                title: "システム動作設定";
                                VerticalLayout {
                                    spacing: 12px;
                                    CustomCheckBox {
                                        text: "スリープの代わりに休止状態（ハイバネート）を使用する";
                                        checked <=> root.hibernate;
                                    }
                                    CustomCheckBox {
                                        text: "スリープ復帰時に自動的に画面をオフにする";
                                        checked <=> root.display-off-on-sleep;
                                    }
                                    CustomCheckBox {
                                        text: "ログイン時に自動起動する";
                                        checked <=> root.auto-start;
                                    }
                                }
                            }

                            ConfigCard {
                                title: "警告・通知設定";
                                VerticalLayout {
                                    spacing: 12px;
                                    CustomCheckBox {
                                        text: "スリープ前にバルーン通知で警告する";
                                        checked <=> root.warn-before-sleep;
                                    }
                                    CustomCheckBox {
                                        text: "スリープ前に警告サウンドを再生する";
                                        checked <=> root.warn-sound-enabled;
                                    }
                                    CustomCheckBox {
                                        text: "トレイアイコンの状態表示を有効にする";
                                        checked <=> root.display-state-by-icon;
                                    }
                                }
                            }
                        }
                    }

                    if (root.active-tab == 1) : ScrollView {
                        VerticalLayout {
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
                                chart-values: root.cpu-history;
                                toggle-enabled => { root.cpu-enabled = !root.cpu-enabled; }
                                threshold-changed(val) => { root.cpu-threshold = val; }
                            }
                        }
                    }

                    if (root.active-tab == 2) : ScrollView {
                        VerticalLayout {
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
                                chart-values: root.network-history;
                                toggle-enabled => { root.network-enabled = !root.network-enabled; }
                                threshold-changed(val) => { root.network-threshold-kb = val; }
                            }
                        }
                    }

                    if (root.active-tab == 3) : ScrollView {
                        VerticalLayout {
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
                                chart-values: root.disk-write-history;
                                toggle-enabled => { root.disk-write-enabled = !root.disk-write-enabled; }
                                threshold-changed(val) => { root.disk-write-threshold-kb = val; }
                            }
                        }
                    }

                    if (root.active-tab == 4) : ScrollView {
                        VerticalLayout {
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
                                chart-values: root.sound-history;
                                toggle-enabled => { root.sound-enabled = !root.sound-enabled; }
                            }
                        }
                    }

                    if (root.active-tab == 5) : ScrollView {
                        VerticalLayout {
                            padding: 16px;
                            spacing: 16px;
                            alignment: start;

                            ConfigCard {
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

                            ConfigCard {
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
                }

                HorizontalLayout {
                    alignment: end;
                    spacing: 12px;
                    height: 32px;

                    CustomButton {
                        text: "キャンセル";
                        width: 100px;
                        clicked => { root.cancel-clicked(); }
                    }

                    CustomButton {
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
fn set_history(window: &SettingsWindow, setter: fn(&SettingsWindow, slint::ModelRc<f32>), data: &[f32]) {
    let model = std::rc::Rc::new(slint::VecModel::from(data.to_vec()));
    setter(window, slint::ModelRc::from(model));
}

fn push_history(hist: &mut Vec<f32>, new_val: f32) {
    hist.insert(0, new_val);
    hist.truncate(30);
    while hist.len() < 30 {
        hist.push(0.0);
    }
}

fn populate_settings_window(
    window: &SettingsWindow,
    state: &SharedState,
    platform: &Arc<WindowsPlatform>,
) {
    let config = {
        let s = state.lock().unwrap();
        s.config.as_ref().clone()
    };

    window.set_sleep_delay_minutes((config.sleep.delay_seconds / 60) as f32);
    window.set_hibernate(config.sleep.hibernate);
    window.set_sound_enabled(config.sound.enabled);
    window.set_auto_start(config.general.auto_start);
    window.set_display_off_on_sleep(config.general.display_off_on_sleep);
    window.set_warn_before_sleep(config.sleep.warn_before_sleep);
    window.set_warn_sound_enabled(config.sleep.warn_sound_enabled);
    window.set_display_state_by_icon(config.general.display_state_by_icon);

    window.set_cpu_enabled(config.cpu.enabled);
    window.set_cpu_threshold(config.cpu.threshold as f32);

    window.set_network_enabled(config.network.enabled);
    window.set_network_threshold_kb((config.network.threshold / 1000.0) as f32);

    window.set_disk_write_enabled(config.disk.write_enabled);
    window.set_disk_write_threshold_kb((config.disk.write_threshold / 1000.0) as f32);

    window.set_excluded_processes(slint::SharedString::from(config.process.excluded.join(", ")));
    window.set_watched_processes(slint::SharedString::from(config.process.watched.join(", ")));
    sync_tags(window, &config.process.watched, "watched");
    sync_tags(window, &config.process.excluded, "excluded");

    let zeros = vec![0.0f32; 30];
    set_history(window, |w, m| w.set_cpu_history(m), &zeros);
    set_history(window, |w, m| w.set_network_history(m), &zeros);
    set_history(window, |w, m| w.set_disk_write_history(m), &zeros);
    set_history(window, |w, m| w.set_sound_history(m), &zeros);

    if let Ok(processes) = ProcessProbe::list_running_processes(platform.as_ref()) {
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

            {
                let mut s = state.lock().unwrap();
                s.settings_window = Some(window.as_weak());
            }

            populate_settings_window(&window, &state, &platform_clone);

            let window_weak = window.as_weak();
            let platform_clone2 = platform_clone.clone();

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

                cfg.sleep.delay_seconds = (window.get_sleep_delay_minutes().round() as u64) * 60;
                cfg.sleep.hibernate = window.get_hibernate();
                cfg.sound.enabled = window.get_sound_enabled();
                cfg.general.auto_start = window.get_auto_start();
                cfg.general.display_off_on_sleep = window.get_display_off_on_sleep();
                cfg.sleep.warn_before_sleep = window.get_warn_before_sleep();
                cfg.sleep.warn_sound_enabled = window.get_warn_sound_enabled();
                cfg.general.display_state_by_icon = window.get_display_state_by_icon();

                cfg.cpu.enabled = window.get_cpu_enabled();
                cfg.cpu.threshold = window.get_cpu_threshold().round() as f64;

                cfg.network.enabled = window.get_network_enabled();
                cfg.network.threshold = (window.get_network_threshold_kb().round() as f64) * 1000.0;

                cfg.disk.write_enabled = window.get_disk_write_enabled();
                cfg.disk.write_threshold = (window.get_disk_write_threshold_kb().round() as f64) * 1000.0;

                let parse_list = |s: slint::SharedString| -> Vec<String> {
                    s.split(',')
                        .map(|item| item.trim().to_lowercase())
                        .filter(|item| !item.is_empty())
                        .collect()
                };

                cfg.process.excluded = parse_list(window.get_excluded_processes());
                cfg.process.watched = parse_list(window.get_watched_processes());

                if let Err(e) = cfg.save(&Config::config_path()) {
                    crate::tracing::error!("Failed to save config: {}", e);
                }

                let _ = StartupControl::set_auto_start(platform_clone2.as_ref(), cfg.general.auto_start);

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

            let window_weak_timer = window.as_weak();
            let platform_timer = platform_clone.clone();
            let cpu_hist = std::sync::Arc::new(std::sync::Mutex::new(vec![0.0f32; 30]));
            let net_hist = std::sync::Arc::new(std::sync::Mutex::new(vec![0.0f32; 30]));
            let disk_hist = std::sync::Arc::new(std::sync::Mutex::new(vec![0.0f32; 30]));
            let sound_hist = std::sync::Arc::new(std::sync::Mutex::new(vec![0.0f32; 30]));
            let timer = slint::Timer::default();
            timer.start(
                slint::TimerMode::Repeated,
                std::time::Duration::from_millis(1000),
                {
                    let cpu = cpu_hist.clone();
                    let net = net_hist.clone();
                    let disk = disk_hist.clone();
                    let sound = sound_hist.clone();
                    move || {
                        if let Some(window) = window_weak_timer.upgrade() {
                            let mut new_cpu = 0.0f32;
                            let mut new_network = 0.0f32;
                            let mut new_disk_write = 0.0f32;
                            let mut sound_rms = 0.0f32;

                            if let Ok(perf) = PerformanceProbe::query_performance(platform_timer.as_ref()) {
                                new_cpu = perf.cpu_percent as f32;
                                new_network = (perf.network_bytes_per_sec / 1024.0) as f32;
                                new_disk_write = (perf.disk_write_bytes_per_sec / 1024.0) as f32;

                                window.set_current_cpu(new_cpu);
                                window.set_current_network_kb(new_network);
                                window.set_current_disk_write_kb(new_disk_write);
                            }
                            if let Ok(sound_val) = AudioProbe::current_sound_rms(platform_timer.as_ref()) {
                                sound_rms = sound_val as f32;
                                window.set_current_sound_active(sound_rms >= 0.01);
                            }

                            let mut ch = cpu.lock().unwrap();
                            push_history(&mut ch, new_cpu);
                            set_history(&window, |w, m| w.set_cpu_history(m), &ch);

                            let mut nh = net.lock().unwrap();
                            push_history(&mut nh, new_network);
                            set_history(&window, |w, m| w.set_network_history(m), &nh);

                            let mut dh = disk.lock().unwrap();
                            push_history(&mut dh, new_disk_write);
                            set_history(&window, |w, m| w.set_disk_write_history(m), &dh);

                            let mut sh = sound.lock().unwrap();
                            push_history(&mut sh, sound_rms);
                            set_history(&window, |w, m| w.set_sound_history(m), &sh);
                        }
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
    match prefix {
        "watched" => window.set_watched_tag_count(count),
        "excluded" => window.set_excluded_tag_count(count),
        _ => {}
    }
    for (i, item) in items.iter().enumerate().take(8) {
        let val = slint::SharedString::from(*item);
        match (prefix, i) {
            ("watched", 0) => window.set_watched_tag_0(val),
            ("watched", 1) => window.set_watched_tag_1(val),
            ("watched", 2) => window.set_watched_tag_2(val),
            ("watched", 3) => window.set_watched_tag_3(val),
            ("watched", 4) => window.set_watched_tag_4(val),
            ("watched", 5) => window.set_watched_tag_5(val),
            ("watched", 6) => window.set_watched_tag_6(val),
            ("watched", 7) => window.set_watched_tag_7(val),
            ("excluded", 0) => window.set_excluded_tag_0(val),
            ("excluded", 1) => window.set_excluded_tag_1(val),
            ("excluded", 2) => window.set_excluded_tag_2(val),
            ("excluded", 3) => window.set_excluded_tag_3(val),
            ("excluded", 4) => window.set_excluded_tag_4(val),
            ("excluded", 5) => window.set_excluded_tag_5(val),
            ("excluded", 6) => window.set_excluded_tag_6(val),
            ("excluded", 7) => window.set_excluded_tag_7(val),
            _ => {}
        }
    }
    for i in items.len()..8 {
        let empty = slint::SharedString::from("");
        match (prefix, i) {
            ("watched", 0) => window.set_watched_tag_0(empty),
            ("watched", 1) => window.set_watched_tag_1(empty),
            ("watched", 2) => window.set_watched_tag_2(empty),
            ("watched", 3) => window.set_watched_tag_3(empty),
            ("watched", 4) => window.set_watched_tag_4(empty),
            ("watched", 5) => window.set_watched_tag_5(empty),
            ("watched", 6) => window.set_watched_tag_6(empty),
            ("watched", 7) => window.set_watched_tag_7(empty),
            ("excluded", 0) => window.set_excluded_tag_0(empty),
            ("excluded", 1) => window.set_excluded_tag_1(empty),
            ("excluded", 2) => window.set_excluded_tag_2(empty),
            ("excluded", 3) => window.set_excluded_tag_3(empty),
            ("excluded", 4) => window.set_excluded_tag_4(empty),
            ("excluded", 5) => window.set_excluded_tag_5(empty),
            ("excluded", 6) => window.set_excluded_tag_6(empty),
            ("excluded", 7) => window.set_excluded_tag_7(empty),
            _ => {}
        }
    }
}
