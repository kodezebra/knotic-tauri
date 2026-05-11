<script lang="ts">
  import type { FileEntry } from '$lib/state/workspace.svelte';
  import { editor } from '$lib/state/editor.svelte';
  import { ui } from '$lib/state/ui.svelte';
  import { Folder, FileText, ChevronRight, ChevronDown, MoreVertical, Edit2, Trash2, ExternalLink, Plus, FolderPlus } from 'lucide-svelte';
  import FileTreeSelf from './FileTree.svelte';
  import { ContextMenu } from 'bits-ui';

  let {
    items = [],
    onRename = (_path: string, _name: string) => {},
    onDelete = (_path: string, _name: string, _isDir: boolean) => {},
    onNewFile = (_parentPath: string) => {},
    onNewFolder = (_parentPath: string) => {},
  } = $props<{
    items: FileEntry[];
    onRename?: (path: string, name: string) => void;
    onDelete?: (path: string, name: string, isDir: boolean) => void;
    onNewFile?: (parentPath: string) => void;
    onNewFolder?: (parentPath: string) => void;
  }>();

  let expanded = $state<Record<string, boolean>>({});

  function toggle(path: string) {
    expanded[path] = !expanded[path];
  }

  function handleFileClick(item: FileEntry) {
    if (item.is_dir) {
      toggle(item.path);
    } else {
      editor.openFile(item.path, item.name);
    }
  }
</script>

<ul class="flex flex-col" class:gap-0.5={ui.viewMode === 'workspace'} class:gap-1={ui.viewMode === 'preview'}>
  {#each items as item (item.path)}
    <li>
      <ContextMenu.Root>
        <ContextMenu.Trigger>
          <button
            class="w-full flex items-center gap-1.5 px-2 rounded transition-all cursor-pointer group {ui.viewMode === 'workspace' ? 'py-1 text-sm' : 'py-2 text-[13px]'} {ui.viewMode === 'workspace' || ui.theme === 'dark' ? 'hover:bg-white/5' : ''} {ui.viewMode === 'preview' && ui.theme === 'light' ? 'hover:bg-black/5' : ''} {editor.activePath !== item.path ? 'text-text-primary' : 'text-brand-primary'} {editor.activePath === item.path && ui.viewMode === 'preview' ? 'font-medium' : ''}"
            onclick={() => handleFileClick(item)}
          >
            {#if ui.viewMode === 'workspace'}
              <span class="w-4 flex items-center justify-center">
                {#if item.is_dir}
                  {#if expanded[item.path]}
                    <ChevronDown size={14} class="text-text-muted" />
                  {:else}
                    <ChevronRight size={14} class="text-text-muted" />
                  {/if}
                {/if}
              </span>

              {#if item.is_dir}
                <Folder size={16} class="text-brand-primary/80" />
              {:else}
                <FileText size={16} class="text-text-muted" />
              {/if}
            {:else if ui.viewMode === 'preview'}
              {#if !item.is_dir}
                <div class="w-1 h-1 rounded-full bg-slate-400 group-hover:bg-brand-primary transition-colors" class:bg-brand-primary={editor.activePath === item.path}></div>
              {/if}
            {/if}

            <span class="truncate flex-1 text-left">{item.name}</span>

            {#if ui.viewMode === 'workspace'}
              <MoreVertical size={14} class="text-text-muted opacity-0 group-hover:opacity-100 transition-opacity" />
            {/if}
          </button>
        </ContextMenu.Trigger>

        {#if item.is_dir}
          <ContextMenu.Content class="z-50 min-w-[180px] bg-bg-sidebar border border-border-subtle p-1 rounded-md shadow-xl text-text-primary">
            <ContextMenu.Item
              class="flex items-center gap-2 px-2 py-1.5 text-xs rounded hover:bg-brand-primary hover:text-white cursor-pointer transition-colors"
              onclick={() => onNewFile(item.path)}
            >
              <Plus size={14} />
              New File
            </ContextMenu.Item>
            <ContextMenu.Item
              class="flex items-center gap-2 px-2 py-1.5 text-xs rounded hover:bg-brand-primary hover:text-white cursor-pointer transition-colors"
              onclick={() => onNewFolder(item.path)}
            >
              <FolderPlus size={14} />
              New Folder
            </ContextMenu.Item>
            <ContextMenu.Separator class="h-px bg-border-subtle my-1" />
            <ContextMenu.Item
              class="flex items-center gap-2 px-2 py-1.5 text-xs rounded hover:bg-brand-primary hover:text-white cursor-pointer transition-colors"
              onclick={() => handleFileClick(item)}
            >
              <ExternalLink size={14} />
              Open
            </ContextMenu.Item>
            <ContextMenu.Item
              class="flex items-center gap-2 px-2 py-1.5 text-xs rounded hover:bg-brand-primary hover:text-white cursor-pointer transition-colors"
              onclick={() => onRename(item.path, item.name)}
            >
              <Edit2 size={14} />
              Rename
            </ContextMenu.Item>
            <ContextMenu.Separator class="h-px bg-border-subtle my-1" />
            <ContextMenu.Item
              class="flex items-center gap-2 px-2 py-1.5 text-xs rounded hover:bg-red-500 hover:text-white cursor-pointer transition-colors"
              onclick={() => onDelete(item.path, item.name, item.is_dir)}
            >
              <Trash2 size={14} />
              Delete
            </ContextMenu.Item>
          </ContextMenu.Content>
        {:else}
          <ContextMenu.Content class="z-50 min-w-[160px] bg-bg-sidebar border border-border-subtle p-1 rounded-md shadow-xl text-text-primary">
            <ContextMenu.Item
              class="flex items-center gap-2 px-2 py-1.5 text-xs rounded hover:bg-brand-primary hover:text-white cursor-pointer transition-colors"
              onclick={() => handleFileClick(item)}
            >
              <ExternalLink size={14} />
              Open
            </ContextMenu.Item>
            <ContextMenu.Item
              class="flex items-center gap-2 px-2 py-1.5 text-xs rounded hover:bg-brand-primary hover:text-white cursor-pointer transition-colors"
              onclick={() => onRename(item.path, item.name)}
            >
              <Edit2 size={14} />
              Rename
            </ContextMenu.Item>
            <ContextMenu.Separator class="h-px bg-border-subtle my-1" />
            <ContextMenu.Item
              class="flex items-center gap-2 px-2 py-1.5 text-xs rounded hover:bg-red-500 hover:text-white cursor-pointer transition-colors"
              onclick={() => onDelete(item.path, item.name, item.is_dir)}
            >
              <Trash2 size={14} />
              Delete
            </ContextMenu.Item>
          </ContextMenu.Content>
        {/if}
      </ContextMenu.Root>

      {#if item.is_dir && (expanded[item.path] || ui.viewMode === 'preview') && item.children}
        <div class="ml-4 border-l border-border-subtle pl-1">
          <FileTreeSelf items={item.children} {onRename} {onDelete} {onNewFile} {onNewFolder} />
        </div>
      {/if}
    </li>
  {/each}
</ul>
