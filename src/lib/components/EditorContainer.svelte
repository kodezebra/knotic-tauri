<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import * as monaco from 'monaco-editor';
  import { editor } from '$lib/state/editor.svelte';
  import { ui } from '$lib/state/ui.svelte';

  let editorElement: HTMLDivElement;
  let monacoInstance: monaco.editor.IStandaloneCodeEditor;

  onMount(() => {
    monacoInstance = monaco.editor.create(editorElement, {
      value: editor.content,
      language: 'markdown',
      theme: ui.theme === 'dark' ? 'vs-dark' : 'vs',
      automaticLayout: true,
      minimap: { enabled: false },
      fontSize: 14,
      fontFamily: 'JetBrains Mono',
      lineNumbers: 'on',
      padding: { top: 10 },
      wordWrap: 'on'
    });

    monacoInstance.onDidChangeModelContent(() => {
      editor.updateContent(monacoInstance.getValue());
    });

    monacoInstance.addCommand(monaco.KeyMod.CtrlCmd | monaco.KeyCode.KeyS, () => {
      editor.save();
    });
  });

  $effect(() => {
    if (monacoInstance) {
      monaco.editor.setTheme(ui.theme === 'dark' ? 'vs-dark' : 'vs');
    }
  });

  $effect(() => {
    if (monacoInstance && editor.content !== monacoInstance.getValue()) {
      monacoInstance.setValue(editor.content);
    }
  });

  onDestroy(() => {
    monacoInstance?.dispose();
  });
</script>

<div class="flex-1 h-full overflow-hidden flex flex-col">
  <div class="flex-1" bind:this={editorElement}></div>
</div>
