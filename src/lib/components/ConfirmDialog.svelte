<script lang="ts">
  import { Dialog } from "bits-ui";
  import { X, AlertTriangle } from "lucide-svelte";

  let {
    open = $bindable(false),
    title = "Confirm",
    message = "Are you sure?",
    confirmLabel = "Delete",
    variant = "danger" as 'danger' | 'default',
    onConfirm = () => {},
  } = $props();

  function handleConfirm() {
    onConfirm();
    open = false;
  }
</script>

<Dialog.Root bind:open>
  <Dialog.Portal>
    <Dialog.Overlay class="fixed inset-0 z-50 bg-black/50 backdrop-blur-sm" />
    <Dialog.Content class="fixed left-[50%] top-[50%] z-50 w-[90vw] max-w-[400px] max-sm:w-[95vw] translate-x-[-50%] translate-y-[-50%] rounded-xl bg-bg-activity p-6 max-sm:p-4 shadow-2xl border border-border-subtle">
      <div class="flex items-center gap-3 mb-6">
        {#if variant === 'danger'}
          <div class="w-10 h-10 rounded-full bg-red-500/20 flex items-center justify-center text-red-400 shrink-0">
            <AlertTriangle size={20} />
          </div>
        {/if}
        <div>
          <Dialog.Title class="text-lg font-bold text-text-primary">{title}</Dialog.Title>
          <p class="text-sm text-text-muted mt-1">{message}</p>
        </div>
        <div class="ml-auto">
          <Dialog.Close class="p-1 hover:bg-white/10 rounded transition-colors text-text-muted hover:text-text-primary">
            <X size={20} />
          </Dialog.Close>
        </div>
      </div>

      <div class="flex justify-end gap-3">
        <Dialog.Close class="px-4 py-2 text-sm text-text-muted hover:text-text-primary transition-colors">
          Cancel
        </Dialog.Close>
        <button
          onclick={handleConfirm}
          class="px-4 py-2 rounded-lg text-sm font-semibold transition-colors {variant === 'danger' ? 'bg-red-500 hover:bg-red-600 text-white' : 'bg-brand-primary hover:bg-brand-primary/80 text-white'}"
        >
          {confirmLabel}
        </button>
      </div>
    </Dialog.Content>
  </Dialog.Portal>
</Dialog.Root>
