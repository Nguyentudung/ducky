//! Slint controller: translates UI intent into small domain-service calls.

use crate::{
    AppWindow, ProblemItem, UiStrings,
    core::{
        i18n::language::{AppLanguage, LanguageManager},
        problems::{Problem, ProblemInput, ProblemQuery, repository},
        workspace::{self, write_problem_manifest},
    },
    platform,
    state::{AppState, load_setting, save_setting, workspace_path},
};
use slint::{ComponentHandle, ModelRc, SharedString, VecModel};
use std::{cell::RefCell, path::PathBuf, rc::Rc};

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let state = Rc::new(AppState::new()?);
    let window = AppWindow::new()?;
    let controller = Rc::new(Controller {
        window,
        state,
        query: RefCell::new(ProblemQuery::default()),
        selected_id: RefCell::new(None),
    });
    controller.refresh_all()?;
    Controller::bind(&controller);
    controller.window.run()?;
    Ok(())
}

struct Controller {
    window: AppWindow,
    state: Rc<AppState>,
    query: RefCell<ProblemQuery>,
    selected_id: RefCell<Option<i64>>,
}

impl Controller {
    fn bind(this: &Rc<Self>) {
        let weak = Rc::downgrade(this);
        this.window.on_navigate(move |page| {
            if let Some(c) = weak.upgrade() {
                c.window.set_page(page);
                let _ = c.refresh_page();
            }
        });
        let weak = Rc::downgrade(this);
        this.window.on_new_problem(move || {
            if let Some(c) = weak.upgrade() {
                c.open_new_form();
            }
        });
        let weak = Rc::downgrade(this);
        this.window.on_cancel_form(move || {
            if let Some(c) = weak.upgrade() {
                c.window.set_show_form(false);
            }
        });
        let weak = Rc::downgrade(this);
        this.window.on_submit_problem(move || {
            if let Some(c) = weak.upgrade() {
                c.submit_form();
            }
        });
        let weak = Rc::downgrade(this);
        this.window.on_open_problem(move |id| {
            if let Some(c) = weak.upgrade() {
                c.open_detail(id);
            }
        });
        let weak = Rc::downgrade(this);
        this.window.on_edit_problem(move || {
            if let Some(c) = weak.upgrade() {
                c.open_edit_form();
            }
        });
        let weak = Rc::downgrade(this);
        this.window.on_delete_problem(move || {
            if let Some(c) = weak.upgrade() {
                c.window.set_show_confirm_delete(true);
            }
        });
        let weak = Rc::downgrade(this);
        this.window.on_cancel_delete(move || {
            if let Some(c) = weak.upgrade() {
                c.window.set_show_confirm_delete(false);
            }
        });
        let weak = Rc::downgrade(this);
        this.window.on_confirm_delete(move || {
            if let Some(c) = weak.upgrade() {
                c.delete_selected();
            }
        });
        let weak = Rc::downgrade(this);
        this.window.on_clear_recent(move || {
            if let Some(c) = weak.upgrade() {
                c.window.set_show_confirm_clear(true);
            }
        });
        let weak = Rc::downgrade(this);
        this.window.on_cancel_clear(move || {
            if let Some(c) = weak.upgrade() {
                c.window.set_show_confirm_clear(false);
            }
        });
        let weak = Rc::downgrade(this);
        this.window.on_confirm_clear(move || {
            if let Some(c) = weak.upgrade() {
                c.clear_recent();
            }
        });
        let weak = Rc::downgrade(this);
        this.window.on_search_changed(move |v| {
            if let Some(c) = weak.upgrade() {
                c.set_query(|q| q.search = v.to_string());
            }
        });
        let weak = Rc::downgrade(this);
        this.window.on_status_filter_changed(move |v| {
            if let Some(c) = weak.upgrade() {
                c.set_query(|q| q.status = v.to_string());
            }
        });
        let weak = Rc::downgrade(this);
        this.window.on_difficulty_filter_changed(move |v| {
            if let Some(c) = weak.upgrade() {
                c.set_query(|q| q.difficulty = v.to_string());
            }
        });
        let weak = Rc::downgrade(this);
        this.window.on_language_filter_changed(move |v| {
            if let Some(c) = weak.upgrade() {
                c.set_query(|q| q.language = v.to_string());
            }
        });
        let weak = Rc::downgrade(this);
        this.window.on_platform_filter_changed(move |v| {
            if let Some(c) = weak.upgrade() {
                c.set_query(|q| q.platform = v.to_string());
            }
        });
        let weak = Rc::downgrade(this);
        this.window.on_tag_filter_changed(move |v| {
            if let Some(c) = weak.upgrade() {
                c.set_query(|q| q.tag = v.to_string());
            }
        });
        let weak = Rc::downgrade(this);
        this.window.on_cycle_sort(move || {
            if let Some(c) = weak.upgrade() {
                c.cycle_sort();
            }
        });
        let weak = Rc::downgrade(this);
        this.window.on_theme_changed(move |v| {
            if let Some(c) = weak.upgrade() {
                c.set_theme(&v);
            }
        });
        let weak = Rc::downgrade(this);
        this.window.on_language_changed(move |v| {
            if let Some(c) = weak.upgrade() {
                c.set_language(&v);
            }
        });
        let weak = Rc::downgrade(this);
        this.window.on_browse_workspace(move || {
            if let Some(c) = weak.upgrade() {
                c.pick_workspace();
            }
        });
        let weak = Rc::downgrade(this);
        this.window.on_open_workspace(move || {
            if let Some(c) = weak.upgrade() {
                c.open_workspace();
            }
        });
        let weak = Rc::downgrade(this);
        this.window.on_default_language_changed(move |v| {
            if let Some(c) = weak.upgrade() {
                c.set_default("default_language", &v);
            }
        });
        let weak = Rc::downgrade(this);
        this.window.on_default_difficulty_changed(move |v| {
            if let Some(c) = weak.upgrade() {
                c.set_default("default_difficulty", &v);
            }
        });
        let weak = Rc::downgrade(this);
        this.window.on_open_problem_workspace(move || {
            if let Some(c) = weak.upgrade() {
                c.open_selected_workspace();
            }
        });
        let weak = Rc::downgrade(this);
        this.window.on_open_problem_url(move || {
            if let Some(c) = weak.upgrade() {
                c.open_selected_url();
            }
        });
        let weak = Rc::downgrade(this);
        this.window.on_problem_status_changed(move |v| {
            if let Some(c) = weak.upgrade() {
                c.change_selected_status(&v);
            }
        });
    }

