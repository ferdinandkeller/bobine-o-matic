import sys
import getopt
import json
import pandas as pd
from math import sqrt, ceil
from pyxlsb import open_workbook


def clean_dataframe(df: pd.DataFrame) -> pd.DataFrame:
    """Clean the input DataFrame to a usable format."""
    # rename columns
    product_reference = "Product Code"
    product_name = "Product Name"
    quantity_label = "Quantity"
    datetime_label = "Date"
    # keep only required columns
    df = df[[product_reference, product_name, datetime_label, quantity_label]]
    df = df.rename(
        columns={
            product_reference: "Reference",
            product_name: "Name",
            datetime_label: "Date",
            quantity_label: "Quantity",
        }
    )
    # remove rows with Unknown Date
    df = df[df["Date"] != "Unknown"]
    # convert the date column to datetime
    df["Date"] = pd.to_datetime(df["Date"])
    # convert the quantity column to float
    df["Quantity"] = df["Quantity"].astype(float)
    df = df.sort_values(["Reference", "Date"])
    return df


def main(
    file_path: str,
    security_coeff: float,
    window_size: int,
    delivery_duration: int,
    order_frequency: int,
    download: bool,
):
    # read excel by ignoring the first 12 rows and the total row
    df = pd.read_excel(file_path, skiprows=12, dtype=str).iloc[:-1]
    df = clean_dataframe(df)

    data_start = df["Date"].min()
    data_end = df["Date"].max()

    if data_end - data_start < pd.Timedelta(days=window_size):
        raise ValueError("Not enough data to calculate rolling average.")

    results = {}
    name_index = {}

    for reference, group in df.groupby("Reference"):
        # add the name to the name index
        name_index[reference] = group["Name"].iloc[0]
        # aggregate duplicate dates by summing quantities
        group = group.groupby("Date").agg({"Quantity": "sum"}).reset_index()
        # set datetime as index for resampling operations
        group = group.set_index("Date")
        # set the min and max dates for this reference
        start_date = group.index.min()
        end_date = data_end
        # if not enough days, ignore this reference
        if (end_date - start_date).days < window_size:
            continue
        # create a date range from start to end
        date_range = pd.date_range(start=start_date, end=end_date, freq="D")
        # reindex to ensure we have one row per day (filling missing days with 0)
        daily_data = group.reindex(date_range, fill_value=0)
        # calculate 7-day rolling average (window=7)
        rolling_avg = (
            daily_data["Quantity"].rolling(window=window_size, min_periods=1).mean()
        )
        # store the result in our dictionary
        results[reference] = rolling_avg

    out = []

    for reference in results:
        name = name_index[reference]
        df = pd.DataFrame(results[reference])

        average = float(df.mean().iloc[0])
        std_dev = float(df.std(ddof=0).iloc[0])

        min_stock = 0
        security_stock = 0
        threshold_stock = 0
        stock = 0
        delta_from_target = 0
        order_quantity = 0
        out.append(
            [
                # product
                reference,
                name,
                # parameters
                delivery_duration,
                security_coeff,
                order_frequency,
                # data observations
                average,
                std_dev,
                # computed values
                min_stock,
                security_stock,
                threshold_stock,
                stock,
                delta_from_target,
                order_quantity,
            ]
        )

    if not download:
        print(json.dumps(out))
        return

    out_df = pd.DataFrame(
        out,
        columns=[
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
        ],
    )
    output_file = file_path.replace(".xlsx", "_output.xlsx")

    with pd.ExcelWriter(output_file, engine="xlsxwriter") as writer:
        out_df.to_excel(writer, sheet_name="Data", index=False)
        worksheet = writer.sheets["Data"]

        # delivery duration
        worksheet.set_column("C:C", None, None, {"hidden": True})
        # security coeff
        worksheet.set_column("D:D", None, None, {"hidden": True})
        # order frequency
        worksheet.set_column("E:E", None, None, {"hidden": True})
        # min stock
        worksheet.set_column("H:H", None, None, {"hidden": True})
        # security stock
        worksheet.set_column("I:I", None, None, {"hidden": True})
        # delta from target
        worksheet.set_column("L:L", None, None, {"hidden": True})

        for row in range(1, len(out_df) + 1):
            # min_stock = average * (delivery_duration + order_frequency)
            worksheet.write(row, 7, f"=F{row + 1}*(C{row + 1}+E{row + 1})")
            # security_stock = std_dev * security_coeff * sqrt(delivery_duration + order_frequency)
            worksheet.write(
                row, 8, f"=D{row + 1}*G{row + 1}*SQRT(C{row + 1}+E{row + 1})"
            )
            # threshold_stock = ceil(min_stock + security_stock)
            worksheet.write(row, 9, f"=ROUNDUP(H{row + 1}+I{row + 1},0)")
            # threshold_stock - stock
            worksheet.write(row, 11, f"=J{row + 1}-K{row + 1}")
            # order_quantity = ceil(delta_from_target + min_stock) if delta_from_target >= 0 else 0
            worksheet.write(
                row, 12, f"=IF(L{row + 1}>=0,ROUNDUP(H{row + 1}+L{row + 1},0),0)"
            )


if __name__ == "__main__":
    arguments = getopt.getopt(
        sys.argv[1:],
        "",
        [
            "file-path=",
            "security-coeff=",
            "window-size=",
            "delivery-duration=",
            "order-frequency=",
            "download=",
        ],
    )[0]
    file_path = None
    security_coeff = None
    window_size = None
    delivery_duration = None
    order_frequency = None
    download = False

    for opt, arg in arguments:
        if opt == "--file-path":
            file_path = arg
        elif opt == "--security-coeff":
            security_coeff = float(arg)
        elif opt == "--window-size":
            window_size = int(arg)
        elif opt == "--delivery-duration":
            delivery_duration = int(arg)
        elif opt == "--order-frequency":
            order_frequency = int(arg)
        elif opt == "--download":
            download = arg.lower() == "true"

    if file_path is None:
        raise ValueError("File path is required.")
    if security_coeff is None:
        raise ValueError("Security coefficient is required.")
    if window_size is None:
        raise ValueError("Window size is required.")
    if delivery_duration is None:
        raise ValueError("Delivery duration is required.")
    if order_frequency is None:
        raise ValueError("Order frequency is required.")

    main(
        file_path,
        security_coeff,
        window_size,
        delivery_duration,
        order_frequency,
        download,
    )
