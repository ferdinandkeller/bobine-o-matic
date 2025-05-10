import sys
import getopt
import json
import pandas as pd
from math import sqrt


def clean_dataframe(df: pd.DataFrame) -> pd.DataFrame:
    """Clean the input DataFrame to a usable format."""
    # rename columns
    reference_label = "Product Code"
    quantity_label = "Quantity"
    datetime_label = "Last Sale Date"
    df = df.rename(
        columns={
            reference_label: "Reference",
            quantity_label: "Quantity",
            datetime_label: "Date",
        }
    )
    # keep only the renamed columns and drop the rest
    df = df[["Reference", "Quantity", "Date"]]
    # convert the date column to datetime
    df["Date"] = pd.to_datetime(df["Date"])
    # convert the quantity column to float
    df["Quantity"] = df["Quantity"].astype(float)
    df = df.sort_values(["Reference", "Date"])
    return df


def main(file_path: str, security_coeff: float, delivery_duration: int, download: bool):
    # read excel by ignoring the first 12 rows and the total row
    df = pd.read_excel(file_path, skiprows=12).iloc[:-1]
    df = clean_dataframe(df)
    print(df.head())
    # save to excel
    df.to_excel(file_path.replace(".xlsx", "_output.xlsx"), index=False)

    # results = {}

    # for reference, group in df.groupby(reference_label):
    #     # set datetime as index for resampling operations
    #     group = group.set_index(datetime_label)
    #     # set the min and max dates for this reference
    #     start_date = group.index.min()
    #     end_date = group.index.max()
    #     # create a date range from start to end
    #     date_range = pd.date_range(start=start_date, end=end_date, freq="D")
    #     # reindex to ensure we have one row per day (filling missing days with NaN)
    #     daily_data = group.reindex(date_range)
    #     # calculate 7-day rolling average (window=7)
    #     # min_periods=1 allows calculation even when fewer than 7 days of data are available
    #     rolling_avg = daily_data[quantity_label].rolling(window=7, min_periods=1).mean()
    #     # store the result in our dictionary
    #     results[reference] = rolling_avg

    # print(results[list(results.keys())[0]].head())

    # out = {}

    # for reference in results:
    #     df = pd.DataFrame(results[reference])
    #     average = df.mean()
    #     std_dev = df.std()
    #     stock_mini = average * delivery_duration
    #     stock_security = std_dev * security_coeff * sqrt(delivery_duration)
    #     stock_trigger = stock_mini + stock_security
    #     out[reference] = round(stock_trigger.iloc[0])

    # if not download:
    #     print(json.dumps(out))
    #     return

    # out_df = pd.DataFrame(out.items(), columns=["Reference", "Stock Trigger"])
    # output_file = file_path.replace(".xlsx", "_output.xlsx")
    # out_df.to_excel(output_file, index=False)


if __name__ == "__main__":
    arguments = getopt.getopt(
        sys.argv[1:],
        "",
        ["file-path=", "security-coeff=", "delivery-duration=", "download="],
    )[0]
    file_path = None
    security_coeff = None
    delivery_duration = None
    download = False

    for opt, arg in arguments:
        if opt == "--file-path":
            file_path = arg
        elif opt == "--security-coeff":
            security_coeff = float(arg)
        elif opt == "--delivery-duration":
            delivery_duration = int(arg)
        elif opt == "--download":
            download = arg.lower() == "true"

    if file_path is None:
        raise ValueError("File path is required.")
    if security_coeff is None:
        raise ValueError("Security coefficient is required.")
    if delivery_duration is None:
        raise ValueError("Delivery duration is required.")

    main(file_path, security_coeff, delivery_duration, download)
