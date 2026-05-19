export async function exportPdf() {
  await new Promise(r => requestAnimationFrame(r));
  window.print();
}
