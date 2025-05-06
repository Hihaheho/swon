import 'vscode-oniguruma';
import '@vscode/codicons/dist/codicon.css';


const editorContainer = document.getElementById('editor');

async function initVSCodeWeb() {
  if (!editorContainer) {
    console.error('Editor container not found');
    return;
  }

  try {
    editorContainer.innerHTML = '<div style="padding: 20px; font-family: sans-serif;">Loading VS Code Web...</div>';
    
    
    console.log('VS Code Web initialized');
  } catch (error) {
    console.error('Failed to initialize VS Code Web:', error);
    editorContainer.innerHTML = `<div style="padding: 20px; color: red; font-family: sans-serif;">
      Failed to initialize VS Code Web: ${error instanceof Error ? error.message : String(error)}
    </div>`;
  }
}

window.addEventListener('DOMContentLoaded', () => {
  initVSCodeWeb();
});
