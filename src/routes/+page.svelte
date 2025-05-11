<script lang="ts">
  import { Command } from "@tauri-apps/plugin-shell";
  import { open } from "@tauri-apps/plugin-dialog";

  import {
    Button,
    Spinner,
    // Table,
    // TableBody,
    // TableBodyCell,
    // TableBodyRow,
    // TableHead,
    // TableHeadCell,
  } from "flowbite-svelte";

  let file_path: string | null = $state(null);
  let analyzing: boolean = $state(false);
  // let analyzed_data: any | null = $state(null);

  // async function analyze_results(path: string) {
  //   console.log("Analyzing results..." + path);
  //   const command = Command.sidecar("src-python/dist/main", [
  //     "--file-path",
  //     path,
  //     "--security-coeff",
  //     "3.08",
  //     "--window-size",
  //     "7",
  //     "--delivery-duration",
  //     "30",
  //     "--download",
  //     "false",
  //   ]);
  //   const output = await command.execute();
  //   analyzed_data = JSON.parse(output.stdout.trim());
  // }

  async function download_results(path: string) {
    const command = Command.sidecar("src-python/dist/main", [
      "--file-path",
      path,
      "--security-coeff",
      "3.08",
      "--window-size",
      "7",
      "--delivery-duration",
      "30",
      "--order-frequency",
      "7",
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
    analyzing = true;
    // await analyze_results(file_path);
    await download_results(file_path);
    analyzing = false;
  }

  // async function download() {
  //   if (analyzed_data === null) await analyze();
  //   if (analyzed_data === null || file_path === null) return;
  //   await download_results(file_path);
  // }
</script>

<main class="h-screen w-full flex flex-col items-center justify-center p-8">
  <h1 class="b-4 text-3xl font-extrabold text-gray-900 uppercase mb-10">
    StockSync
  </h1>

  {#if !analyzing}
    <div class="mb-10">
      <Button color="light" class="cursor-pointer" onclick={analyze}
        >Analyze Excel File</Button
      >
      <!-- <Button color="light" class="cursor-pointer" onclick={download}
        >Download Results</Button
        > -->
    </div>
  {:else}
    <p class="mb-3">Analyzing your file...</p>
    <Spinner />
  {/if}

  <!-- {#if analyzed_data !== null}
    <Table striped={true} hoverable={true} class="w-1/2">
      <TableHead>
        <TableHeadCell>Reference</TableHeadCell>
        <TableHeadCell>Threshold</TableHeadCell>
        <TableHeadCell>Écart-Type</TableHeadCell>
      </TableHead>
      <TableBody>
        {#each analyzed_data as [reference, threshold, stddev]}
          <TableBodyRow>
            <TableBodyCell>{reference}</TableBodyCell>
            <TableBodyCell>{threshold}</TableBodyCell>
            <TableBodyCell>{stddev}</TableBodyCell>
          </TableBodyRow>
        {/each}
      </TableBody>
    </Table>
  {/if} -->
</main>
