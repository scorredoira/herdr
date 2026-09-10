use super::App;

impl App {
    pub(crate) fn set_host_terminal_appearance_state(
        &mut self,
        appearance: Option<crate::terminal_theme::HostAppearance>,
        explicit: bool,
    ) -> bool {
        if self.state.host_terminal_appearance == appearance
            && self.state.host_terminal_appearance_explicit == explicit
        {
            return false;
        }
        self.state.host_terminal_appearance = appearance;
        self.state.host_terminal_appearance_explicit = explicit;
        // The theme may follow the host (auto_switch), and the panes follow the theme.
        self.refresh_effective_app_theme();
        self.apply_host_terminal_appearance_to_panes();
        true
    }

    pub(crate) fn set_host_terminal_theme(
        &mut self,
        theme: crate::terminal_theme::TerminalTheme,
    ) -> bool {
        if theme == self.state.host_terminal_theme {
            return false;
        }
        self.state.host_terminal_theme = theme;
        self.apply_host_terminal_theme_to_panes();
        true
    }

    pub(super) fn refresh_effective_app_theme(&mut self) -> bool {
        let (palette, theme_name) = super::resolve_effective_theme(
            &self.state.theme_runtime,
            self.state.host_terminal_appearance,
        );
        if self.state.theme_name == theme_name && self.state.palette == palette {
            return false;
        }
        self.state.theme_name = theme_name;
        self.state.palette = palette;
        self.apply_host_terminal_appearance_to_panes();
        self.render_dirty.request_generic();
        self.render_notify.notify_one();
        true
    }

    fn apply_host_terminal_appearance_to_panes(&self) {
        let appearance = self.state.pane_appearance();
        for runtime in self.terminal_runtimes.values() {
            runtime.apply_host_terminal_appearance(appearance);
        }
    }

    fn apply_host_terminal_theme_to_panes(&self) {
        for runtime in self.terminal_runtimes.values() {
            runtime.apply_host_terminal_theme(self.state.host_terminal_theme);
        }

        self.render_dirty.request_generic();
        self.render_notify.notify_one();
    }
}
