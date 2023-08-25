<script>
    import { invoke } from "@tauri-apps/api/tauri";

    let dir = "./";
    let files = "";

    async function listFiles() {
        files = await invoke("list_files", { dir });
    }
</script>

<button on:click={listFiles}> List files </button>
<div>
    {files}
</div>
