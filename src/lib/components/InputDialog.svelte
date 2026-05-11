<script lang="ts">
  import { Dialog } from "bits-ui";
  import { X } from "lucide-svelte";

  let {
    open = $bindable(false),
    title = "New File",
    label = "Name",
    placeholder = "",
    initialValue = "",
    submitLabel = "Create",
    onSubmit = (value: string) => {},
  } = $props();

  let inputValue = $state('');

  $effect(() => {
    if (open) {
      inputValue = initialValue;
      setTimeout(() => {
        document.getElementById('input-dialog-field')?.focus();
      }, 50);
    }
  });

  function handleSubmit() {
    if (inputValue.trim()) {
      onSubmit(inputValue.trim());
      open = false;
    }
  }

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === 'Enter') handleSubmit();
  }
</script>

<Dialog.Root bind:open>
  <Dialog.Portal>
    <Dialog.Overlay class="fixed inset-0 z-50 bg-black/50 backdrop-blur-sm" />
    <Dialog.Content class="fixed left-[50%] top-[50%] z-50 w-[90vw] max-w-[400px] max-sm:w-[95vw] translate-x-[-50%] translate-y-[-50%] rounded-xl bg-bg-activity p-6 max-sm:p-4 shadow-2xl border border-border-subtle">
      <div class="flex items-center justify-between mb-6">
        <Dialog.Title class="text-lg font-bold text-text-primary">{title}</Dialog.Title>
        <Dialog.Close class="p-1 hover:bg-white/10 rounded transition-colors text-text-muted hover:text-text-primary">
          <X size={20} />
        </Dialog.Close>
      </div>

      <div class="space-y-2">
        <label for="input-dialog-field" class="text-xs font-semibold uppercase text-text-secondary">{label}</label>
        <input
          id="input-dialog-field"
          bind:value={inputValue}
          placeholder={placeholder}
          onkeydown={handleKeydown}
          class="w-full bg-bg-editor border border-border-subtle rounded-lg px-3 py-2 text-sm text-text-primary focus:outline-none focus:border-brand-primary transition-colors"
        />
      </div>

      <div class="mt-8 flex justify-end gap-3">
        <Dialog.Close class="px-4 py-2 text-sm text-text-muted hover:text-text-primary transition-colors">
          Cancel
        </Dialog.Close>
        <button
          onclick={handleSubmit}
          disabled={!inputValue.trim()}
          class="px-4 py-2 bg-brand-primary hover:bg-brand-primary/80 disabled:opacity-50 disabled:cursor-not-allowed text-white rounded-lg text-sm font-semibold transition-colors"
        >
          {submitLabel}
        </button>
      </div>
    </Dialog.Content>
  </Dialog.Portal>
</Dialog.Root>
