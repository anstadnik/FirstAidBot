import pandas as pd

# Prod
sheet_id = "Миші з'їли"

# Test
sheet_id = "1seobblWaZXSu82yf3CnanIps26vCv3QARo75-sAC2KQ"
sheet_name = "Sheet1"
url = f"https://docs.google.com/spreadsheets/d/{sheet_id}/gviz/tq?tqx=out:csv&sheet={sheet_name}"


def fill_item(df: pd.DataFrame, parent_key: str):
    ret = {}
    for _, row in df[df["hierarchy"].str.startswith(parent_key)].iterrows():
        key = f'{row["hierarchy"]}.'
        ret[row["option"]] = (row["answer"], fill_item(df, key))
    return ret


def get_data():
    df = pd.DataFrame(pd.read_csv(url, dtype=str))
    data = {}
    for _, row in df[~df["hierarchy"].str.contains(".", regex=False)].iterrows():
        key = f'{row["hierarchy"]}.'
        data[row["option"]] = (row["answer"], fill_item(df, key))
    return data


def main():
    data = get_data()
    print(data)


if __name__ == "__main__":
    main()
