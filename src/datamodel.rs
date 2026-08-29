use ratatui::widgets::ListState;

use crate::editor::{self};
use crate::persistence::{Category, Project, ProjectList};

#[derive(Debug)]
pub enum EditMode {
    ProjectView,
    EditFilter,
    EditingProject(editor::ProjectEditor),
    CreatingProject(editor::ProjectEditor),
}

impl Default for EditMode {
    fn default() -> Self {
        Self::ProjectView
    }
}

#[derive(Clone, Copy)]
pub enum Entry {
    Header(usize),
    Project { category: usize, index: usize },
}

#[derive(Debug)]
pub struct ProjectListView {
    pub projects: ProjectList,
    pub state: ListState,
    pub mode: EditMode,
    pub filter: String,
}

impl ProjectListView {
    pub fn entries(&self) -> Vec<Entry> {
        let mut out = Vec::new();
        for (ci, cat) in self.projects.categories.iter().enumerate() {
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

    fn step(&mut self, dir: isize) {
        let entries = self.entries();
        let mut i = self.state.selected().unwrap_or(0) as isize;
        loop {
            i += dir;
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
                &self.projects.categories[*category],
                &self.projects.categories[*category].projects[*index],
            )),
            Entry::Header(_) => None,
        }
    }

    pub fn insert(&mut self, project: Project, category: String) {
        if let Some(cat) = self
            .projects
            .categories
            .iter_mut()
            .find(|c| c.name == category)
        {
            cat.projects.push(project);
        } else {
            self.projects.categories.push(Category {
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
            self.projects.categories[*category].projects.remove(*index);
            Some(())
        } else {
            None
        }
    }
}
