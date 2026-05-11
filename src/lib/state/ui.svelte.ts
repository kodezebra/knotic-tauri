const THEME_STORAGE_KEY = 'knotic-theme';

export type ViewMode = 'workspace' | 'focus' | 'preview';

class UIState {
    sidebarVisible = $state(true);
    activityBarWidth = $state(50);
    sidebarWidth = $state(260);
    previewVisible = $state(true);
    viewMode = $state<ViewMode>('workspace');
    theme = $state<'light' | 'dark'>('dark');
    newFileCounter = $state(0);
    windowWidth = $state(1200);

    isOverlaySidebar = $derived(this.windowWidth < 768);
    isSmall = $derived(this.windowWidth < 768);

    initResizeTracking() {
        this.windowWidth = window.innerWidth;
        const handler = () => this.windowWidth = window.innerWidth;
        window.addEventListener('resize', handler);
        return () => window.removeEventListener('resize', handler);
    }

    toggleSidebar() {
        this.sidebarVisible = !this.sidebarVisible;
    }

    togglePreview() {
        this.previewVisible = !this.previewVisible;
    }

    setViewMode(mode: ViewMode) {
        this.viewMode = mode;
        if (mode === 'focus') {
            this.sidebarVisible = false;
            this.previewVisible = false;
        } else if (mode === 'preview') {
            this.sidebarVisible = true;
            this.previewVisible = true;
        } else {
            this.sidebarVisible = true;
            this.previewVisible = true;
        }
    }

    requestNewFile() {
        this.newFileCounter++;
    }

    setTheme(theme: 'light' | 'dark') {
        this.theme = theme;
        if (theme === 'dark') {
            document.documentElement.classList.add('dark');
        } else {
            document.documentElement.classList.remove('dark');
        }
        localStorage.setItem(THEME_STORAGE_KEY, theme);
    }

    toggleTheme() {
        this.setTheme(this.theme === 'dark' ? 'light' : 'dark');
    }

    initTheme() {
        const saved = localStorage.getItem(THEME_STORAGE_KEY);
        if (saved === 'light' || saved === 'dark') {
            this.setTheme(saved);
        } else {
            this.setTheme('dark');
        }
    }
}

export const ui = new UIState();
