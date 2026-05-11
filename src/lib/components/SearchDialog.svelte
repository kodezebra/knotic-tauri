<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import { editor } from '$lib/state/editor.svelte';
  import { workspace } from '$lib/state/workspace.svelte';
  import { Search, FileText, LoaderCircle, X } from 'lucide-svelte';
  import { onMount } from 'svelte';

  interface SearchMatch {
    line: number;
    content: string;
  }

  interface SearchResult {
    path: string;
    name: string;
    matches: SearchMatch[];
    is_filename_match: boolean;
  }

  let {
    open = false,
    onclose = () => {},
  } = $props();

  let query = $state('');
  let results = $state<SearchResult[]>([]);
  let selectedIndex = $state(0);
  let isLoading = $state(false);
  let searchTimer: ReturnType<typeof setTimeout> | null = null;
  let inputEl = $state<HTMLInputElement>();
  let dialogEl = $state<HTMLDivElement>();

  onMount(() => {
    function handleKeydown(e: KeyboardEvent) {
      if (!open) return;
      if (e.key === 'Escape') {
        e.preventDefault();
        onclose();
      } else if (e.key === 'ArrowDown') {
        e.preventDefault();
        selectedIndex = Math.min(selectedIndex + 1, Math.max(results.length - 1, 0));
      } else if (e.key === 'ArrowUp') {
        e.preventDefault();
        selectedIndex = Math.max(selectedIndex - 1, 0);
      } else if (e.key === 'Enter') {
        e.preventDefault();
        const r = results[selectedIndex];
        if (r) openResult(r);
      }
    }

    window.addEventListener('keydown', handleKeydown);
    return () => window.removeEventListener('keydown', handleKeydown);
  });

  $effect(() => {
    if (open) {
      query = '';
      results = [];
      selectedIndex = 0;
      isLoading = false;
      setTimeout(() => inputEl?.focus(), 50);
    }
  });

  function onQueryInput() {
    if (searchTimer) clearTimeout(searchTimer);
    if (!query.trim()) {
      results = [];
      return;
    }
    searchTimer = setTimeout(performSearch, 200);
  }

  async function performSearch() {
    const q = query.trim();
    if (!q || !workspace.current) return;
    isLoading = true;
    try {
      results = await invoke<SearchResult[]>('search_files', { query: q });
      selectedIndex = 0;
    } catch (e) {
      console.error('Search failed:', e);
    } finally {
      isLoading = false;
    }
  }

  function openResult(result: SearchResult) {
    editor.openFile(result.path, result.name);
    onclose();
  }
</script>

{#if open}
  <div bind:this={dialogEl} class="fixed inset-0 z-50 flex items-start justify-center pt-[20vh]">
    <div
      class="fixed inset-0 bg-black/40 backdrop-blur-sm"
      role="button"
      tabindex="0"
      onclick={() => onclose()}
      onkeydown={(e) => { if (e.key === 'Enter' || e.key === ' ') onclose(); }}
    ></div>

    <div class="relative w-[90vw] max-w-[600px] max-sm:w-[95vw] flex flex-col gap-1">
      <div class="bg-bg-activity border border-border-subtle rounded-xl shadow-2xl overflow-hidden">
        <div class="flex items-center gap-2 px-4 py-3 border-b border-border-subtle">
          <Search size={16} class="text-text-muted shrink-0" />
          <input
            bind:this={inputEl}
            type="text"
            bind:value={query}
            oninput={onQueryInput}
            placeholder="Search files and content..."
            class="flex-1 bg-transparent text-sm text-text-primary placeholder-text-muted focus:outline-none"
          />
          {#if isLoading}
            <LoaderCircle size={16} class="text-brand-primary animate-spin shrink-0" />
          {/if}
          <button
            class="p-0.5 hover:bg-white/10 rounded text-text-muted hover:text-text-primary transition-colors shrink-0 cursor-pointer"
            onclick={() => onclose()}
          >
            <X size={16} />
          </button>
        </div>

        {#if query.trim() && !isLoading}
          {#if results.length > 0}
            <div class="max-h-[400px] max-sm:max-h-[60vh] overflow-y-auto p-1">
              {#each results as result, i (result.path)}
                <button
                  class="w-full flex items-start gap-3 px-3 py-2 rounded-lg text-left transition-colors cursor-pointer {i === selectedIndex ? 'bg-brand-primary/20' : 'hover:bg-white/5'}"
                  onclick={() => openResult(result)}
                  onmouseenter={() => selectedIndex = i}
                >
                  <div class="p-1 rounded bg-white/5 mt-0.5 shrink-0">
                    <FileText size={14} class="text-text-muted" />
                  </div>
                  <div class="flex-1 min-w-0">
                    <div class="flex items-center gap-2">
                      <span class="text-sm font-medium text-text-primary truncate">{result.name}</span>
                      {#if result.is_filename_match}
                        <span class="text-[10px] px-1.5 py-0.5 rounded bg-brand-primary/20 text-brand-primary font-medium">filename</span>
                      {/if}
                    </div>
                    <div class="text-[11px] text-text-muted truncate mt-0.5">{result.path}</div>
                    {#if result.matches.length > 0}
                      <div class="mt-1 space-y-0.5">
                        {#each result.matches.slice(0, 2) as match}
                          <div class="text-[11px] text-text-muted font-mono truncate pl-2 border-l-2 border-text-muted">
                            <span class="text-text-muted mr-1">L{match.line}</span>
                            {match.content}
                          </div>
                        {/each}
                        {#if result.matches.length > 2}
                          <div class="text-[10px] text-text-muted pl-2">+{result.matches.length - 2} more matches</div>
                        {/if}
                      </div>
                    {/if}
                  </div>
                </button>
              {/each}
            </div>
          {:else}
            <div class="p-6 text-center text-sm text-text-muted">No results found</div>
          {/if}
        {:else if !query.trim() && !isLoading}
          <div class="p-6 text-center text-sm text-text-muted">Type to search files and content</div>
        {/if}
      </div>
      <div class="px-3 py-1.5 text-[10px] text-text-muted flex items-center gap-3">
        <span>↑↓ navigate</span>
        <span>↵ open</span>
        <span>esc close</span>
      </div>
    </div>
  </div>
{/if}