    fn refresh_all(&self) -> Result<(), String> {
        self.refresh_strings()?;
        self.refresh_settings()?;
        self.refresh_data()
    }
    fn refresh_page(&self) -> Result<(), String> {
        match self.window.get_page().as_str() {
            "settings" => self.refresh_settings(),
            _ => self.refresh_data(),
        }
    }
    fn refresh_strings(&self) -> Result<(), String> {
        let l = self.state.language.lock().map_err(|e| e.to_string())?;
        self.window.set_strings(strings(&l));
        Ok(())
    }
    fn refresh_settings(&self) -> Result<(), String> {
        let conn = self.state.db.lock().map_err(|e| e.to_string())?;
        let workspace = workspace_path(&conn).map_err(|e| e.to_string())?;
        self.window
            .set_workspace_value(workspace.to_string_lossy().to_string().into());
        self.window.set_default_language_value(
            load_setting(&conn, "default_language")
                .unwrap_or_else(|| "rust".into())
                .into(),
        );
        self.window.set_default_difficulty_value(
            load_setting(&conn, "default_difficulty")
                .unwrap_or_else(|| "medium".into())
                .into(),
        );
        drop(conn);
        let language = self
            .state
            .language
            .lock()
            .map_err(|e| e.to_string())?
            .language()
            .locale();
        self.window.set_language_value(language.into());
        let theme = self.state.theme.lock().map_err(|e| e.to_string())?.clone();
        self.window.set_theme_value(theme.clone().into());
        self.apply_theme(&theme);
        Ok(())
    }
    fn refresh_data(&self) -> Result<(), String> {
        let conn = self.state.db.lock().map_err(|e| e.to_string())?;
        let dash = repository::dashboard(&conn).map_err(|e| e.to_string())?;
        let items =
            repository::list(&conn, self.query.borrow().clone()).map_err(|e| e.to_string())?;
        let recent = repository::recent(&conn, 100).map_err(|e| e.to_string())?;
        drop(conn);
        let labels = self.window.get_strings();
        self.window.set_total(dash.total as i32);
        self.window.set_solved(dash.solved as i32);
        self.window.set_in_progress(dash.in_progress as i32);
        self.window.set_not_started(dash.not_started as i32);
        self.window.set_problems(model(items, &labels));
        self.window.set_recent(model(recent, &labels));
        self.window.set_modified(model(dash.modified, &labels));
        self.apply_query_properties();
        Ok(())
    }
    fn apply_query_properties(&self) {
        let q = self.query.borrow();
        self.window.set_search(q.search.clone().into());
        self.window.set_status_filter(q.status.clone().into());
        self.window
            .set_difficulty_filter(q.difficulty.clone().into());
        self.window.set_language_filter(q.language.clone().into());
        self.window.set_platform_filter(q.platform.clone().into());
        self.window.set_tag_filter(q.tag.clone().into());
        self.window
            .set_sort_label(sort_label(&q.sort, &self.window.get_strings()).into());
    }
    fn set_query(&self, change: impl FnOnce(&mut ProblemQuery)) {
        change(&mut self.query.borrow_mut());
        if let Err(e) = self.refresh_data() {
            self.error(e)
        }
    }
    fn cycle_sort(&self) {
        let mut q = self.query.borrow_mut();
        q.sort = match q.sort.as_str() {
            "modified" => "created",
            "created" => "title",
            "title" => "difficulty",
            "difficulty" => "status",
            _ => "modified",
        }
        .into();
        drop(q);
        if let Err(e) = self.refresh_data() {
            self.error(e)
        }
    }
    fn open_new_form(&self) {
        let (lang, difficulty, workspace) = match self.state.db.lock() {
            Ok(c) => (
                load_setting(&c, "default_language").unwrap_or_else(|| "rust".into()),
                load_setting(&c, "default_difficulty").unwrap_or_else(|| "medium".into()),
                workspace_path(&c)
                    .ok()
                    .map(|p| p.to_string_lossy().into_owned())
                    .unwrap_or_default(),
            ),
            Err(_) => ("rust".into(), "medium".into(), String::new()),
        };
        self.window.set_form_editing(false);
        self.window.set_form_title("".into());
        self.window.set_form_identifier("".into());
        self.window.set_form_platform("".into());
        self.window.set_form_url("".into());
        self.window.set_form_tags("".into());
        self.window.set_form_description("".into());
        self.window.set_form_notes("".into());
        self.window.set_form_workspace(workspace.into());
        self.window.set_form_difficulty(difficulty.into());
        self.window.set_form_status("not_started".into());
        self.window.set_form_template("starter".into());
        self.set_languages(&[lang]);
        self.window.set_show_form(true);
    }
    fn open_edit_form(&self) {
        let p = self.window.get_selected();
        self.window.set_show_detail(false);
        self.window.set_form_editing(true);
        self.window.set_form_title(p.title);
        self.window.set_form_identifier(p.identifier);
        self.window.set_form_platform(p.platform);
        self.window.set_form_url(p.url);
        self.window.set_form_tags(p.tags);
        self.window.set_form_description(p.description);
        self.window.set_form_notes(p.notes);
        self.window.set_form_workspace(p.workspace_path);
        self.window.set_form_difficulty(p.difficulty);
        self.window.set_form_status(p.status);
        self.window.set_form_template("starter".into());
        self.set_languages(
            &p.languages
                .split(", ")
                .map(str::to_string)
                .collect::<Vec<_>>(),
        );
        self.window.set_show_form(true);
    }
    fn set_languages(&self, languages: &[String]) {
        self.window.set_lang_c(languages.iter().any(|v| v == "c"));
        self.window
            .set_lang_cpp(languages.iter().any(|v| v == "cpp"));
        self.window
            .set_lang_rust(languages.iter().any(|v| v == "rust"));
        self.window
            .set_lang_python(languages.iter().any(|v| v == "python"));
        self.window
            .set_lang_java(languages.iter().any(|v| v == "java"));
    }
    fn submit_form(&self) {
        let languages = [
            ("c", self.window.get_lang_c()),
            ("cpp", self.window.get_lang_cpp()),
            ("rust", self.window.get_lang_rust()),
            ("python", self.window.get_lang_python()),
            ("java", self.window.get_lang_java()),
        ]
        .into_iter()
        .filter(|(_, yes)| *yes)
        .map(|(v, _)| v.into())
        .collect();
        let input = ProblemInput {
            identifier: self.window.get_form_identifier().to_string(),
            title: self.window.get_form_title().to_string(),
            description: self.window.get_form_description().to_string(),
            platform: self.window.get_form_platform().to_string(),
            url: self.window.get_form_url().to_string(),
            difficulty: self.window.get_form_difficulty().to_string(),
            tags: self
                .window
                .get_form_tags()
                .split(',')
                .map(str::trim)
                .filter(|v| !v.is_empty())
                .map(str::to_string)
                .collect(),
            status: self.window.get_form_status().to_string(),
            languages,
            workspace_path: Some(self.window.get_form_workspace().to_string()),
            notes: self.window.get_form_notes().to_string(),
            template: self.window.get_form_template().to_string(),
        };
        let result = if self.window.get_form_editing() {
            self.update_selected(input)
        } else {
            self.create(input)
        };
        match result {
            Ok(message) => {
                self.window.set_show_form(false);
                self.notice(message, false);
                let _ = self.refresh_data();
            }
            Err(e) => self.error(e),
        }
    }
    fn create(&self, input: ProblemInput) -> Result<String, String> {
        let input = input.normalized().map_err(|e| e.to_string())?;
        let conn = self.state.db.lock().map_err(|e| e.to_string())?;
        repository::validate_new(&conn, &input).map_err(|e| e.to_string())?;
        let root = workspace::ensure_workspace_root(
            PathBuf::from(input.workspace_path.as_ref().unwrap()).as_path(),
        )
        .map_err(|e| e.to_string())?;
        let manifest = workspace::ProblemManifest {
            id: None,
            identifier: &input.identifier,
            title: &input.title,
            platform: &input.platform,
            url: &input.url,
            difficulty: &input.difficulty,
            tags: &input.tags,
            status: &input.status,
            languages: &input.languages,
            description: &input.description,
            notes: &input.notes,
        };
        let location = workspace::create_problem_workspace(
            &root,
            &input.title,
            &input.languages,
            &input.template,
            &manifest,
        )
        .map_err(|e| e.to_string())?;
        let p = repository::create(&conn, input, &location.to_string_lossy())
            .map_err(|e| e.to_string())?;
        let _ = write_problem_manifest(&location, &repository::manifest(&p));
        Ok(self.window.get_strings().message_created.to_string())
    }
    fn update_selected(&self, input: ProblemInput) -> Result<String, String> {
        let id = (*self.selected_id.borrow()).ok_or_else(|| "No problem selected".to_string())?;
        let input = input.normalized().map_err(|e| e.to_string())?;
        let conn = self.state.db.lock().map_err(|e| e.to_string())?;
        let p = repository::update(&conn, id, input).map_err(|e| e.to_string())?;
        let _ = write_problem_manifest(
            PathBuf::from(&p.workspace_path).as_path(),
            &repository::manifest(&p),
        );
        self.window
            .set_selected(to_item(&p, &self.window.get_strings()));
        Ok(self.window.get_strings().message_updated.to_string())
    }
    fn open_detail(&self, id: i32) {
        let result = (|| -> Result<(), String> {
            let conn = self.state.db.lock().map_err(|e| e.to_string())?;
            let p = repository::record_open(&conn, id as i64).map_err(|e| e.to_string())?;
            let detail = repository::detail(&conn, id as i64).map_err(|e| e.to_string())?;
            self.window
                .set_selected(to_item(&p, &self.window.get_strings()));
            self.window.set_selected_source_files(
                detail
                    .source_files
                    .iter()
                    .map(|f| f.path.as_str())
                    .collect::<Vec<_>>()
                    .join("\n")
                    .into(),
            );
            *self.selected_id.borrow_mut() = Some(id as i64);
            self.window.set_show_detail(true);
            Ok(())
        })();
        if let Err(e) = result {
            self.error(e)
        }
    }
    fn delete_selected(&self) {
        let result = (|| -> Result<(), String> {
            let id =
                (*self.selected_id.borrow()).ok_or_else(|| "No problem selected".to_string())?;
            let conn = self.state.db.lock().map_err(|e| e.to_string())?;
            repository::delete(&conn, id).map_err(|e| e.to_string())?;
            Ok(())
        })();
        match result {
            Ok(()) => {
                self.window.set_show_confirm_delete(false);
                self.window.set_show_detail(false);
                self.notice(self.window.get_strings().message_deleted.to_string(), false);
                let _ = self.refresh_data();
            }
            Err(e) => self.error(e),
        }
    }
    fn clear_recent(&self) {
        let result = self
            .state
            .db
            .lock()
            .map_err(|e| e.to_string())
            .and_then(|c| repository::clear_recent(&c).map_err(|e| e.to_string()));
        match result {
            Ok(()) => {
                self.window.set_show_confirm_clear(false);
                let _ = self.refresh_data();
            }
            Err(e) => self.error(e),
        }
    }
    fn set_theme(&self, value: &str) {
        let value = if ["light", "dark", "system"].contains(&value) {
            value
        } else {
            "system"
        };
        let result = (|| -> Result<(), String> {
            *self.state.theme.lock().map_err(|e| e.to_string())? = value.into();
            let c = self.state.db.lock().map_err(|e| e.to_string())?;
            save_setting(&c, "theme", value).map_err(|e| e.to_string())?;
            Ok(())
        })();
        match result {
            Ok(()) => {
                self.window.set_theme_value(value.into());
                self.apply_theme(value)
            }
            Err(e) => self.error(e),
        }
    }
    fn apply_theme(&self, value: &str) {
        let dark = match value {
            "dark" => true,
            "light" => false,
            _ => matches!(dark_light::detect(), Ok(dark_light::Mode::Dark)),
        };
        self.window.global::<crate::Theme>().set_dark(dark);
    }
    fn set_language(&self, value: &str) {
        let language = match value {
            "en" => AppLanguage::English,
            "vi" => AppLanguage::Vietnamese,
            _ => AppLanguage::System,
        };
        let result = (|| -> Result<(), String> {
            self.state
                .language
                .lock()
                .map_err(|e| e.to_string())?
                .set_language(language);
            let c = self.state.db.lock().map_err(|e| e.to_string())?;
            save_setting(&c, "language", language.locale()).map_err(|e| e.to_string())?;
            Ok(())
        })();
        match result {
            Ok(()) => {
                let _ = self.refresh_strings();
                self.window.set_language_value(language.locale().into());
                let _ = self.refresh_data();
            }
            Err(e) => self.error(e),
        }
    }
    fn pick_workspace(&self) {
        let start = self.window.get_workspace_value().to_string();
        if let Some(path) = rfd::FileDialog::new().set_directory(start).pick_folder() {
            let path = path.to_string_lossy().into_owned();
            if self.window.get_show_form() {
                self.window.set_form_workspace(path.into());
                return;
            }
            let result = (|| -> Result<(), String> {
                let root = workspace::ensure_workspace_root(PathBuf::from(&path).as_path())
                    .map_err(|e| e.to_string())?;
                let c = self.state.db.lock().map_err(|e| e.to_string())?;
                save_setting(&c, "workspace_path", &root.to_string_lossy())
                    .map_err(|e| e.to_string())?;
                Ok(())
            })();
            match result {
                Ok(()) => {
                    let _ = self.refresh_settings();
                    self.notice(
                        self.window
                            .get_strings()
                            .message_workspace_updated
                            .to_string(),
                        false,
                    )
                }
                Err(e) => self.error(e),
            }
        }
    }
    fn open_workspace(&self) {
        let result = self
            .state
            .db
            .lock()
            .map_err(|e| e.to_string())
            .and_then(|c| workspace_path(&c).map_err(|e| e.to_string()))
            .and_then(|p| platform::open_path(&p));
        if let Err(e) = result {
            self.error(e)
        }
    }
    fn set_default(&self, key: &str, value: &str) {
        let result = self
            .state
            .db
            .lock()
            .map_err(|e| e.to_string())
            .and_then(|c| save_setting(&c, key, value).map_err(|e| e.to_string()));
        match result {
            Ok(()) => {
                let _ = self.refresh_settings();
            }
            Err(e) => self.error(e),
        }
    }
    fn open_selected_workspace(&self) {
        if let Err(e) = platform::open_path(
            PathBuf::from(self.window.get_selected().workspace_path.as_str()).as_path(),
        ) {
            self.error(e)
        }
    }
    fn open_selected_url(&self) {
        let url = self.window.get_selected().url.to_string();
        if url.is_empty() {
            self.error("This problem does not have a source URL".into())
        } else if let Err(e) = platform::open_url(&url) {
            self.error(e)
        }
    }
    fn change_selected_status(&self, status: &str) {
        let p = self.window.get_selected();
        let input = ProblemInput {
            identifier: p.identifier.to_string(),
            title: p.title.to_string(),
            description: p.description.to_string(),
            platform: p.platform.to_string(),
            url: p.url.to_string(),
            difficulty: p.difficulty.to_string(),
            tags: p
                .tags
                .split(", ")
                .filter(|v| !v.is_empty())
                .map(str::to_string)
                .collect(),
            status: status.into(),
            languages: p
                .languages
                .split(", ")
                .filter(|v| !v.is_empty())
                .map(str::to_string)
                .collect(),
            workspace_path: None,
            notes: p.notes.to_string(),
            template: "starter".into(),
        };
        match self.update_selected(input) {
            Ok(message) => {
                self.notice(message, false);
                let _ = self.refresh_data();
            }
            Err(e) => self.error(e),
        }
    }
    fn notice(&self, text: String, error: bool) {
        self.window.set_notice(text.into());
        self.window.set_notice_error(error)
    }
    fn error(&self, error: String) {
        self.notice(error, true)
    }
}

