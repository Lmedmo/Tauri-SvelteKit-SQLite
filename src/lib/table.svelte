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
   
   <style>
        div{
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