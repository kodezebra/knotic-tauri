<script lang="ts">
  import { ui } from '$lib/state/ui.svelte';
  import { editor } from '$lib/state/editor.svelte';
  import { workspace } from '$lib/state/workspace.svelte';
  import Sidebar from './Sidebar.svelte';
  import SearchDialog from './SearchDialog.svelte';
  import ShortcutsDialog from './ShortcutsDialog.svelte';
  import { onMount } from 'svelte';
  import { Files, PenLine, BookOpen, Settings, User, Search, Sun, Moon, Keyboard } from 'lucide-svelte';
  import { Tooltip } from 'bits-ui';

  let { children } = $props();

  let searchOpen = $state(false);
  let shortcutsOpen = $state(false);
  let isDragging = $state(false);

  function startResize(e: MouseEvent) {
    isDragging = true;
    e.preventDefault();

    const handleMouseMove = (e: MouseEvent) => {
      ui.setSidebarWidth(e.clientX - ui.activityBarWidth);
    };

    const handleMouseUp = () => {
      isDragging = false;
      window.removeEventListener('mousemove', handleMouseMove);
      window.removeEventListener('mouseup', handleMouseUp);
    };

    window.addEventListener('mousemove', handleMouseMove);
    window.addEventListener('mouseup', handleMouseUp);
  }

  onMount(() => {
    workspace.initialize();
    ui.initTheme();

    const handleKeyDown = (e: KeyboardEvent) => {
      const mod = e.metaKey || e.ctrlKey;

      if (mod && e.shiftKey && e.key === 'p') {
        e.preventDefault();
        ui.togglePreview();
        return;
      }

      if (mod && e.key === 'p') {
        e.preventDefault();
        searchOpen = !searchOpen;
        return;
      }

      if (mod && e.key === 'n') {
        e.preventDefault();
        ui.requestNewFile();
        return;
      }

      if (mod && e.key === 'w') {
        e.preventDefault();
        if (editor.activePath) editor.closeTab(editor.activePath);
        return;
      }

      if (mod && e.key === 'b') {
        e.preventDefault();
        ui.toggleSidebar();
        return;
      }

      if (mod && e.key === '/') {
        e.preventDefault();
        shortcutsOpen = !shortcutsOpen;
        return;
      }

      if (mod && e.key === 'Tab') {
        e.preventDefault();
        const tabs = editor.tabs;
        if (tabs.length < 2) return;
        const currentIndex = tabs.findIndex(t => t.path === editor.activePath);
        const nextIndex = e.shiftKey
          ? (currentIndex - 1 + tabs.length) % tabs.length
          : (currentIndex + 1) % tabs.length;
        const next = tabs[nextIndex];
        if (next) editor.openFile(next.path, next.name);
        return;
      }

      if (e.key === 'Escape' && ui.viewMode === 'focus') {
        ui.setViewMode('workspace');
      }
    };

    window.addEventListener('keydown', handleKeyDown);

    const handleFocus = () => workspace.refreshFileTree();
    window.addEventListener('focus', handleFocus);

    return () => {
      window.removeEventListener('keydown', handleKeyDown);
      window.removeEventListener('focus', handleFocus);
    };
  });
</script>

