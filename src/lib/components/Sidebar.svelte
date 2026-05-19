<script lang="ts">
  import { workspace } from '$lib/state/workspace.svelte';
  import { editor } from '$lib/state/editor.svelte';
  import { ui } from '$lib/state/ui.svelte';
  import { open } from '@tauri-apps/plugin-dialog';
  import { FolderOpen, Plus, Folder, FileText, Search, List } from 'lucide-svelte';
  import FileTree from './FileTree.svelte';
  import InputDialog from './InputDialog.svelte';
  import ConfirmDialog from './ConfirmDialog.svelte';

  let { onsearch = () => {} } = $props<{ onsearch?: () => void }>();

  let newFileDialogOpen = $state(false);
  let newFolderDialogOpen = $state(false);
  let renameDialogOpen = $state(false);
  let deleteConfirmOpen = $state(false);

  let renameTarget = $state<{ path: string; name: string } | null>(null);
  let deleteTarget = $state<{ path: string; name: string; isDir: boolean } | null>(null);
  let newFileTargetDir = $state<string | null>(null);
  let newFolderTargetDir = $state<string | null>(null);

  $effect(() => {
    if (ui.newFileCounter > 0) {
      promptNewFile();
    }
  });

  async function handleOpenWorkspace() {
    const selected = await open({
      directory: true,
      multiple: false,
    });
    if (selected) {
      await workspace.open(selected);
    }
  }

  function promptNewFile(parentPath?: string) {
    newFileTargetDir = parentPath ?? null;
    newFileDialogOpen = true;
  }

  function promptNewFolder(parentPath?: string) {
    newFolderTargetDir = parentPath ?? null;
    newFolderDialogOpen = true;
  }

  async function handleCreateFile(name: string) {
    const filePath = await workspace.createFile(name, newFileTargetDir ?? undefined);
    newFileTargetDir = null;
    if (filePath) {
      editor.openFile(filePath, name);
    }
  }

  async function handleCreateFolder(name: string) {
    await workspace.createFolder(name, newFolderTargetDir ?? undefined);
    newFolderTargetDir = null;
  }

  function promptRename(path: string, name: string) {
    renameTarget = { path, name };
    renameDialogOpen = true;
  }

  async function handleRename(newName: string) {
    if (!renameTarget) return;
    const oldPath = renameTarget.path;
    const oldName = renameTarget.name;
    const newPath = await workspace.renameEntry(oldPath, newName);
    if (newPath) {
      editor.updateTabPath(oldPath, newPath, newName);
    }
    renameTarget = null;
  }

  function promptDelete(path: string, name: string, isDir: boolean) {
    deleteTarget = { path, name, isDir };
    deleteConfirmOpen = true;
  }

  async function handleDelete() {
    if (!deleteTarget) return;
    editor.closeTabsForPath(deleteTarget.path);
    await workspace.deleteEntry(deleteTarget.path);
    deleteTarget = null;
  }
</script>

<div class="flex flex-col h-full" class:bg-white={ui.viewMode === 'preview' && ui.theme === 'light'} class:bg-bg-sidebar={ui.viewMode === 'preview' && ui.theme === 'dark'}>
  <div class="px-2 py-1.5 border-b border-border-subtle flex items-center justify-between group h-9">
    {#if ui.viewMode === 'workspace'}
      <h2 class="text-xs font-bold uppercase tracking-wider text-text-muted">Explorer</h2>
      <div class="flex items-center gap-1 opacity-0 group-hover:opacity-100 transition-opacity">
        <button class="p-1 hover:bg-white/10 rounded cursor-pointer" title="New File"
          onclick={() => promptNewFile()}>
          <Plus size={14} />
        </button>
        <button class="p-1 hover:bg-white/10 rounded cursor-pointer" title="New Folder"
          onclick={() => promptNewFolder()}>
          <Folder size={14} />
        </button>
        <button class="p-1 hover:bg-white/10 rounded cursor-pointer" title="Open Folder"
          onclick={handleOpenWorkspace}>
          <FolderOpen size={14} />
        </button>
      </div>
    {:else if ui.viewMode === 'preview'}
      <h2 class="text-xs font-bold uppercase tracking-wider text-text-muted flex items-center gap-2">
        <List size={14} />
        Navigation
      </h2>
      <div class="flex items-center gap-1">
        <button class="p-1 hover:bg-black/5 dark:hover:bg-white/10 rounded cursor-pointer text-text-muted" onclick={onsearch}>
          <Search size={14} />
        </button>
      </div>
    {/if}
  </div>

  <div class="flex-1 overflow-y-auto p-1">
    {#if workspace.error}
      <div class="mx-2 mb-2 p-2 bg-red-500/10 border border-red-500/30 rounded text-xs text-red-400">
        {workspace.error}
      </div>
    {/if}

    {#if workspace.isLoading}
      <div class="flex items-center justify-center h-full">
        <div class="w-5 h-5 border-2 border-brand-primary border-t-transparent rounded-full animate-spin"></div>
      </div>
    {:else if workspace.current}
      <div 
        class:prose={ui.viewMode === 'preview'} 
        class:prose-sm={ui.viewMode === 'preview'}
        class:prose-invert={ui.viewMode === 'preview' && ui.theme === 'dark'}
      >
        <FileTree
          items={workspace.fileTree}
          onNewFile={promptNewFile}
          onNewFolder={promptNewFolder}
          onRename={promptRename}
          onDelete={promptDelete}
        />
      </div>
    {:else}
      <div class="flex flex-col items-center justify-center h-full gap-4 p-4 text-center">
        <p class="text-sm text-text-muted">No workspace open</p>
        <button
          class="px-3 py-1.5 bg-brand-primary hover:bg-brand-primary/80 text-white rounded text-xs transition-colors"
          onclick={handleOpenWorkspace}
        >
          Open Folder
        </button>
      </div>
    {/if}
  </div>
</div>

<InputDialog
  bind:open={newFileDialogOpen}
  title="New File"
  label="File name"
  placeholder="my-note.md"
  submitLabel="Create"
  onSubmit={handleCreateFile}
/>

<InputDialog
  bind:open={newFolderDialogOpen}
  title="New Folder"
  label="Folder name"
  placeholder="my-topic"
  submitLabel="Create"
  onSubmit={handleCreateFolder}
/>

<InputDialog
  bind:open={renameDialogOpen}
  title="Rename"
  label="New name"
  initialValue={renameTarget?.name ?? ''}
  submitLabel="Rename"
  onSubmit={handleRename}
/>

<ConfirmDialog
  bind:open={deleteConfirmOpen}
  title={deleteTarget?.isDir ? 'Delete Folder' : 'Delete File'}
  message={deleteTarget ? `Are you sure you want to delete "${deleteTarget.name}"?` : ''}
  confirmLabel="Delete"
  variant="danger"
  onConfirm={handleDelete}
/>
