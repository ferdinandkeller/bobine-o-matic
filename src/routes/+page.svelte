<script lang="ts">
  import { Command } from "@tauri-apps/plugin-shell";
  import { open } from "@tauri-apps/plugin-dialog";

  import {
    Button,
    Table,
    TableBody,
    TableBodyCell,
    TableBodyRow,
    TableHead,
    TableHeadCell,
  } from "flowbite-svelte";

  let file_path: string | null = $state(null);
  let analyzed_data: any | null = $state(null);

  async function analyze_results(path: string) {
    const command = Command.sidecar("src-python/dist/main/main", [
      "--file-path",
      path,
      "--security-coeff",
      "3.08",
      "--delivery-duration",
      "1",
      "--download",
      "false",
    ]);
    const output = await command.execute();
    analyzed_data = JSON.parse(output.stdout.trim());
  }

  async function download_results(path: string) {
    const command = Command.sidecar("src-python/dist/main/main", [
      "--file-path",
      path,
      "--security-coeff",
      "3.08",
      "--delivery-duration",
      "1",
      "--download",
      "true",
    ]);
    await command.execute();
    console.log("Download complete");
  }

  async function open_file() {
    const file = await open({
      multiple: false,
      directory: false,
    });
    if (file !== null) {
      file_path = file as string;
    }
  }

  async function analyze() {
    await open_file();
    if (file_path === null) return;
    await analyze_results(file_path);
  }

  async function download() {
    if (analyzed_data === null) await analyze();
    if (analyzed_data === null || file_path === null) return;
    await download_results(file_path);
  }
</script>

<main class="h-screen w-full flex flex-col items-center justify-center p-8">
  <h1 class="b-4 text-3xl font-extrabold text-gray-900 uppercase mb-10">
    StockSync
  </h1>

  <div class="mb-10">
    <Button color="light" class="cursor-pointer" onclick={analyze}
      >Analyze Excel File</Button
    >
    <Button color="light" class="cursor-pointer" onclick={download}
      >Download Results</Button
    >
  </div>

  {#if analyzed_data !== null}
    <!-- <h2>Analyzed Data</h2> -->
    <!-- <table>
      <thead>
        <tr>
          <th>reference</th>
          <th>threshold</th>
        </tr>
      </thead>
      <tbody>
        {#each Object.entries(analyzed_data) as [reference, threshold]}
          <tr>
            <td>{reference}</td>
            <td>{threshold}</td>
          </tr>
        {/each}
      </tbody>
    </table> -->
    <Table striped={true} hoverable={true} class="w-1/2">
      <TableHead>
        <TableHeadCell>Reference</TableHeadCell>
        <TableHeadCell>Threshold</TableHeadCell>
      </TableHead>
      <TableBody>
        {#each Object.entries(analyzed_data) as [reference, threshold]}
          <TableBodyRow>
            <TableBodyCell>{reference}</TableBodyCell>
            <TableBodyCell>{threshold}</TableBodyCell>
          </TableBodyRow>
        {/each}
      </TableBody>
    </Table>
  {/if}
</main>