fn model(items: Vec<Problem>, labels: &UiStrings) -> ModelRc<ProblemItem> {
    ModelRc::new(Rc::new(VecModel::from(
        items
            .iter()
            .map(|item| to_item(item, labels))
            .collect::<Vec<_>>(),
    )))
}
fn to_item(p: &Problem, labels: &UiStrings) -> ProblemItem {
    ProblemItem {
        id: p.id as i32,
        identifier: p.identifier.clone().into(),
        title: p.title.clone().into(),
        description: p.description.clone().into(),
        platform: p.platform.clone().into(),
        url: p.url.clone().into(),
        difficulty: p.difficulty.clone().into(),
        difficulty_label: difficulty_label(&p.difficulty, labels).into(),
        tags: p.tags.join(", ").into(),
        status: p.status.clone().into(),
        status_label: status_label(&p.status, labels).into(),
        languages: p.languages.join(", ").into(),
        workspace_path: p.workspace_path.clone().into(),
        notes: p.notes.clone().into(),
        created_at: short_date(&p.created_at).into(),
        updated_at: short_date(&p.updated_at).into(),
        last_opened_at: p
            .last_opened_at
            .as_deref()
            .map(short_date)
            .unwrap_or_default()
            .into(),
    }
}
fn short_date(value: &str) -> String {
    value
        .replace('T', " ")
        .split('.')
        .next()
        .unwrap_or(value)
        .trim_end_matches('Z')
        .to_string()
}
fn sort_label(sort: &str, s: &UiStrings) -> String {
    match sort {
        "created" => s.created.to_string(),
        "title" => s.title.to_string(),
        "difficulty" => s.difficulty.to_string(),
        "status" => s.status.to_string(),
        _ => s.modified.to_string(),
    }
}
fn status_label(value: &str, s: &UiStrings) -> String {
    match value {
        "in_progress" => s.status_in_progress.to_string(),
        "solved" => s.status_solved.to_string(),
        "review" => s.status_review.to_string(),
        _ => s.status_not_started.to_string(),
    }
}
fn difficulty_label(value: &str, s: &UiStrings) -> String {
    match value {
        "easy" => s.difficulty_easy.to_string(),
        "hard" => s.difficulty_hard.to_string(),
        "custom" => s.difficulty_custom.to_string(),
        _ => s.difficulty_medium.to_string(),
    }
}
fn t(manager: &LanguageManager, key: &str) -> SharedString {
    manager.translate(key).into()
}
fn strings(m: &LanguageManager) -> UiStrings {
    UiStrings {
        app_name: t(m, "app-name"),
        nav_home: t(m, "navigation-home"),
        nav_problems: t(m, "navigation-problems"),
        nav_recent: t(m, "navigation-recent"),
        nav_settings: t(m, "navigation-settings"),
        home_title: t(m, "home-title"),
        home_overview: t(m, "home-overview"),
        home_empty_title: t(m, "home-empty-title"),
        home_empty_description: t(m, "home-empty-description"),
        home_total: t(m, "home-total-problems"),
        home_solved: t(m, "home-solved"),
        home_progress: t(m, "home-in-progress"),
        home_not_started: t(m, "home-not-started"),
        home_completion: t(m, "home-completion"),
        home_quick_actions: t(m, "home-quick-actions"),
        home_recent: t(m, "home-recent-section"),
        home_modified: t(m, "home-modified-section"),
        problems_title: t(m, "problems-title"),
        problems_empty_title: t(m, "problems-empty-title"),
        problems_empty_description: t(m, "problems-empty-description"),
        no_results_title: t(m, "problems-no-results-title"),
        no_results_description: t(m, "problems-no-results-description"),
        search_placeholder: t(m, "problems-search-placeholder"),
        filter_status: t(m, "problems-filter-status"),
        filter_difficulty: t(m, "problems-filter-difficulty"),
        filter_language: t(m, "problems-filter-language"),
        filter_platform: t(m, "problems-filter-platform"),
        filter_tags: t(m, "problems-filter-tag"),
        sort: t(m, "problems-sort"),
        recent_title: t(m, "recent-title"),
        recent_empty_title: t(m, "recent-empty-title"),
        recent_empty_description: t(m, "recent-empty-description"),
        clear_history: t(m, "recent-clear"),
        settings_title: t(m, "settings-title"),
        appearance: t(m, "settings-appearance"),
        theme: t(m, "settings-theme"),
        language: t(m, "settings-language"),
        workspace: t(m, "settings-workspace"),
        defaults: t(m, "settings-problem-defaults"),
        workspace_path: t(m, "settings-workspace-path"),
        default_language: t(m, "settings-default-language"),
        default_difficulty: t(m, "settings-default-difficulty"),
        new_problem: t(m, "common-new"),
        edit_problem: t(m, "dialog-edit-problem-title"),
        create: t(m, "common-create"),
        save: t(m, "common-save"),
        cancel: t(m, "common-cancel"),
        close: t(m, "common-close"),
        delete: t(m, "common-delete"),
        edit: t(m, "common-edit"),
        open: t(m, "common-open"),
        browse: t(m, "common-browse"),
        open_workspace: t(m, "common-open-workspace"),
        open_url: t(m, "common-open-url"),
        title: t(m, "dialog-problem-title"),
        identifier: t(m, "dialog-problem-identifier"),
        platform: t(m, "dialog-problem-platform"),
        url: t(m, "dialog-problem-url"),
        difficulty: t(m, "dialog-problem-difficulty"),
        status: t(m, "dialog-problem-status"),
        tags: t(m, "dialog-problem-tags"),
        description: t(m, "dialog-problem-description"),
        notes: t(m, "dialog-problem-notes"),
        languages: t(m, "dialog-problem-language"),
        template: t(m, "dialog-problem-template"),
        starter: t(m, "template-starter"),
        empty: t(m, "template-empty"),
        details: t(m, "detail-title"),
        source_files: t(m, "detail-source-files"),
        created: t(m, "detail-created"),
        modified: t(m, "detail-modified"),
        last_opened: t(m, "detail-last-opened"),
        confirm_delete: t(m, "dialog-confirm-delete-title"),
        confirm_delete_body: t(m, "dialog-confirm-delete-body"),
        confirm_clear: t(m, "dialog-confirm-clear-recent-title"),
        confirm_clear_body: t(m, "dialog-confirm-clear-recent-body"),
        status_not_started: t(m, "status-not-started"),
        status_in_progress: t(m, "status-in-progress"),
        status_solved: t(m, "status-solved"),
        status_review: t(m, "status-review"),
        difficulty_easy: t(m, "difficulty-easy"),
        difficulty_medium: t(m, "difficulty-medium"),
        difficulty_hard: t(m, "difficulty-hard"),
        difficulty_custom: t(m, "difficulty-custom"),
        message_created: t(m, "message-problem-created"),
        message_updated: t(m, "message-problem-updated"),
        message_deleted: t(m, "message-problem-deleted"),
        message_workspace_updated: t(m, "message-workspace-updated"),
        error: t(m, "common-error"),
        system: t(m, "settings-theme-system"),
        light: t(m, "settings-theme-light"),
        dark: t(m, "settings-theme-dark"),
        all: t(m, "problems-filter-all"),
    }
}
