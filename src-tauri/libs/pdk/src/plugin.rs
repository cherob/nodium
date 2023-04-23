impl NodiumPlugin for NodiumPluginBrowser {
    fn name(&self) -> String {
        "nodium_plugin_browser".to_string()
    }

    fn with_events(&mut self) {
        self.events = Some(Arc::new(Mutex::new(events)));
    }

    // will create a browser window
    fn windows(&self) -> Vec<Box<dyn NodiumWindow>> {
        let events = self.events.as_ref().unwrap().clone();
        let crates_service = Arc::new(CratesService::new(events));
        vec![Box::new(CratesBrowserWindow::new(crates_service))]
    }

    fn nodes(&self) -> Vec<NodiumNode> {
        vec![]
    }

    fn services(&self) -> Vec<NodiumService> {
        vec![]
    }
}
