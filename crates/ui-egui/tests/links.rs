//! Independent community links and factual upstream attribution in the preview shell.

use egui_kittest::Harness;
use egui_kittest::kittest::Queryable;
use pdfcraft_engine::links;
use pdfcraft_ui_egui::{Dialog, PdfCraftApp};

fn harness(setup: impl FnOnce(&mut PdfCraftApp) + 'static) -> Harness<'static, PdfCraftApp> {
    let mut h = Harness::builder().with_size(egui::vec2(1400.0, 900.0)).build_eframe(move |_cc| {
        let mut app = PdfCraftApp::new();
        setup(&mut app);
        app
    });
    h.run_steps(4);
    h
}

#[test]
fn contributor_button_in_the_top_bar_opens_the_independent_profile() {
    let mut h = harness(|_| {});
    h.get_by_label("Contributor profile").click();
    h.run_steps(2);
    assert_eq!(h.state().last_opened_url.as_deref(), Some(links::DISCORD));
    assert_eq!(links::DISCORD, "https://github.com/mhadidi06");
}

#[test]
fn home_screen_links() {
    for (label, url) in [
        ("Community contributor", links::DISCORD),
        ("Upstream project", "https://github.com/storytold/pdfcraft"),
        ("Upstream source", "https://github.com/storytold/pdfcraft"),
        ("Upstream license", "https://github.com/storytold/pdfcraft/blob/main/LICENSE-MIT"),
    ] {
        let mut h = harness(|_| {});
        h.get_by_label("Community testing");
        h.get_by_label(label).click();
        h.run_steps(2);
        assert_eq!(h.state().last_opened_url.as_deref(), Some(url), "{label}");
    }
}

#[test]
fn about_dialog_shows_independent_identity_disclaimer_and_attribution_links() {
    let pdf = b"%PDF-1.7\n1 0 obj << /Type /Catalog /Pages 2 0 R >> endobj\n2 0 obj << /Type /Pages /Kids [3 0 R] /Count 1 >> endobj\n3 0 obj << /Type /Page /Parent 2 0 R /MediaBox [0 0 200 200] >> endobj\ntrailer << /Root 1 0 R >>\n%%EOF";
    // With a document open, so the home screen's own links are not on screen.
    let mut h = harness(move |app| {
        app.open_bytes("one.pdf", None, pdf.to_vec()).unwrap();
        app.dialog = Some(Dialog::About);
    });
    assert!(h.query_all_by_label("PDF Navigation Preview").count() >= 2, "neutral icon and application name");
    assert_eq!(h.query_all_by_label("ArtCraft").count(), 0, "no protected brand image alt text");
    h.get_by_label(pdfcraft_ui_egui::identity::DISCLAIMER);
    h.get_by_label("Unofficial Community Beta — Windows x64");
    h.get_by_label("Community contributor").click();
    h.run_steps(2);
    assert_eq!(h.state().last_opened_url.as_deref(), Some(links::DISCORD));
}

#[test]
fn help_commands_open_each_link() {
    for l in links::LINKS {
        let mut h = harness(|_| {});
        assert!(h.state_mut().execute(l.command), "{}", l.command);
        assert_eq!(h.state().last_opened_url.as_deref(), Some(l.url));
        assert_eq!(pdfcraft_engine::commands::command(l.command).unwrap().menu, Some("Help"));
    }
}

#[test]
fn about_dialog_has_contributors_and_models_tabs() {
    let mut h = harness(|app| app.dialog = Some(Dialog::About));
    h.get_by_label("Contributors").click();
    h.run_steps(2);
    // The owner is always in the compiled-in credits (contributors/contributors.json), shown by username.
    h.get_by_label("@echelon");
    h.get_by_label("Table").click();
    h.run_steps(2);
    h.get_by_label("PRs");
    h.get_by_label("Display name").click();
    h.run_steps(2);
    h.get_by_label("Brandon Thomas");
    h.get_by_label("Models").click();
    h.run_steps(2);
    assert!(h.query_all_by_label("Anthropic").count() >= 1);
}
