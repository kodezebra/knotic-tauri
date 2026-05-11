<script lang="ts">
  import { onMount } from 'svelte';
  import MarkdownIt from 'markdown-it';
  import { editor } from '$lib/state/editor.svelte';
  import { ui } from '$lib/state/ui.svelte';
  import { createHighlighter } from 'shiki';

  let md: MarkdownIt;
  let highlighter: any;
  let renderedContent = $state('');

  onMount(async () => {
    highlighter = await createHighlighter({
      themes: ['github-dark', 'github-light'],
      langs: ['javascript', 'typescript', 'rust', 'svelte', 'css', 'html', 'json', 'markdown']
    });

    md = new MarkdownIt({
      html: true,
      linkify: true,
      typographer: true,
      highlight: (code: string, lang: string) => {
        if (highlighter && lang) {
          return highlighter.codeToHtml(code, {
            lang,
            theme: ui.theme === 'dark' ? 'github-dark' : 'github-light'
          });
        }
        return ''; // use external default escaping
      }
    });

    updatePreview();
  });

  function updatePreview() {
    if (md) {
      renderedContent = md.render(editor.content);
    }
  }

  $effect(() => {
    if (editor.content || true) {
      updatePreview();
    }
  });
</script>

  <div class="flex-1 h-full overflow-y-auto p-8 prose dark:prose-invert max-w-none bg-bg-editor transition-colors">
  {@html renderedContent}
</div>

<style>
  :global(.prose pre) {
    background-color: transparent !important;
    padding: 0 !important;
  }
</style>
