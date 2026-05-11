<script lang="ts">
  import { Dialog } from 'bits-ui';
  import { X, Keyboard } from 'lucide-svelte';

  let { open = $bindable(false) } = $props();

  interface ShortcutGroup {
    label: string;
    items: { keys: string; desc: string }[];
  }

  const groups: ShortcutGroup[] = [
    {
      label: 'General',
      items: [
        { keys: 'Ctrl + P', desc: 'Search files and content' },
        { keys: 'Ctrl + N', desc: 'New file' },
        { keys: 'Ctrl + S', desc: 'Save current file' },
        { keys: 'Ctrl + W', desc: 'Close active tab' },
        { keys: 'Ctrl + Tab', desc: 'Next tab' },
        { keys: 'Ctrl + Shift + Tab', desc: 'Previous tab' },
      ],
    },
    {
      label: 'View',
      items: [
        { keys: 'Ctrl + B', desc: 'Toggle sidebar' },
        { keys: 'Ctrl + Shift + P', desc: 'Toggle preview pane' },
        { keys: 'Esc', desc: 'Exit focus mode' },
      ],
    },
    {
      label: 'Editor',
      items: [
        { keys: 'Ctrl + Z', desc: 'Undo' },
        { keys: 'Ctrl + Y', desc: 'Redo' },
        { keys: 'Ctrl + F', desc: 'Find in file' },
        { keys: 'Ctrl + H', desc: 'Replace in file' },
      ],
    },
  ];

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === 'Escape') open = false;
  }
</script>

<Dialog.Root bind:open>
  <Dialog.Portal>
    <Dialog.Overlay class="fixed inset-0 z-50 bg-black/40 backdrop-blur-sm" />
    <Dialog.Content
      class="fixed left-[50%] top-[50%] z-50 w-[90vw] max-w-[480px] max-sm:w-[95vw] translate-x-[-50%] translate-y-[-50%] rounded-xl bg-bg-activity p-6 max-sm:p-4 shadow-2xl border border-border-subtle"
      onkeydown={handleKeydown}
    >
      <div class="flex items-center justify-between mb-6">
        <div class="flex items-center gap-2">
          <Keyboard size={18} class="text-brand-primary" />
          <Dialog.Title class="text-lg font-bold text-text-primary">
            Keyboard Shortcuts
          </Dialog.Title>
        </div>
        <Dialog.Close class="p-1 hover:bg-white/10 rounded transition-colors text-text-muted hover:text-text-primary">
          <X size={20} />
        </Dialog.Close>
      </div>

      <div class="space-y-5 max-h-[400px] max-sm:max-h-[60vh] overflow-y-auto -mx-2 px-2">
        {#each groups as group}
          <div>
            <h3 class="text-xs font-bold uppercase tracking-wider text-text-muted mb-2">{group.label}</h3>
            <div class="space-y-1">
              {#each group.items as item}
                <div class="flex items-center justify-between py-1.5 px-2 rounded hover:bg-white/5">
                  <span class="text-sm text-text-primary">{item.desc}</span>
                  <kbd class="text-[11px] px-2 py-0.5 rounded bg-bg-editor border border-border-subtle text-text-secondary font-mono">
                    {item.keys}
                  </kbd>
                </div>
              {/each}
            </div>
          </div>
        {/each}
      </div>

      <div class="mt-6 pt-3 border-t border-border-subtle text-[11px] text-text-muted text-center">
        Press <kbd class="px-1.5 py-0.5 rounded bg-bg-editor border border-border-subtle font-mono text-text-secondary">Ctrl + /</kbd> to open this dialog anytime
      </div>
    </Dialog.Content>
  </Dialog.Portal>
</Dialog.Root>
