import { invoke } from '@tauri-apps/api/core';

export interface FileEntry {
    name: string;
    path: string;
    is_dir: boolean;
    children?: FileEntry[];
}

export interface Workspace {
    name: string;
    path: string;
}

const WORKSPACE_STORAGE_KEY = 'knotic-last-workspace';

const HIDDEN_FILES = new Set(['knotic.json', 'node_modules', '.git']);

class WorkspaceState {
    current = $state<Workspace | null>(null);
    fileTree = $state<FileEntry[]>([]);
    isLoading = $state(false);
    error = $state<string | null>(null);

    async create(path: string, name: string) {
        this.isLoading = true;
        this.error = null;
        try {
            this.current = await invoke('create_workspace', { path, name, scaffold: true });
            this._saveToStorage();
            await this.refreshFileTree();
        } catch (e) {
            this.error = String(e);
        } finally {
            this.isLoading = false;
        }
    }

    async open(path: string) {
        this.isLoading = true;
        this.error = null;
        try {
            this.current = await invoke('open_workspace', { path });
        } catch {
            const folderName = path.split('/').filter(Boolean).pop() || path.split('\\').filter(Boolean).pop() || 'Workspace';
            this.current = await invoke('create_workspace', { path, name: folderName, scaffold: false });
        }
        if (this.current) {
            this._saveToStorage();
            await this.refreshFileTree();
        }
        this.isLoading = false;
    }

    async refreshFileTree() {
        if (!this.current) return;
        try {
            const tree: FileEntry[] = await invoke('list_files', { path: this.current.path });
            this.fileTree = this._filterHidden(tree);
        } catch (e) {
            this.error = String(e);
        }
    }

    async createFile(name: string, parentPath?: string): Promise<string | null> {
        if (!this.current) return null;
        const dir = parentPath || this.current.path;
        const filePath = `${dir}/${name}`;
        try {
            await invoke('write_file', { path: filePath, content: '' });
            await this.refreshFileTree();
            return filePath;
        } catch (e) {
            this.error = String(e);
            return null;
        }
    }

    async createFolder(name: string, parentPath?: string): Promise<string | null> {
        if (!this.current) return null;
        const dir = parentPath || this.current.path;
        const dirPath = `${dir}/${name}`;
        try {
            await invoke('create_dir', { path: dirPath });
            await this.refreshFileTree();
            return dirPath;
        } catch (e) {
            this.error = String(e);
            return null;
        }
    }

    async deleteEntry(path: string) {
        try {
            await invoke('delete_path', { path });
            await this.refreshFileTree();
        } catch (e) {
            this.error = String(e);
        }
    }

    async renameEntry(oldPath: string, newName: string): Promise<string | null> {
        const parent = oldPath.substring(0, oldPath.lastIndexOf('/'));
        const newPath = `${parent}/${newName}`;
        try {
            await invoke('rename_path', { from: oldPath, to: newPath });
            await this.refreshFileTree();
            return newPath;
        } catch (e) {
            this.error = String(e);
            return null;
        }
    }

    async initialize() {
        try {
            const saved = localStorage.getItem(WORKSPACE_STORAGE_KEY);
            if (saved) {
                const parsed = JSON.parse(saved);
                this.current = await invoke('open_workspace', { path: parsed.path });
                if (this.current) {
                    await this.refreshFileTree();
                    return;
                }
            }
            this.current = await invoke('get_current_workspace');
            if (this.current) {
                await this.refreshFileTree();
            }
        } catch (e) {
            console.error('Failed to initialize workspace:', e);
            localStorage.removeItem(WORKSPACE_STORAGE_KEY);
        }
    }

    async closeWorkspace() {
        this.current = null;
        this.fileTree = [];
        this.error = null;
        localStorage.removeItem(WORKSPACE_STORAGE_KEY);
    }

    private _saveToStorage() {
        if (this.current) {
            localStorage.setItem(WORKSPACE_STORAGE_KEY, JSON.stringify(this.current));
        }
    }

    private _filterHidden(entries: FileEntry[]): FileEntry[] {
        return entries
            .filter(e => !HIDDEN_FILES.has(e.name))
            .map(e => ({
                ...e,
                children: e.children ? this._filterHidden(e.children) : undefined,
            }));
    }
}

export const workspace = new WorkspaceState();
