use ratatui::layout::Rect;

/// A clickable region in the TUI with an associated action identifier.
#[derive(Debug, Clone)]
pub struct ClickRegion {
    pub area: Rect,
    pub action: ClickAction,
}

/// Actions that can be triggered by mouse clicks.
#[derive(Debug, Clone, PartialEq)]
pub enum ClickAction {
    ScrollUp,
    ScrollDown,
    TableRow(usize),
    #[allow(dead_code)]
    HeaderTab(HeaderTab),
    ScrollbarTrack(ScrollDirection),
}

/// Scroll direction for scrollbar track clicks.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ScrollDirection {
    Up,
    Down,
}

/// Header tabs that can be clicked.
#[derive(Debug, Clone, Copy, PartialEq)]
#[allow(dead_code)]
pub enum HeaderTab {
    SymbolList,
    StatsDashboard,
}

/// Collection of clickable regions for hit testing.
#[derive(Debug, Default)]
pub struct ClickRegions {
    pub regions: Vec<ClickRegion>,
}

impl ClickRegions {
    pub fn new() -> Self {
        Self {
            regions: Vec::new(),
        }
    }

    pub fn add(&mut self, area: Rect, action: ClickAction) {
        self.regions.push(ClickRegion { area, action });
    }

    /// Find the action for a mouse click at (x, y).
    /// Returns the first matching region (topmost in rendering order).
    pub fn hit_test(&self, x: u16, y: u16) -> Option<&ClickAction> {
        self.regions
            .iter()
            .find(|region| {
                x >= region.area.x
                    && x < region.area.x + region.area.width
                    && y >= region.area.y
                    && y < region.area.y + region.area.height
            })
            .map(|region| &region.action)
    }
}
