<script lang="ts">
  import { Command } from "@tauri-apps/plugin-shell";

  async function run_python(): Promise<number[]> {
    const command = Command.sidecar("../src-python/dist/main");
    const output = await command.execute();
    const result = output.stdout.trim();
    const data = JSON.parse(result);
    return data;
  }

  async function data(): Promise<string> {
    const data = await run_python();
    return JSON.stringify({ data: data }, null, 4);
  }
</script>

<main class="container">
  <h1>Welcome to Tauri + Svelte</h1>

  {#await data()}
    <p>Loading Python data ...</p>
  {:then data}
    <pre><code>{data}</code></pre>
  {:catch someError}
    <p>Python error : {someError}.</p>
  {/await}
</main>
