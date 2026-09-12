use ratatui::widgets::ListState;

use crate::config::{Category, Project};
use crate::editor::{self};

#[derive(Debug, Default)]
pub enum EditMode {
    #[default]
    ProjectView,
    EditFilter,
    EditingProject(editor::ProjectEditor),
    CreatingProject(editor::ProjectEditor),
}

#[derive(Clone, Copy)]
pub enum Entry {
    Header(usize),
    Project { category: usize, index: usize },
}

#[derive(Debug)]
pub struct ProjectListView {
    pub categories: Vec<Category>,
    pub state: ListState,
    pub mode: EditMode,
    pub filter: String,
}

impl ProjectListView {
    pub fn new(projects: Vec<Category>) -> Self {
        ProjectListView {
            state: ListState::default(),
            mode: EditMode::default(),
            filter: String::new(),
            categories: projects,
        }
    }
    pub fn entries(&self) -> Vec<Entry> {
        let mut out = Vec::new();
        for (ci, cat) in self.categories.iter().enumerate() {
            let hits: Vec<(usize, &Project)> = cat
                .projects
                .iter()
                .enumerate()
                .filter(|(_, p)| self.matches_filter(p))
                .collect();
            if hits.is_empty() {
                continue;
            }
            out.push(Entry::Header(ci));
            out.extend(hits.into_iter().map(|(pi, _)| Entry::Project {
                category: ci,
                index: pi,
            }));
        }
        out
    }

    fn step(&mut self, direction: isize) {
        let entries = self.entries();
        let mut i = self.state.selected().unwrap_or(0) as isize;
        loop {
            i += direction;
            if i < 0 || i >= entries.len() as isize {
                return;
            }
            if matches!(entries[i as usize], Entry::Project { .. }) {
                self.state.select(Some(i as usize));
                return;
            }
        }
    }
    pub fn next(&mut self) {
        self.step(1);
    }
    pub fn prev(&mut self) {
        self.step(-1);
    }

    fn matches_filter(&self, project: &Project) -> bool {
        self.filter.is_empty() || project.name.contains(&self.filter)
    }

    pub fn get_from_rendered(&self) -> Option<(&Category, &Project)> {
        match self.entries().get(self.state.selected()?)? {
            Entry::Project { category, index } => Some((
                &self.categories[*category],
                &self.categories[*category].projects[*index],
            )),
            Entry::Header(_) => None,
        }
    }

    pub fn insert(&mut self, project: Project, category: String) {
        if let Some(cat) = self.categories.iter_mut().find(|c| c.name == category) {
            cat.projects.push(project);
        } else {
            self.categories.push(Category {
                name: category,
                projects: vec![project],
            })
        }
    }

    pub fn update_selected(&mut self, project: Project, category: String) {
        self.del_selected();
        self.insert(project, category);
    }
    pub fn del_selected(&mut self) -> Option<()> {
        if let Entry::Project { category, index } = self.entries().get(self.state.selected()?)? {
            self.categories[*category].projects.remove(*index);
            Some(())
        } else {
            None
        }
    }
}
