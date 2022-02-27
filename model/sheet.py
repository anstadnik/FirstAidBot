import re
# import requests
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

OptionalUrlAndStr = tuple[str, str] | str
FiniteStateOptions = dict[str, Optional["FiniteState"]] | None
FiniteState = tuple[OptionalUrlAndStr, FiniteStateOptions]


def add_link(answer: str, link):
    if isinstance(link, str):
        if "file/d" in link:
            link = re.sub(r".*file/d/", "", link)
            link = re.sub(r"/.*", "", link)
            link = f"https://drive.google.com/uc?id={link}"
            # r = requests.get(link)
            # link = r.url
        return link, answer
    return answer


def fill_item(
    df: pd.DataFrame, parent_key: str
) -> None | dict[str, Optional[dict[str, FiniteState]]]:
    ret = {}
    for _, row in df[df["hierarchy"].str.startswith(parent_key)].iterrows():
        if "." in row["hierarchy"].replace(parent_key, "", 1):
            continue
        key = f'{row["hierarchy"]}.'
        answer = add_link(row["answer"], row["link"])
        ret[row["option"]] = (answer, fill_item(df, key))
    return ret or None


def get_data() -> FiniteState:
    df = pd.DataFrame(pd.read_csv(url, dtype=str))
    data = {}
    for _, row in df[~df["hierarchy"].str.contains(".", regex=False)].iterrows():
        key = f'{row["hierarchy"]}.'
        answer = add_link(row["answer"], row["link"])
        data[row["option"]] = (answer, fill_item(df, key))
    return ("Що трапилось?", data)

def disp(data: FiniteState, indent=0):
    msg, options = data
    # print(msg)
    if options:
        for msg, options in options.items():
            print(" " * indent, msg)
            if options:
                disp(options, indent+2)

def main():
    data = get_data()
    disp(data)
    # __import__("pprint").pprint(data)


if __name__ == "__main__":
    main()
