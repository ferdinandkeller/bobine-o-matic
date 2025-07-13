<script lang="ts">
  import { open } from "@tauri-apps/plugin-dialog";
  import { openPath } from "@tauri-apps/plugin-opener";
  import { invoke } from "@tauri-apps/api/core";

  import { Button, Spinner } from "flowbite-svelte";

  let analyzing: boolean = $state(false);

  const SECURITY_COEFF = 3.08;
  const WINDOW_SIZE = 7;
  const DELIVERY_DURATION = 30;
  const ORDER_FREQUENCY = 7;

  async function select_excel_file() {
    return await open({
      multiple: false,
      directory: false,
      filters: [
        {
          name: "Sélectionne un fichier Excel",
          extensions: ["xlsx"],
        },
      ],
    });
  }

  function parse_date(date_string: string): Date {
    return new Date(date_string + "T00:00:00.000Z");
  }

  function group_by_reference(data: { product_code: string }[]) {
    let grouped: { [key: string]: any } = {};
    data.forEach((item) => {
      if (!grouped[item.product_code]) {
        grouped[item.product_code] = [];
      }
      grouped[item.product_code].push(item);
    });
    return grouped;
  }

  function analyze_group(group: DataRow[]) {
    const start_date = group[0].date;
    const end_date = group[group.length - 1].date;
    const total_duration =
      (end_date.getTime() - start_date.getTime()) / (1000 * 60 * 60 * 24);
    if (total_duration <= WINDOW_SIZE) return null;
    // perform a rolling average over the group, with a window of 7 days
    const rolling_average: { [date: string]: number } = {};
    // populate with the days in the range with zeros
    let date = new Date(start_date);
    while (date <= end_date) {
      rolling_average[date.toISOString().split("T")[0]] = 0;
      date.setUTCDate(date.getUTCDate() + 1);
    }
    // for each date add the quantity +7 days after the event
    for (const item of group) {
      for (let i = 0; i < WINDOW_SIZE; i++) {
        const future_date = new Date(item.date);
        future_date.setUTCDate(future_date.getUTCDate() + i);
        const future_date_str = future_date.toISOString().split("T")[0];
        if (rolling_average[future_date_str] !== undefined) {
          rolling_average[future_date_str] += item.quantity;
        }
      }
    }
    // divide for each date by 7
    for (const date_str in rolling_average) {
      rolling_average[date_str] /= WINDOW_SIZE;
    }
    // remoe first 6 days
    for (let i = 0; i < WINDOW_SIZE - 1; i++) {
      const date_to_remove = new Date(start_date);
      date_to_remove.setUTCDate(date_to_remove.getUTCDate() + i);
      const date_str = date_to_remove.toISOString().split("T")[0];
      delete rolling_average[date_str];
    }
    // compute mean and standard deviation
    const values = Object.values(rolling_average);
    const mean = values.reduce((a, b) => a + b, 0) / values.length;
    const stddev = Math.sqrt(
      values.reduce((a, b) => a + Math.pow(b - mean, 2), 0) / values.length,
    );

    // const min_stock = mean * (DELIVERY_DURATION + ORDER_FREQUENCY);
    // const security_stock =
    //   stddev * SECURITY_COEFF * Math.sqrt(DELIVERY_DURATION + ORDER_FREQUENCY);
    // const threshold_stock = Math.ceil(min_stock + security_stock);
    // const stock = 0;
    // const delta_from_target = threshold_stock - stock;
    // const order_quantity = Math.ceil(
    //   delta_from_target >= 0 ? delta_from_target + min_stock : 0,
    // );

    return {
      // product
      reference: group[0].product_code,
      designation: group[0].product_name,
      product_brand: group[0].product_brand,
      // parameters
      delivery_duration: DELIVERY_DURATION.toString().replace(".", ","),
      security_coeff: SECURITY_COEFF.toString().replace(".", ","),
      order_frequency: ORDER_FREQUENCY.toString().replace(".", ","),
      // data observations
      average_consumption: mean.toString().replace(".", ","),
      std_dev: stddev.toString().replace(".", ","),
    };
  }

  type DataRow = {
    date: Date;
    quantity: number;
    product_code: string;
    product_name: string;
    product_brand: string;
  };

  function analyze_results(data: DataRow[]) {
    const start_date = data[0].date;
    const end_date = data[data.length - 1].date;
    const total_duration =
      (end_date.getTime() - start_date.getTime()) / (1000 * 60 * 60 * 24);
    if (total_duration <= WINDOW_SIZE) return null;

    const grouped_data = group_by_reference(data);

    const results: any[] = [];

    for (const reference in grouped_data) {
      const group = grouped_data[reference];
      const out = analyze_group(group);
      if (out === null) continue;
      results.push(out);
    }

    return JSON.stringify(results, null, 0);
  }

  async function analyze() {
    const file_path = await select_excel_file();

    if (file_path === null) return;
    if (!file_path.toLowerCase().endsWith(".xlsx")) return;

    analyzing = true;

    const data = JSON.parse(await invoke("excel2", { filepath: file_path }))
      .map((r: DataRow | { date: string }) => {
        r.date = parse_date(r.date as string);
        return r;
      })
      .sort((a: DataRow, b: DataRow) => a.date.getTime() - b.date.getTime());

    const out = analyze_results(data);
    if (out === null) return;

    let new_path = file_path.replace(/\.xlsx$/, "_output.xlsx");

    invoke("excel", {
      content: out,
      filename: new_path,
    }).then(async () => {
      analyzing = false;
      await openPath(new_path);
    });
  }
</script>

<main class="h-screen w-full flex flex-col items-center justify-center p-8">
  <h1 class="b-4 text-3xl font-extrabold text-gray-900 mb-10">BobinOMatic</h1>

  {#if !analyzing}
    <div class="mb-10">
      <Button color="light" class="cursor-pointer" onclick={analyze}>
        Analyser un fichier Excel
      </Button>
    </div>
  {:else}
    <p class="mb-3">Analyse en cours</p>
    <Spinner />
  {/if}
</main>
