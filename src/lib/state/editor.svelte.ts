import { invoke } from '@tauri-apps/api/core';

export interface Tab {
    path: string;
    name: string;
    isDirty: boolean;
}

class EditorState {
    tabs = $state<Tab[]>([]);
    activePath = $state<string | null>(null);
    content = $state<string>('');
    activeTab = $derived(this.tabs.find(t => t.path === this.activePath) || null);

    private _autosaveTimer: ReturnType<typeof setTimeout> | null = null;
    private _autosaveDelay = 2000;

    async openFile(path: string, name: string) {
        const existingTab = this.tabs.find(t => t.path === path);
        if (!existingTab) {
            this.tabs.push({ path, name, isDirty: false });
        }

        this.activePath = path;
        this.content = await invoke('read_file', { path });
    }

    async closeTab(path: string) {
        const tab = this.tabs.find(t => t.path === path);
        if (tab?.isDirty && this.activePath === path) {
            await this.save();
        }

        const index = this.tabs.findIndex(t => t.path === path);
        if (index !== -1) {
            this.tabs.splice(index, 1);
            if (this.activePath === path) {
                this.activePath = this.tabs[this.tabs.length - 1]?.path || null;
                if (this.activePath) {
                    this.content = await invoke('read_file', { path: this.activePath });
                } else {
                    this.content = '';
                }
            }
        }
    }

    async save() {
        if (!this.activePath) return;
        try {
            await invoke('write_file', { path: this.activePath, content: this.content });
            const tab = this.tabs.find(t => t.path === this.activePath);
            if (tab) tab.isDirty = false;
        } catch (e) {
            console.error('Failed to save file:', e);
        }
    }

    async saveForPath(path: string) {
        const tab = this.tabs.find(t => t.path === path);
        if (!tab) return;
        try {
            const savedContent = await invoke('read_file', { path });
            await invoke('write_file', { path, content: savedContent });
            tab.isDirty = false;
        } catch (e) {
            console.error('Failed to save file:', e);
        }
    }

    updateContent(newContent: string) {
        this.content = newContent;
        const tab = this.tabs.find(t => t.path === this.activePath);
        if (tab) tab.isDirty = true;

        if (this._autosaveTimer) clearTimeout(this._autosaveTimer);
        this._autosaveTimer = setTimeout(() => {
            this.save();
        }, this._autosaveDelay);
    }

    closeTabsForPath(path: string) {
        const index = this.tabs.findIndex(t => t.path === path);
        if (index !== -1) {
            this.tabs.splice(index, 1);
            if (this.activePath === path) {
                this.activePath = this.tabs[this.tabs.length - 1]?.path || null;
                if (this.activePath) {
                    invoke<string>('read_file', { path: this.activePath }).then(c => this.content = c);
                } else {
                    this.content = '';
                }
            }
        }
    }

    updateTabPath(oldPath: string, newPath: string, newName: string) {
        const tab = this.tabs.find(t => t.path === oldPath);
        if (tab) {
            tab.path = newPath;
            tab.name = newName;
        }
        if (this.activePath === oldPath) {
            this.activePath = newPath;
        }
    }
}

export const editor = new EditorState();
