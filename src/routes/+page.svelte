<script lang="ts">
  import { editor } from '$lib/state/editor.svelte';
  import { ui } from '$lib/state/ui.svelte';
  import EditorContainer from '$lib/components/EditorContainer.svelte';
  import PreviewContainer from '$lib/components/PreviewContainer.svelte';
  import { X, BookOpenCheck } from 'lucide-svelte';

  function closeTab(path: string) {
    editor.closeTab(path);
  }
</script>

{#if editor.tabs.length > 0}
  <div class="flex flex-col h-full overflow-hidden">
    <!-- Tab Bar (Only in Workspace mode) -->
    {#if ui.viewMode === 'workspace'}
      <div class="flex h-9 bg-bg-activity border-b border-border-subtle overflow-x-auto no-scrollbar">
        {#each editor.tabs as tab (tab.path)}
          <button 
            class="flex items-center h-full px-3 gap-2 border-r border-border-subtle min-w-[120px] max-w-[200px] text-xs transition-colors group cursor-pointer"
            class:bg-bg-editor={editor.activePath === tab.path}
            class:text-text-primary={editor.activePath === tab.path}
            class:text-text-muted={editor.activePath !== tab.path}
            onclick={() => editor.openFile(tab.path, tab.name)}
          >
            <span class="truncate flex-1 text-left">{tab.name}</span>
            {#if tab.isDirty}
              <div class="w-2 h-2 rounded-full bg-slate-400 group-hover:hidden"></div>
            {/if}
            <span
              role="button"
              tabindex="0"
              class="p-0.5 hover:bg-white/10 rounded opacity-0 group-hover:opacity-100 transition-opacity cursor-pointer"
              onclick={(e) => { e.stopPropagation(); closeTab(tab.path); }}
              onkeydown={(e) => { if (e.key === 'Enter' || e.key === ' ') { e.stopPropagation(); closeTab(tab.path); } }}
            >
              <X size={12} />
            </span>
          </button>
        {/each}
      </div>
    {/if}

    <!-- Content Area -->
    <div class="flex-1 flex overflow-hidden">
      {#if ui.viewMode === 'workspace'}
        <!-- Workspace Mode: Split View -->
        <div class="flex-1 h-full overflow-hidden border-r border-border-subtle">
          <EditorContainer />
        </div>

        {#if ui.previewVisible}
          <div class="flex-1 h-full overflow-hidden bg-bg-editor">
            <PreviewContainer />
          </div>
        {/if}
      {:else if ui.viewMode === 'focus'}
        {#if ui.previewVisible}
          <!-- Focus Mode: Split (no chrome) -->
          <div class="flex-1 flex overflow-hidden bg-bg-editor">
            <div class="flex-1 h-full overflow-hidden border-r border-border-subtle">
              <EditorContainer />
            </div>
            <div class="flex-1 h-full overflow-hidden">
              <PreviewContainer />
            </div>
          </div>
        {:else}
          <!-- Focus Mode: Centered Editor -->
          <div class="flex-1 h-full overflow-hidden bg-bg-editor flex justify-center py-8">
            <div class="w-full max-w-4xl h-full shadow-2xl border border-border-subtle rounded-lg overflow-hidden">
              <EditorContainer />
            </div>
          </div>
        {/if}
      {:else if ui.viewMode === 'preview'}
        <!-- Preview Mode: Documentation View -->
        <div class="flex-1 h-full overflow-hidden bg-bg-editor">
          <PreviewContainer />
        </div>
      {/if}
    </div>
  </div>
{:else}
  <div class="flex-1 flex flex-col items-center justify-center gap-6 p-8 text-center bg-bg-editor">
    <div class="w-24 h-24 rounded-2xl bg-brand-primary/10 flex items-center justify-center text-brand-primary">
      <BookOpenCheck size={48} />
    </div>
    <div class="space-y-2">
      <h1 class="text-2xl font-bold text-text-primary">Welcome to Knotic</h1>
      <p class="text-text-muted max-w-md">Open a tutorial from the explorer or create a new one to get started with your documentation.</p>
    </div>
  </div>
{/if}

<style>
  .no-scrollbar::-webkit-scrollbar {
    display: none;
  }
  .no-scrollbar {
    -ms-overflow-style: none;
    scrollbar-width: none;
  }
</style>
