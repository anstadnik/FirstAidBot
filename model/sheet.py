from typing import Optional

import pandas as pd

# These are the "links" to the sheets. Below is the full table and the test
# table

# Prod
sheet_id = "Миші з'їли"

# Test
# sheet_id = "1seobblWaZXSu82yf3CnanIps26vCv3QARo75-sAC2KQ"
sheet_name = "Sheet1"
url = f"https://docs.google.com/spreadsheets/d/{sheet_id}/gviz/tq?tqx=out:csv&sheet={sheet_name}"

FiniteStateOptions = dict[str, Optional["FiniteState"]] | None
FiniteState = tuple[str, FiniteStateOptions]


def fill_item(
    df: pd.DataFrame, parent_key: str
) -> None | dict[str, Optional[dict[str, FiniteState]]]:
    ret = {}
    for _, row in df[df["hierarchy"].str.startswith(parent_key)].iterrows():
        if '.' in row['hierarchy'].replace(parent_key, ''):
            continue
        key = f'{row["hierarchy"]}.'
        ret[row["option"]] = (row["answer"], fill_item(df, key))
    return ret or None


def get_data() -> FiniteState:
    df = pd.DataFrame(pd.read_csv(url, dtype=str))
    data = {}
    for _, row in df[~df["hierarchy"].str.contains(".", regex=False)].iterrows():
        key = f'{row["hierarchy"]}.'
        data[row["option"]] = (row["answer"], fill_item(df, key))
    return ("Що трапилось?", data)


def main():
    data = get_data()
    __import__("pprint").pprint(data)


if __name__ == "__main__":
    main()
