![tauri-svelte-sqlite](https://github.com/Lmedmo/Tauri-SvelteKit-SQLite/assets/102483463/dce676f1-d899-4695-98be-df16a02a0eab)

# Overview
This guide will walk you through the process of integrating the primary components of a modern full stack desktop application — Frontend, Backend, and Database. Tauri which is implemented in Rust will serve as our Backend, while SvelteKit will serve as our Frontend. The database we will use is SQLite and to do this we will be using SQLx; a Rust toolkit for communicating asynchronously with a number of different databases (MySQL, MSSQL, PostgreSQL, and SQLite). We will use Tauri’s built-in capabilities to then pass information back and forth between the front and backend. The purpose of this guide is simply to share what I have learned in my journey to create cross platform applications for a number of personal and professional projects. As I continue to grow my understanding and refine techniques to build applications on top of this core architecture, I will be sure to update the project’s GitHub repository along with this guide to reflect my methodology (See the ‘Additional Enhancements’ section). I will also include this [link to my Notion page for this project](https://leafdeveloper.notion.site/Tauri-SvelteKit-SQLite-e520a1051f5c47c29f919fbf6bc02015?pvs=4) which includes helpful links to articles, documentation, examples, and eventually links to my own applications once I have gotten them ready enough to be seen and taken seriously haha. With all that said, let’s get coding!!

---

<details>
    <summary>
    <h2>
        Step 1: Set up environment and Install prerequisites
    </h2>
    </summary>
    
<blockquote><details><summary><h3>Windows</h3></summary>

- [ ]  Install **MS Visual Studio C++ build tools** and **Windows 10 SDK**
- [ ]  WebView2 (Most likely already installed as it is part of Win10 and Win11 by default)
- [ ]  Install Rust from the [website](https://www.rust-lang.org/tools/install) or by running the following command
    ```powershell
    winget install --id Rustlang.Rustup
    ```
</details></blockquote>
<blockquote><details><summary><h3>macOS</h3></summary>

- [ ]  Install CLang and macOS Dev Dependencies using the following command
    ```shell
    xcode-select --install
    ```
- [ ]  Install Rust with the following command then restart your Terminal for changes to take effect
    ```shell
    curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
    ```    
</details></blockquote>
<blockquote><details><summary><h3>Linux</h3></summary>

Use this link to [the official Tauri website](https://tauri.app/v1/guides/getting-started/prerequisites#installing) to find instructions on how to install system dependencies for your specific distro        
</details></blockquote> 
</details>

---

<details>
    <summary>
    <h2>
        Step 2: Create the SvelteKit Frontend
    </h2>
    </summary>

- [ ]  Using the tool of your choice, run one of the following commands
    ```shell
    npm create svelte@latest
    ```
    ```shell
    pnpm create svelte
    ```
- [ ]  Select the following options:
    - Current directory
    - Yes (to Continue)
    - Skeleton project
    - TypeScript
    - Additions: ESLint, Prettier
- [ ]  Using the tool of your choice, run one of the following commands
    ```shell
    npm install
    ```
    ```shell
    pnpm install
    ```
- [ ]  Add static adapter for Static Site Generation
    ```shell
    npm install --save-dev @sveltejs/adapter-static
    ```
    ```shell
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
</details>

---

<details>
    <summary>
    <h2>
        Step 3: Add and Configure Tauri
    </h2>
    </summary>
    
- [ ]  Install the Tauri CLI with the following command:
    ```shell
    pnpm add -D @tauri-apps/cli
    ```
    ```shell
    npm install --save-dev @tauri-apps/cli
    ```
    > Note:
    > For **npm** to detect Tauri correctly you need to add it to the "scripts" section in your package.json file:
    > ```json
    > "scripts": {
    >     "tauri": "tauri"
    > }
    > ```
- [ ]  Run the Tauri scaffolding utility and use the options below. Use the following command to run the utility:
    ```shell
    npm run tauri init
    ```
    - App Name: *Any*
    - Window Title: *Any*
    - Web Asset Path: `../build` (*located relative to the `<current dir>/src-tauri/tauri.conf.json` file that will be created)*
    - Dev Server URL: `http://localhost:5173`
    - Frontend dev command: `npm run dev` or `pnpm run dev`
    - Frontend build command: `npm run build` or `pnpm run build`
    
    The result is a folder called `src-tauri` that should contain files such as `Cargo.toml`, `tauri.conf.json`, *icons* and `src/main.rs`. Each is used for tauri to work. To learn more about how to use these files visit the [tauri docs site](https://tauri.app/v1/guides/getting-started/setup/sveltekit).
    
- [ ]  Add the tauri-apps/api JavaScript library
    ```shell
    pnpm add @tauri-apps/api
    ```
    ```shell
    npm install @tauri-apps/api
    ```
    
</details>

---

<details>
    <summary>
    <h2>
        Step 4: SQLite and SQLx
    </h2>
    </summary>

- [ ]  Open `Cargo.toml` and add the following dependencies for **SQLx** and the **async-std** runtime
    ```toml
    [dependencies]
    serde_json = "1.0"
    serde = { version = "1.0", features = ["derive"] }
    tauri = { version = "1.4.0", features = [] }
    sqlx = { version = "0.7", features = [ "runtime-async-std", "tls-native-tls", "sqlite", "macros" ] }
    async-std = { version = "1.7.0", features = [ "attributes" ] }
    ```
- [ ]  Open `main.rs` and add **async_std** to `main()` function. Save changes
    ```rust
    // Prevents additional console window on Windows in release, DO NOT REMOVE!!
    #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
    
    #[async_std::main]
    async fn main() {
      tauri::Builder::default()
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    }
    ```
- [ ]  Create `db.rs` file in `src-tauri/src` directory and add the following
    ```rust
    use sqlx::{ migrate::MigrateDatabase, Sqlite, SqlitePool };
    
    const DB_URL: &str = "sqlite://sqlite.db";
    
    // Check for DB, create if non existent
    pub async fn init() {
        if !Sqlite::database_exists(DB_URL).await.unwrap_or(false) {
            match Sqlite::create_database(DB_URL).await {
                Ok(_) => println!("Create db success"),
                Err(error) => panic!("error: {}", error),
            }
        } else {
            println!("Database already exists");
        }
    
        create_schema().await;
    }
    
    // Create Schema
    async fn create_schema() {
        let pool = SqlitePool::connect(DB_URL).await.expect("unable to connect");
        let sql = "
            PRAGMA foreign_keys = ON ;
            CREATE TABLE IF NOT EXISTS projects
            (
                id              INTEGER    PRIMARY KEY    NOT NULL,
                name            TEXT                      NOT NULL
            );
    
            CREATE TABLE IF NOT EXISTS tasks
            (
                id              INTEGER    PRIMARY KEY    NOT NULL,
                value           TEXT                      NOT NULL,
                completed       INTEGER                   NOT NULL,
                date_completed  TEXT,
                project_id      INTEGER                   NOT NULL,
                FOREIGN KEY (project_id)   REFERENCES projects (id) ON UPDATE SET NULL ON DELETE SET NULL
            );
        ";
        
        let query = sqlx::query(&sql);
        let result = query.execute(&pool).await.unwrap();
        println!("Create Schema result: {:?}", result);   
        pool.close().await;
    }
    ```
- [ ]  Add `db.rs` to `main.rs` as a module and call the `init()` function within the `main()`
    ```rust
    // Prevents additional console window on Windows in release, DO NOT REMOVE!!
    #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
    
    mod db;
    
    #[async_std::main]
    async fn main() {
      db::init().await;
      
      tauri::Builder::default()
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    }
    ```
- [ ]  Create `commands.rs` file in the `src-tauri/src` directory and add the following
    ```rust
    use sqlx::FromRow;
    use serde::{Deserialize, Serialize};
    
    #[derive(Debug,FromRow,Deserialize,Serialize)]
    pub struct Task {
        pub id: i64,
        pub value: String,
        pub completed: i8,
        pub date_completed: String,
        pub project_id: i64,  
    }
    
    #[tauri::command(rename_all = "snake_case")]
    pub async fn get_tasks() -> Result<Vec<Task>, String>{
        let url = "sqlite://sqlite.db";
    
        let pool = sqlx::sqlite::SqlitePool::connect(url).await.expect("unable to connect");
    
        let sql = "SELECT * FROM tasks";
    
        let query = sqlx::query_as::<_, Task>(sql);
        
        let response = query.fetch_all(&pool).await.expect("unable to list tasks");
    
        pool.close().await;
    
        Ok(response)
    }
    ```
- [ ]  Add `commands.rs` to `main.rs` as a module and add the following to `tauri::Builder::default()`
    ```rust
    // Prevents additional console window on Windows in release, DO NOT REMOVE!!
    #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
    
    mod db;
    mod commands;
    
    #[async_std::main]
    async fn main() {
      db::init().await;
    
      tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
          commands::get_tasks,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    }
    ```
- [ ]  Create a set of records in each table to test the functionality of the application. There are a number of different ways to achieve this, I chose to create a function that will perform this operation as part of the `db::init()` fn, and by commenting it out or uncommenting it when necessary *(This fn would need to be deleted before building and deploying)*
    ```rust
    // Check for DB, create if non existent
    pub async fn init() {
        if !Sqlite::database_exists(DB_URL).await.unwrap_or(false) {
            match Sqlite::create_database(DB_URL).await {
                Ok(_) => println!("Create db success"),
                Err(error) => panic!("error: {}", error),
            }
        } else {
            println!("Database already exists");
        }
    
        create_schema().await;
        
        // Uncomment the fn below and run to make some records for testing
        insert_dev_records().await;
    }
    
    // Create schema
    // ...
    
    // Create some test records in each table
    async fn insert_dev_records() {
        let pool = SqlitePool::connect(DB_URL).await.expect("unable to connect");
        let sql = "
            INSERT INTO projects (name)
            VALUES ('Awesome Current Product'), ('Top Secret Product'), ('Super Top Secret Product');
    
            INSERT INTO tasks (value, completed, date_completed, project_id)
            VALUES ('Design the UI',                    0,      NULL,                   3),
                   ('Design DB Schema',                 0,      NULL,                   3),
                   ('Build prototype app',              0,      NULL,                   3),
                   ('Design a cool logo',               1,      DATE('2023-04-22'),     3),
                   ('Refactor component lib',           0,      NULL,                   2),
                   ('Add input sanitization to ipc',    0,      NULL,                   2),
                   ('Security audit testing for v1.5',  0,      NULL,                   1),
                   ('Add Dark Mode',                    1,      DATE('2023-04-20'),     1),
                   ('Fix UI glitch',                    1,      DATE('2023-04-20'),     1);
        ";
        
        let query = sqlx::query(&sql);
        let result = query.execute(&pool).await.unwrap();
        println!("Create Records result: {:?}", result);   
        pool.close().await;
    }
    ```
</details>

---

<details>
    <summary><h2>Step 5: Build Frontend</h2></summary>
    
- [ ]  Within `/src/lib` folder, create 2 new files called `table.svelte` and `index.ts` with the following contents
    ```tsx
    <script lang="ts">
        import { invoke } from '@tauri-apps/api';
        import { onMount } from 'svelte';
       
        let fields: any = [];
    
        let records: any = [];
    
            function getFields(response: any){
                 let objs = Object.values(response);  // returns [ Object, Object, ... ]
                 let firstObj: any = objs[0];
                 fields = Object.keys(firstObj);
            }
    
            function getData(response: any){
                 let objs = Object.values(response); // returns [ Object, Object, ... ]
                 records = Object.values(objs);
            }
       
            onMount(async () => {
                 const resp = await invoke("get_tasks");
                 getFields(resp);
                 getData(resp);
            });
    </script>
       
    <div>
        <h1>Tasks</h1>
        <table>
            <thead>
                <tr class="headings">
    
                {#each fields as field}
                    <th>{field}</th>
                {/each}
    
                </tr>
            </thead>
            
            <tbody>
    
            {#each records as record}
                <tr>
    
                {#each Object.values(record) as value}
                     <td>{value}</td>
                {/each}
                
                </tr>
            {/each}
    
            </tbody>
        </table>
    </div>
    ```
    ```tsx
    // place files you want to import through the `$lib` alias in this folder.
    export { default } from "./table.svelte";
    ```
    <blockquote>
    <details>
        <summary>Optional Table styles </summary>    
        
    ```tsx
    <style>
        div {
            display: flex;
            flex-direction: column;
            padding: 20px;
        }    
    
        h1 {
            font-family: "Avenir Next";
        }
          
        table {
            display: table;
        }
    
        tr {
            font-family: "Avenir Next";
            border-bottom: 1px solid #4D4D4D;
        }
    
        th {
            text-align: start;
            padding: 8px;
            background-color: #4D4D4D;
            color: white;
            font-size: 1.05em;
        }
    
        td {
            padding: 5px 0px 5px 10px;
        }
    
        tr:nth-child(even) {
            background-color: #caffef;
        }
    </style>
    ```
       
    </details>
    </blockquote>  
- [ ]  Within `src/routes`, open the `+page.svelte` file and change it’s contents to the following
    ```tsx
    <script>
    	import Table from "$lib/table.svelte";
    
    </script>
    
    <h1>My Tauri + SvelteKit + SQLite App</h1>
    
    <Table />
    ```    
</details>

---

<details>
    <summary>
    <h2>
        Step 6: Test the App
    </h2>
    </summary>

- [ ]  Make sure everything is saved then run the following command in the terminal
    ```shell
    pnpm tauri dev
    ```
    ![tauri-sveltekit-sqlx_screenshot](https://github.com/Lmedmo/Tauri-SvelteKit-SQLite/assets/102483463/8a67c736-c59c-4fb1-bc3c-d11e540aef82)

    > **🎉 Congrats!**
    > You have built a very basic full-stack desktop app with an embedded database. There’s a lot of potential with how this template can be applied and there are plenty of concepts, best practices, and steps that you should be aware of that are not discussed/included in this basic example, so do your own research regarding the technologies used here, experiment with new things, and most importantly have fun — coding is cool😎
    > PS: If you like this or have suggestions let me know!! I’m still pretty new to Rust and backend web development, and I’m always looking for ways to improve my skills as a developer (Rust, Svelte, SQLite/DBs, etc.). Furthermore, I like knowing when I do a good job so that I can flex or if my code is trash so I can fix it and then flex (Just kidding, maybe).

</details>
