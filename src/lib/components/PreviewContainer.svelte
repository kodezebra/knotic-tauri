<script lang="ts">
  import { onMount } from 'svelte';
  import MarkdownIt from 'markdown-it';
  import { editor } from '$lib/state/editor.svelte';
  import { ui } from '$lib/state/ui.svelte';
  import { createHighlighter } from 'shiki';

  let md: MarkdownIt;
  let highlighter: any;
  let renderedContent = $state('');
  let previewEl: HTMLDivElement;

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
          const html = highlighter.codeToHtml(code, {
            lang,
            theme: ui.theme === 'dark' ? 'github-dark' : 'github-light'
          });
          return html.replace('<pre', `<pre data-language="${lang}"`);
        }
        return '';
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

  $effect(() => {
    if (renderedContent && previewEl) {
      const pres = previewEl.querySelectorAll<HTMLPreElement>('pre[data-language]:not([data-enhanced])');
      pres.forEach(pre => enhanceCodeBlock(pre));
    }
  });

  function enhanceCodeBlock(pre: HTMLPreElement) {
    pre.dataset.enhanced = 'true';
    const lang = pre.dataset.language || '';

    const wrapper = document.createElement('div');
    wrapper.className = 'code-block';

    const header = document.createElement('div');
    header.className = 'code-block-header';

    const badge = document.createElement('span');
    badge.className = 'code-block-lang';
    badge.textContent = lang;

    const copyBtn = document.createElement('button');
    copyBtn.className = 'copy-btn';
    copyBtn.dataset.code = encodeURIComponent(pre.textContent || '');
    copyBtn.textContent = 'Copy';

    header.appendChild(badge);
    header.appendChild(copyBtn);
    wrapper.appendChild(header);

    pre.parentNode?.insertBefore(wrapper, pre);
    wrapper.appendChild(pre);
  }

  function handleCopyClick(e: MouseEvent) {
    const target = e.target as HTMLElement;
    const btn = target.closest('.copy-btn') as HTMLButtonElement | null;
    if (!btn || !btn.dataset.code) return;

    navigator.clipboard.writeText(decodeURIComponent(btn.dataset.code));
    btn.textContent = 'Copied';
    setTimeout(() => { btn.textContent = 'Copy'; }, 2000);
  }
</script>

<!-- svelte-ignore a11y_click_events_have_key_events -->
<div
  bind:this={previewEl}
  role="none"
  class="flex-1 h-full overflow-y-auto p-8 prose dark:prose-invert max-w-none bg-bg-editor transition-colors"
  onclick={handleCopyClick}
>
  {@html renderedContent}
</div>

<style>
  :global(.code-block) {
    border: 1px solid var(--color-border-subtle);
    border-radius: 8px;
    overflow: hidden;
    margin: 1.5em 0;
  }

  :global(.code-block pre) {
    margin: 0;
    border-radius: 0;
    padding: 1rem !important;
    overflow-x: auto;
  }

  :global(.code-block-header) {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 0.375rem 0.75rem;
    background: var(--color-bg-sidebar);
    border-bottom: 1px solid var(--color-border-subtle);
    font-size: 0.75rem;
    font-family: var(--font-sans);
  }

  :global(.code-block-lang) {
    text-transform: uppercase;
    font-weight: 600;
    color: var(--color-text-muted);
    letter-spacing: 0.05em;
  }

  :global(.copy-btn) {
    opacity: 0;
    transition: opacity 0.15s;
    cursor: pointer;
    background: none;
    border: none;
    color: var(--color-text-muted);
    font-size: 0.75rem;
    font-family: inherit;
    padding: 0.125rem 0.375rem;
    border-radius: 4px;
  }

  :global(.copy-btn:hover) {
    color: var(--color-text-primary);
    background: rgba(255, 255, 255, 0.05);
  }

  :global(.code-block:hover .copy-btn) {
    opacity: 1;
  }
</style>
