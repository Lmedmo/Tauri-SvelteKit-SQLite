# Synopsis

This guide will walk you through the process of integrating the primary components of a modern full stack desktop application — Frontend, Backend, and Database. Tauri which is implemented in Rust will serve as our Backend, while SvelteKit will serve as our Frontend. The database we will use is SQLite and to do this we will be using SQLx; a Rust toolkit for communicating asynchronously with a number of different databases (MySQL, MSSQL, PostgreSQL, and SQLite). We will use Tauri’s built-in capabilities to then pass information back and forth between the front and backend. The purpose of this guide is simply to share what I have learned in my journey to create cross platform applications for a number of personal and professional projects. As I continue to grow my understanding and refine techniques to build applications on top of this core architecture, I will be sure to update the project’s GitHub repository along with this guide to reflect my methodology (See the ‘Additional Enhancements’ section). I will also include helpful links to articles, documentation, examples, and eventually links to my own applications once I have gotten them ready enough to be seen and taken seriously haha. With all that said, let’s get coding!!

## Step 1: Set up environment and Install prerequisites

### Windows

### MacOS

### Linux

## Step 2: Create the SvelteKit Frontend

- [ ]  Using the tool of your choice, run one of the following commands
    
    ```bash
    npm create svelte@latest
    ```
    
    ```bash
    pnpm create svelte
    ```
    
- [ ]  Select the following options:
    - Current directory
    - Yes (to Continue)
    - Skeleton project
    - TypeScript
    - Additions: ESLint, Prettier
- [ ]  Using the tool of your choice, run one of the following commands
    
    ```bash
    npm install
    ```
    
    ```bash
    pnpm install
    ```
    

<aside>
💡 **Optional**

Initialize Git repository and commit

```bash
git init && git add -A && git commit -m "Initial commit”
```

</aside>
- [ ]  Add static adapter for Static Site Generation
    
    ```bash
    npm install --save-dev @sveltejs/adapter-static
    ```
    
    ```bash
    pnpm add -D @sveltejs/adapter-static
    ```
    
- [ ]  Open the `svelte.config.js` file and edit the adapter import line:
    
    ```jsx
    import adapter from '@sveltejs/adapter-static'; // <-- This was changed from 'adapter-auto' to 'adapter-static'
    import { vitePreprocess } from '@sveltejs/kit/vite';
    
    /** @type {import('@sveltejs/kit').Config} */
    const config = {
    	// Consult https://kit.svelte.dev/docs/integrations#preprocessors
    	// for more information about preprocessors
    	preprocess: vitePreprocess(),
    
    	kit: {
    		// adapter-auto only supports some environments, see https://kit.svelte.dev/docs/adapter-auto for a list.
    		// If your environment is not supported or you settled on a specific environment, switch out the adapter.
    		// See https://kit.svelte.dev/docs/adapters for more information about adapters.
    		adapter: adapter()
    	}
    };
    
    export default config;
    ```
    
- [ ]  Create a `+layout.ts` file inside the `/*app_name*/src/routes` directory. Insert the following 2 lines of code to disable Server Side Rendering (SSR):
    
    ```tsx
    export const prerender = true
    export const ssr = false
    ```
    

---
