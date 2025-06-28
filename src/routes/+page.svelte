<script lang="ts">
  import { read } from "$app/server";
  import { open } from "@tauri-apps/plugin-dialog";
  import { readTextFile, writeTextFile } from "@tauri-apps/plugin-fs";

  import { invoke } from "@tauri-apps/api/core";

  invoke("greet", { name: "Ferdinand" }).then((response) => {
    console.log("Response from Rust:", response);
  });

  import { Button, Spinner } from "flowbite-svelte";

  let analyzing: boolean = $state(false);

  const SECURITY_COEFF = 3.08;
  const WINDOW_SIZE = 7;
  const DELIVERY_DURATION = 30;
  const ORDER_FREQUENCY = 7;

  async function open_file() {
    return await open({
      multiple: false,
      directory: false,
      filters: [
        {
          name: "Sélectionne un fichier CSV",
          extensions: ["csv"],
        },
      ],
    });
  }

  function parse_date(date_string: string): Date {
    return new Date(date_string + "T00:00:00.000Z");
  }

  function parse_csv(file_content: string) {
    // normalize line endings
    file_content = file_content.replace(/\r\n/g, "\n");

    // remove first 10 lines of crap
    const lines = file_content.split("\n").slice(10);

    // find columns indices using header
    const headers = lines[0].split(",");
    const datetime_label_index = headers.indexOf("Date");
    const product_reference_index = headers.indexOf("Product Code");
    const product_name_index = headers.indexOf("Product Name");
    const quantity_label_index = headers.indexOf("Quantity");

    // parse rows to objects
    return lines
      .slice(1, -1)
      .map((line) => {
        const columns = line.split(",");
        // skip if columns[datetime_label_index] is "Unknown"
        if (columns[datetime_label_index] === "Unknown") {
          return null;
        }
        return {
          datetime: parse_date(columns[datetime_label_index]),
          product_reference: columns[product_reference_index],
          product_name: columns[product_name_index].replace(/^"+|"+$/g, ""),
          quantity: parseFloat(columns[quantity_label_index]),
        };
      })
      .filter((row) => row !== null)
      .sort((a, b) => a.datetime.getTime() - b.datetime.getTime());
  }

  function group_by_reference(data: { product_reference: string }[]) {
    let grouped: { [key: string]: any } = {};
    data.forEach((item) => {
      if (!grouped[item.product_reference]) {
        grouped[item.product_reference] = [];
      }
      grouped[item.product_reference].push(item);
    });
    return grouped;
  }

  function analyze_group(
    group: {
      datetime: Date;
      product_reference: string;
      product_name: string;
      quantity: number;
    }[],
  ): any[] | null {
    const start_date = group[0].datetime;
    const end_date = group[group.length - 1].datetime;
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
        const future_date = new Date(item.datetime);
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

    // excel formulas to compute the values
    // for row in range(1, len(out_df) + 1):
    //     # min_stock = average * (delivery_duration + order_frequency)
    //     worksheet.write(row, 7, f"=F{row + 1}*(C{row + 1}+E{row + 1})")
    //     # security_stock = std_dev * security_coeff * sqrt(delivery_duration + order_frequency)
    //     worksheet.write(
    //         row, 8, f"=D{row + 1}*G{row + 1}*SQRT(C{row + 1}+E{row + 1})"
    //     )
    //     # threshold_stock = ceil(min_stock + security_stock)
    //     worksheet.write(row, 9, f"=ROUNDUP(H{row + 1}+I{row + 1},0)")
    //     # threshold_stock - stock
    //     worksheet.write(row, 11, f"=J{row + 1}-K{row + 1}")
    //     # order_quantity = ceil(delta_from_target + min_stock) if delta_from_target >= 0 else 0
    //     worksheet.write(
    //         row, 12, f"=IF(L{row + 1}>=0,ROUNDUP(H{row + 1}+L{row + 1},0),0)"
    //     )

    const min_stock = mean * (DELIVERY_DURATION + ORDER_FREQUENCY);
    const security_stock =
      stddev * SECURITY_COEFF * Math.sqrt(DELIVERY_DURATION + ORDER_FREQUENCY);
    const threshold_stock = Math.ceil(min_stock + security_stock);
    const stock = 0;
    const delta_from_target = threshold_stock - stock;
    const order_quantity = Math.ceil(
      delta_from_target >= 0 ? delta_from_target + min_stock : 0,
    );

    return [
      // product
      group[0].product_reference,
      group[0].product_name,
      // parameters
      DELIVERY_DURATION,
      SECURITY_COEFF,
      ORDER_FREQUENCY,
      // data observations
      mean,
      stddev,
      // computed values
      min_stock,
      security_stock,
      threshold_stock,
      stock,
      delta_from_target,
      order_quantity,
    ];
  }

  function analyze_results(file_content: string): string | null {
    const data = parse_csv(file_content);
    const start_date = data[0].datetime;
    const end_date = data[data.length - 1].datetime;
    const total_duration =
      (end_date.getTime() - start_date.getTime()) / (1000 * 60 * 60 * 24);
    if (total_duration <= WINDOW_SIZE) return null;

    const grouped_data = group_by_reference(data);
    const results = [
      [
        "Référence",
        "Désignation",
        "Durée de Livraison (jours)",
        "Coefficient de Sécurité (99.9%)",
        "Fréquence de Commande (jours)",
        "Consommation Moyenne / jour",
        "Ecart-type",
        "Stock Minimum",
        "Stock de Sécurité",
        "Seuil de Commande",
        "Stock Actuel",
        "Delta de Stock",
        "Quantité à Commander",
      ].join(","),
    ];
    for (const reference in grouped_data) {
      const group = grouped_data[reference];
      const out = analyze_group(group);
      if (out === null) continue;
      results.push(out.join(","));
    }
    const out = results.join("\n");
    return out;
  }

  async function analyze() {
    const file_path = await open_file();
    if (file_path === null) return;
    if (!file_path.toLowerCase().endsWith(".csv")) return;

    analyzing = true;

    const file_content = await readTextFile(file_path);
    if (file_content === null) return;
    const out = analyze_results(file_content);
    if (out === null) return;
    invoke("excel", {
      content: out,
      filename: file_path.replace(/\.csv$/, "_output.xlsx"),
    }).then(() => (analyzing = false));
  }
</script>

<main class="h-screen w-full flex flex-col items-center justify-center p-8">
  <h1 class="b-4 text-3xl font-extrabold text-gray-900 uppercase mb-10">
    StockWise
  </h1>

  {#if !analyzing}
    <div class="mb-10">
      <Button color="light" class="cursor-pointer" onclick={analyze}>
        Analyse le fichier Excel
      </Button>
    </div>
  {:else}
    <p class="mb-3">Analyse en cours</p>
    <Spinner />
  {/if}
</main>
