<script lang="ts">
  import { Dialog } from "bits-ui";
  import { workspace } from "$lib/state/workspace.svelte";
  import { open } from "@tauri-apps/plugin-dialog";
  import { Plus, X, Folder } from "lucide-svelte";

  let openState = $state(false);
  let name = $state("");
  let path = $state("");

  async function selectFolder() {
    const selected = await open({
      directory: true,
      multiple: false,
    });
    if (selected) {
      path = selected;
    }
  }

  async function createWorkspace() {
    if (name && path) {
      await workspace.create(path, name);
      openState = false;
      name = "";
      path = "";
    }
  }
</script>

<Dialog.Root bind:open={openState}>
  <Dialog.Trigger
    class="p-1 hover:bg-white/10 rounded cursor-pointer"
    title="New Workspace"
  >
    <Plus size={14} />
  </Dialog.Trigger>
  <Dialog.Portal>
    <Dialog.Overlay
      class="fixed inset-0 z-50 bg-black/50 backdrop-blur-sm"
    />
    <Dialog.Content
      class="fixed left-[50%] top-[50%] z-50 w-[90vw] max-w-[400px] max-sm:w-[95vw] translate-x-[-50%] translate-y-[-50%] rounded-xl bg-bg-activity p-6 max-sm:p-4 shadow-2xl border border-border-subtle"
    >
      <div class="flex items-center justify-between mb-6">
        <Dialog.Title class="text-lg font-bold text-text-primary">New Workspace</Dialog.Title>
        <Dialog.Close class="p-1 hover:bg-white/10 rounded transition-colors text-text-muted hover:text-text-primary">
          <X size={20} />
        </Dialog.Close>
      </div>

      <div class="space-y-4">
        <div class="space-y-2">
          <label for="name" class="text-xs font-semibold uppercase text-text-muted">Workspace Name</label>
          <input
            id="name"
            bind:value={name}
            placeholder="e.g. My Tutorial"
            class="w-full bg-bg-editor border border-border-subtle rounded-lg px-3 py-2 text-sm text-text-primary focus:outline-none focus:border-brand-primary transition-colors"
          />
        </div>

        <div class="space-y-2">
          <label for="path" class="text-xs font-semibold uppercase text-text-muted">Location</label>
          <div class="flex gap-2">
            <input
              id="path"
              bind:value={path}
              readonly
              placeholder="Select folder..."
              class="flex-1 bg-bg-editor border border-border-subtle rounded-lg px-3 py-2 text-sm text-text-primary focus:outline-none"
            />
            <button
              onclick={selectFolder}
              class="px-3 bg-bg-sidebar hover:bg-white/5 border border-border-subtle rounded-lg text-text-secondary transition-colors cursor-pointer"
            >
              <Folder size={18} />
            </button>
          </div>
        </div>
      </div>

      <div class="mt-8 flex justify-end gap-3">
        <Dialog.Close class="px-4 py-2 text-sm text-text-muted hover:text-text-primary transition-colors">
          Cancel
        </Dialog.Close>
        <button
          onclick={createWorkspace}
          disabled={!name || !path}
          class="px-4 py-2 bg-brand-primary hover:bg-brand-primary/80 disabled:opacity-50 disabled:cursor-not-allowed text-white rounded-lg text-sm font-semibold transition-colors"
        >
          Create Workspace
        </button>
      </div>
    </Dialog.Content>
  </Dialog.Portal>
</Dialog.Root>