<div class="flex h-screen w-screen flex-col overflow-hidden bg-bg-editor selection:bg-brand-primary/30">
  <div class="flex flex-1 overflow-hidden" class:select-none={isDragging}>
    <!-- Activity Bar (VS Code Style) -->
    {#if ui.viewMode !== 'focus'}
      <div 
        class="flex flex-col items-center py-4 bg-bg-activity border-r border-border-subtle z-20"
        style="width: {ui.activityBarWidth}px"
      >
        <div class="flex flex-1 flex-col gap-4">
          <Tooltip.Provider>
            <Tooltip.Root delayDuration={200}>
              <Tooltip.Trigger 
                class="p-2 rounded-md transition-colors cursor-pointer group relative {ui.viewMode === 'workspace' ? 'text-brand-primary' : 'text-text-muted'}"
                onclick={() => ui.setViewMode('workspace')}
              >
                <Files size={24} strokeWidth={1.5} />
                {#if ui.viewMode === 'workspace'}
                  <div class="absolute left-0 top-1/2 -translate-y-1/2 w-0.5 h-6 bg-brand-primary rounded-r-full"></div>
                {/if}
              </Tooltip.Trigger>
              <Tooltip.Content 
                side="right" 
                class="z-50 px-3 py-1.5 text-xs bg-bg-activity border border-border-subtle text-text-primary rounded-md shadow-lg"
                sideOffset={10}
              >
                Explorer (Ctrl+Shift+E)
              </Tooltip.Content>
            </Tooltip.Root>

            <Tooltip.Root delayDuration={200}>
              <Tooltip.Trigger 
                class="p-2 rounded-md transition-colors cursor-pointer group relative text-text-muted"
                onclick={() => ui.setViewMode('focus')}
              >
                <PenLine size={24} strokeWidth={1.5} />
              </Tooltip.Trigger>
              <Tooltip.Content 
                side="right" 
                class="z-50 px-3 py-1.5 text-xs bg-bg-activity border border-border-subtle text-text-primary rounded-md shadow-lg"
                sideOffset={10}
              >
                Focus Mode (Esc)
              </Tooltip.Content>
            </Tooltip.Root>

            <Tooltip.Root delayDuration={200}>
              <Tooltip.Trigger 
                class="p-2 rounded-md transition-colors cursor-pointer group relative {ui.viewMode === 'preview' ? 'text-brand-primary' : 'text-text-muted'}"
                onclick={() => ui.setViewMode('preview')}
              >
                <BookOpen size={24} strokeWidth={1.5} />
                {#if ui.viewMode === 'preview'}
                  <div class="absolute left-0 top-1/2 -translate-y-1/2 w-0.5 h-6 bg-brand-primary rounded-r-full"></div>
                {/if}
              </Tooltip.Trigger>
              <Tooltip.Content 
                side="right" 
                class="z-50 px-3 py-1.5 text-xs bg-bg-activity border border-border-subtle text-text-primary rounded-md shadow-lg"
                sideOffset={10}
              >
                Preview Mode (Ctrl+Shift+P)
              </Tooltip.Content>
            </Tooltip.Root>

            <Tooltip.Root delayDuration={200}>
              <Tooltip.Trigger
                class="p-2 rounded-md transition-colors cursor-pointer text-text-muted hover:text-text-primary"
                onclick={() => searchOpen = true}
              >
                <Search size={24} strokeWidth={1.5} />
              </Tooltip.Trigger>
              <Tooltip.Content
                side="right"
                class="z-50 px-3 py-1.5 text-xs bg-bg-activity border border-border-subtle text-text-primary rounded-md shadow-lg"
                sideOffset={10}
              >
                Search (Ctrl+P)
              </Tooltip.Content>
            </Tooltip.Root>
          </Tooltip.Provider>
        </div>

        <div class="flex flex-col gap-4">
          <button class="p-2 text-text-muted hover:text-text-primary transition-colors cursor-pointer">
            <User size={24} strokeWidth={1.5} />
          </button>
          <button class="p-2 text-text-muted hover:text-text-primary transition-colors cursor-pointer">
            <Settings size={24} strokeWidth={1.5} />
          </button>
        </div>
      </div>
    {/if}

    <!-- Sidebar (File Explorer / TOC) -->
    {#if ui.sidebarVisible}
      <div
        class="bg-bg-sidebar border-r border-border-subtle overflow-hidden flex flex-col z-10 shrink-0"
        style="width: {ui.sidebarWidth}px"
      >
        <Sidebar onsearch={() => searchOpen = true} />
      </div>
      <button
        class="w-1 shrink-0 cursor-col-resize hover:bg-brand-primary/50 active:bg-brand-primary transition-colors z-20 p-0 {isDragging ? 'bg-brand-primary/50' : ''}"
        onmousedown={startResize}
        aria-label="Resize sidebar"
      ></button>
    {/if}

    <!-- Main Content -->
    <main class="flex-1 flex flex-col min-w-0 overflow-hidden relative">
      {#if ui.viewMode === 'focus'}
        <button 
          class="absolute top-4 left-4 p-2 text-text-muted hover:text-text-primary hover:bg-white/5 rounded-md z-30 transition-all opacity-0 hover:opacity-100"
          onclick={() => ui.setViewMode('workspace')}
          title="Exit Focus Mode (Esc)"
        >
          <Files size={20} />
        </button>
      {/if}
      
      <div class="flex-1 flex flex-col min-w-0 overflow-hidden">
        {@render children()}
      </div>
    </main>
  </div>

  <!-- Status Bar -->
  {#if ui.viewMode !== 'focus'}
    <footer class="h-6 bg-bg-status text-white text-[11px] flex items-center px-3 justify-between z-30">
      <div class="flex items-center gap-4">
        <div class="flex items-center gap-1.5">
          <div class="w-2 h-2 rounded-full bg-green-400"></div>
          <span class="font-medium">{workspace.current?.name || 'No Workspace'}</span>
        </div>
        <span class="opacity-70 truncate max-w-md">{workspace.current?.path || ''}</span>
      </div>
      <div class="flex items-center gap-4">
        <button
          class="flex items-center gap-1 opacity-60 hover:opacity-100 transition-opacity cursor-pointer"
          onclick={() => shortcutsOpen = true}
          title="Keyboard Shortcuts"
        >
          <Keyboard size={12} />
        </button>
        <button
          class="flex items-center gap-1.5 opacity-80 hover:opacity-100 transition-opacity cursor-pointer"
          onclick={() => ui.toggleTheme()}
          title="Toggle Theme"
        >
          {#if ui.theme === 'dark'}
            <Sun size={12} />
          {:else}
            <Moon size={12} />
          {/if}
          <span class="uppercase tracking-wider">{ui.theme} mode</span>
        </button>
        <span class="uppercase tracking-wider opacity-80">{ui.viewMode}</span>
        <span class="opacity-80">UTF-8</span>
      </div>
    </footer>
  {/if}
</div>

<SearchDialog open={searchOpen} onclose={() => searchOpen = false} />
<ShortcutsDialog bind:open={shortcutsOpen} />
